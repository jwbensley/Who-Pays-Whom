pub mod rib_parser {
    use crate::comm_mappings::community_mappings::AsnMappings;
    use crate::mrt_asn::asn::Tier1Asn;
    use crate::mrt_communities::standard_communities::StandardCommunities;
    use crate::mrt_large_communities::large_communities::LargeCommunities;
    use crate::mrt_route::route::{IpVersion, Route};
    use crate::peerings::global_peerings::GlobalPeerings;
    use crate::ribs::rib_getter::RibFile;
    use bgpkit_parser::models::{
        AsPathSegment, Asn, AttrFlags, AttrType, Attribute, AttributeValue, MrtMessage, Peer,
        RibAfiEntries, RibEntry, TableDumpV2Message, TableDumpV2Type,
    };
    use bgpkit_parser::{BgpkitParser, MrtRecord};
    use ipnet::IpNet;
    use log::{debug, info};
    use rayon::ThreadPoolBuilder;
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use std::collections::HashMap;
    use std::net::IpAddr;

    /// Parse a list of RIB files in parallel
    pub fn find_peer_data(rib_files: &Vec<RibFile>, threads: &u32) -> GlobalPeerings {
        info!("Paring {} RIB files", rib_files.len());
        debug!(
            "{:?}",
            rib_files
                .iter()
                .map(|x| &x.filename)
                .collect::<Vec<&String>>()
        );

        let asn_mappings = AsnMappings::new();

        ThreadPoolBuilder::new()
            .num_threads((*threads).try_into().unwrap())
            .build_global()
            .unwrap();

        let global_peerings = GlobalPeerings::new();

        rib_files.into_par_iter().map(|rib_file| {
            parse_rib_file(rib_file.filename.clone(), &asn_mappings, &global_peerings)
        });

        global_peerings
    }

    fn parse_rib_file(fp: String, asn_mappings: &AsnMappings, global_peerings: &GlobalPeerings) {
        info!("Parsing {}", fp);

        let mut count: u32 = 0;
        let mut id_peer_map = HashMap::<u16, Peer>::new();

        let parser =
            BgpkitParser::new(fp.as_str()).unwrap_or_else(|_| panic!("Unable to parse {}", fp));

        for mrt_entry in parser.into_record_iter() {
            if count == 0 {
                id_peer_map = get_peer_id_map(&fp);
                debug!("Peer Map for {}: {:#?}\n", fp, id_peer_map);
                count += 1;
                continue;
            }

            parse_rib_entries(
                &mrt_entry,
                global_peerings,
                &id_peer_map,
                asn_mappings,
                &fp,
                &count,
            );

            count += 1;
        }

        info!("Parsed {} records in {}.", count, fp,);
    }

    /// Return the mapping of peer IDs to peer details
    fn get_peer_id_map(fp: &String) -> HashMap<u16, Peer> {
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

    fn parse_rib_entries(
        mrt_entry: &MrtRecord,
        global_peerings: &GlobalPeerings,
        id_peer_map: &HashMap<u16, Peer>,
        asn_mappings: &AsnMappings,
        fp: &String,
        count: &u32,
    ) {
        let rib_entries = get_rib_entries(mrt_entry, fp, count);
        if rib_entries.is_none() {
            return;
        }
        let rib_entries = rib_entries.unwrap_or_else(|| {
            panic!(
                "Unable to consume RIB entries from {}: {:#?}",
                fp, mrt_entry
            )
        });

        for rib_entry in &rib_entries.rib_entries {
            let prefix = rib_entries.prefix.prefix;
            let ip_version;
            if prefix.to_string().contains(".") {
                ip_version = IpVersion::Ipv4
            } else {
                ip_version = IpVersion::Ipv6
            }
            let next_hop = get_next_hop(rib_entry, fp, count);
            let communities = get_communities(rib_entry);
            let large_communities = get_large_communities(rib_entry);
            let mut as_sequence = get_as_sequence(rib_entry, fp, count);
            as_sequence.dedup();

            for local_asn in &as_sequence {
                if local_asn.is_t1() {
                    let peer_asn =
                        &as_sequence[as_sequence.iter().position(|x| x == local_asn).unwrap() + 1];

                    if peer_asn.is_t1() {
                        let peer_location = communities.get_peer_location(local_asn, asn_mappings);
                        let peer_type = communities.get_peer_type(local_asn, asn_mappings);
                        let route = Route::new(
                            local_asn.clone(),
                            peer_asn.clone(),
                            peer_type.clone(),
                            peer_location.clone(),
                            as_sequence.clone(),
                            fp.clone(),
                            next_hop,
                            id_peer_map[&rib_entry.peer_index],
                            prefix,
                            ip_version.clone(),
                            communities.clone(),
                            large_communities.clone(),
                        );
                    }
                }
            }
        }
    }

    /// Return the RIB entry in the MRT record.
    /// This is either a single v4 prefix or a single v6 prefix
    /// Skip default route.
    fn get_rib_entries<'a>(
        mrt_entry: &'a MrtRecord,
        fp: &String,
        count: &u32,
    ) -> Option<&'a RibAfiEntries> {
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
                    "Unexpected record type {:#?} in file {} ({})",
                    mrt_entry.message, fp, count
                ),
            }
        } else {
            panic!(
                "MRT record isn't of type RibAfi in file {} ({}): {:#?}",
                fp, count, mrt_entry
            );
        }
    }

    /// Return the next-nop which can be v4 or v6.
    /// If v6 LL and GUA nh exists, GUA is returned.
    fn get_next_hop(rib_entry: &RibEntry, fp: &String, count: &u32) -> IpAddr {
        if rib_entry.attributes.get_reachable_nlri().is_some() {
            let mp_nlri = rib_entry
                .attributes
                .get_reachable_nlri()
                .unwrap_or_else(|| {
                    panic!(
                        "Couldn't extract MP NLRI in file {} ({}) for: {:#?}",
                        fp, count, rib_entry
                    )
                });

            assert!(
                mp_nlri.is_ipv6(),
                "MP NLRI is used for non-IPv6 info in file {} ({}): {:#?}",
                fp,
                count,
                rib_entry
            );

            mp_nlri.next_hop_addr()
        } else {
            rib_entry.attributes.next_hop().unwrap_or_else(|| {
                panic!(
                    "No next-hop in file {} ({}) for: {:#?}",
                    fp, count, rib_entry
                )
            })
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

    fn get_large_communities(rib_entry: &RibEntry) -> LargeCommunities {
        if let AttributeValue::LargeCommunities(large_communities) = rib_entry
            .attributes
            .get_attr(AttrType::LARGE_COMMUNITIES)
            .unwrap_or(Attribute {
                value: AttributeValue::LargeCommunities(Vec::new()),
                flag: AttrFlags::OPTIONAL | AttrFlags::TRANSITIVE,
            })
            .value
        {
            LargeCommunities::new(large_communities)
        } else {
            LargeCommunities::new(Vec::new())
        }
    }

    /// Split the segments of the AS Path into an AS Sequence and an AS Set.
    /// The likelihood of there being more than on AS Sequnece (because the path)
    /// is longer than 255 ASNs is incredibly low. Also, because we're looking for
    /// T1 AS path, we're not interested in AS_SETs.
    fn get_as_sequence(rib_entry: &RibEntry, fp: &String, count: &u32) -> Vec<Asn> {
        let as_path_segments = &rib_entry
            .attributes
            .as_path()
            .unwrap_or_else(|| {
                panic!(
                    "Unable to unpack AS Path segments from RIB entry at {} in {}:  {:#?}",
                    count, fp, rib_entry
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
}
