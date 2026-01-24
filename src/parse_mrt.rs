pub mod mrt_parser {
    use crate::comm_mappings::community_mappings::AsnMappings;
    use crate::mrt_asn::asn::Tier1Asn;
    use crate::mrt_communities::standard_communities::StandardCommunities;
    use crate::mrt_route::route::Route;
    use crate::peer_attrs::peer_data::{PeerLocation, PeerType};
    use crate::peerings::global_peerings::GlobalPeerings;
    use bgpkit_parser::models::{
        AsPathSegment, Asn, AttrFlags, AttrType, Attribute, AttributeValue, MrtMessage, Peer,
        RibAfiEntries, RibEntry, TableDumpV2Message, TableDumpV2Type,
    };
    use bgpkit_parser::{BgpkitParser, MrtRecord};
    use ipnet::IpNet;
    use std::collections::HashMap;
    use std::net::IpAddr;
    use std::sync::{Arc, RwLock};

    pub fn parse_mrt_entry(
        mrt_entry: &MrtRecord,
        global_peerings: &Arc<RwLock<GlobalPeerings>>,
        id_peer_map: &HashMap<u16, Peer>,
        asn_mappings: &AsnMappings,
        fp: &String,
    ) {
        let rib_entries = get_rib_entries(mrt_entry, fp);
        if rib_entries.is_none() {
            return;
        }
        let rib_entries = rib_entries.unwrap_or_else(|| {
            panic!(
                "Unable to consume RIB entries from {}: {:#?}",
                fp, mrt_entry
            )
        });

        let prefix = rib_entries.prefix.prefix;

        for rib_entry in &rib_entries.rib_entries {
            let mut as_sequence = get_as_sequence(rib_entry, fp);
            as_sequence.dedup();

            for asn_1 in as_sequence.iter() {
                // We could see up to three T1 ASNs in a row e.g. AS3 AS2 AS1 AS65535.
                // AS3 peers with AS2, AS1 is transit customer of AS2 (despite being "Tier 1").
                // 65535 is transit customer of AS1.
                // In this case we need to check AS3-AS2 communities and AS2-AS1 communities.
                if asn_1.is_t1() {
                    let pos_1 = as_sequence.iter().position(|x| x == asn_1).unwrap();
                    if pos_1 == as_sequence.len() - 1 {
                        // Last ASN in the path
                        break;
                    }

                    let pos_2 = pos_1 + 1;
                    let asn_2 = &as_sequence[pos_2];
                    if asn_2.is_t1() {
                        check_peering(
                            rib_entry,
                            fp,
                            asn_mappings,
                            asn_1,
                            asn_2,
                            &as_sequence,
                            &id_peer_map[&rib_entry.peer_index],
                            &prefix,
                            global_peerings,
                        );

                        if pos_2 == as_sequence.len() - 1 {
                            // Last ASN in the path
                            break;
                        }

                        let pos_3 = pos_2 + 1;
                        let asn_3 = &as_sequence[pos_3];
                        if asn_3.is_t1() {
                            check_peering(
                                rib_entry,
                                fp,
                                asn_mappings,
                                asn_2,
                                asn_3,
                                &as_sequence,
                                &id_peer_map[&rib_entry.peer_index],
                                &prefix,
                                global_peerings,
                            );
                        }
                    }
                }
            }
        }
    }

    /// Return the mapping of peer IDs to peer details
    pub fn get_peer_id_map(fp: &String) -> HashMap<u16, Peer> {
        let parser = BgpkitParser::new(fp.as_str())
            .unwrap_or_else(|_| panic!("Unable to parse MRT file {}", fp));

        let mrt_record = parser.into_record_iter().next().unwrap();

        if let MrtMessage::TableDumpV2Message(TableDumpV2Message::PeerIndexTable(peer_table)) =
            &mrt_record.message
        {
            peer_table.id_peer_map.clone()
        } else {
            panic!("Couldn't extract peer table from table dump in {}", fp);
        }
    }

    /// Return the RIB entry in the MRT record.
    /// This is either a single v4 prefix or a single v6 prefix
    /// Skip default route.
    fn get_rib_entries<'a>(mrt_entry: &'a MrtRecord, fp: &String) -> Option<&'a RibAfiEntries> {
        let v4_default: IpNet = "0.0.0.0/0".parse().unwrap();
        let v6_default: IpNet = "::/0".parse().unwrap();

        if let MrtMessage::TableDumpV2Message(TableDumpV2Message::RibAfi(rib_entries)) =
            &mrt_entry.message
        {
            match rib_entries.rib_type {
                TableDumpV2Type::RibIpv4Unicast | TableDumpV2Type::RibIpv4UnicastAddPath => {
                    if rib_entries.prefix.prefix == v4_default {
                        return None;
                    }
                    Some(rib_entries)
                }
                TableDumpV2Type::RibIpv6Unicast | TableDumpV2Type::RibIpv6UnicastAddPath => {
                    if rib_entries.prefix.prefix == v6_default {
                        return None;
                    }
                    Some(rib_entries)
                }
                _ => panic!(
                    "Unexpected record type {:#?} in file {}",
                    mrt_entry.message, fp
                ),
            }
        } else {
            panic!(
                "MRT record isn't of type RibAfi in file {}: {:#?}",
                fp, mrt_entry
            );
        }
    }

    /// Return the next-nop which can be v4 or v6.
    /// If v6 LL and GUA nh exists, GUA is returned.
    fn get_next_hop(rib_entry: &RibEntry, fp: &String) -> IpAddr {
        if rib_entry.attributes.get_reachable_nlri().is_some() {
            let mp_nlri = rib_entry
                .attributes
                .get_reachable_nlri()
                .unwrap_or_else(|| {
                    panic!(
                        "Couldn't extract MP NLRI in file {} for: {:#?}",
                        fp, rib_entry
                    )
                });

            assert!(
                mp_nlri.is_ipv6(),
                "MP NLRI is used for non-IPv6 info in file {} for: {:#?}",
                fp,
                rib_entry
            );

            mp_nlri.next_hop_addr()
        } else {
            rib_entry
                .attributes
                .next_hop()
                .unwrap_or_else(|| panic!("No next-hop in file {} for: {:#?}", fp, rib_entry))
        }
    }

    fn get_communities(rib_entry: &RibEntry) -> StandardCommunities {
        if let AttributeValue::Communities(communities) = rib_entry
            .attributes
            .get_attr(AttrType::COMMUNITIES)
            .unwrap_or(Attribute {
                value: AttributeValue::Communities(Vec::new()),
                flag: AttrFlags::OPTIONAL | AttrFlags::TRANSITIVE,
            })
            .value
        {
            StandardCommunities::from_iter(communities)
        } else {
            StandardCommunities::from_iter(Vec::new())
        }
    }

    // fn get_large_communities(rib_entry: &RibEntry) -> LargeCommunities {
    //     if let AttributeValue::LargeCommunities(large_communities) = rib_entry
    //         .attributes
    //         .get_attr(AttrType::LARGE_COMMUNITIES)
    //         .unwrap_or(Attribute {
    //             value: AttributeValue::LargeCommunities(Vec::new()),
    //             flag: AttrFlags::OPTIONAL | AttrFlags::TRANSITIVE,
    //         })
    //         .value
    //     {
    //         LargeCommunities::new(large_communities)
    //     } else {
    //         LargeCommunities::default()
    //     }
    // }

    /// Split the segments of the AS Path into an AS Sequence and an AS Set.
    /// The likelihood of there being more than on AS Sequence (because the path)
    /// is longer than 255 ASNs is incredibly low. Also, because we're looking for
    /// T1 AS path, we're not interested in AS_SETs.
    fn get_as_sequence(rib_entry: &RibEntry, fp: &String) -> Vec<Asn> {
        let as_path_segments = &rib_entry
            .attributes
            .as_path()
            .unwrap_or_else(|| {
                panic!(
                    "Unable to unpack AS Path segments from RIB entry in {}:  {:#?}",
                    fp, rib_entry
                )
            })
            .segments;

        for path_seg in as_path_segments {
            if let AsPathSegment::AsSequence(asns) = path_seg {
                return asns.clone();
            }
        }

        Vec::<Asn>::new()
    }

    fn build_route(
        rib_entry: &RibEntry,
        fp: &String,
        asn_mappings: &AsnMappings,
        local_asn: &Asn,
        peer_asn: &Asn,
        as_sequence: &Vec<Asn>,
        peer: &Peer,
        prefix: &IpNet,
    ) -> Route {
        let next_hop = get_next_hop(rib_entry, fp);
        let communities = get_communities(rib_entry);
        let peer_location = communities.get_peer_location(local_asn, asn_mappings);
        let peer_type = communities.get_peer_type(local_asn, asn_mappings);

        Route::new(
            *local_asn,
            *peer_asn,
            peer_type.clone(),
            peer_location.clone(),
            as_sequence.clone(),
            fp.clone(),
            next_hop.to_owned(),
            peer.to_owned(),
            prefix.to_owned(),
            communities.clone(),
        )
    }

    fn check_peering(
        rib_entry: &RibEntry,
        fp: &String,
        asn_mappings: &AsnMappings,
        local_asn: &Asn,
        peer_asn: &Asn,
        as_sequence: &Vec<Asn>,
        peer: &Peer,
        prefix: &IpNet,
        global_peerings: &Arc<RwLock<GlobalPeerings>>,
    ) {
        let route = build_route(
            rib_entry,
            fp,
            asn_mappings,
            local_asn,
            peer_asn,
            as_sequence,
            peer,
            prefix,
        );

        if *route.get_peer_type() != PeerType::NoneFound
            && *route.get_peer_location() != PeerLocation::NoneFound
        {
            let has_peering: bool;
            {
                let gb_lock = global_peerings.read().unwrap();
                has_peering = gb_lock.has_peering(&route);
            }
            if !has_peering {
                let mut gb_lock = global_peerings.write().unwrap();
                gb_lock.add_peering(route);
            }
        }
    }
}
