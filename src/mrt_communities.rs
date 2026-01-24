pub mod standard_communities {
    use crate::comm_mappings::community_mappings::AsnMappings;
    use crate::peer_attrs::peer_data::{PeerLocation, PeerType};
    use bgpkit_parser::models::Asn;
    use bgpkit_parser::models::Community;
    use log::debug;
    use std::hash::Hash;
    use std::vec::Vec;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct StandardCommunity {
        standard_community: Community,
    }

    impl Hash for StandardCommunity {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            core::mem::discriminant(&self.standard_community).hash(state);
        }
    }

    impl StandardCommunity {
        pub fn new(asn: u32, value: u16) -> Self {
            StandardCommunity {
                standard_community: Community::Custom(Asn::new_32bit(asn), value),
            }
        }

        pub fn from(standard_community: Community) -> Self {
            StandardCommunity { standard_community }
        }

        pub fn get_asn(&self) -> Option<&Asn> {
            if let Community::Custom(asn, _) = &self.standard_community {
                Some(asn)
            } else {
                debug!(
                    "Couldn't unpack ASN from standard community: {}",
                    self.standard_community
                );
                None
            }
        }

        // pub fn get_values(&self) -> (&Asn, &u16) {
        //     if let Community::Custom(asn, value) = &self.standard_community {
        //         return (asn, value);
        //     } else {
        //         panic!("Couldn't unpack value from standard community: {}", self.standard_community);
        //     }
        // }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct StandardCommunities {
        standard_communities: Vec<StandardCommunity>,
    }

    impl Default for StandardCommunities {
        fn default() -> Self {
            Self::new(Vec::<StandardCommunity>::new())
        }
    }

    impl StandardCommunities {
        pub fn new(standard_communities: Vec<StandardCommunity>) -> Self {
            StandardCommunities {
                standard_communities,
            }
        }

        pub fn add(&mut self, c: StandardCommunity) {
            self.standard_communities.push(c);
        }

        pub fn get_peer_location<'a>(
            &'a self,
            local_asn: &Asn,
            asn_mappings: &'a AsnMappings,
        ) -> &'a PeerLocation {
            for standard_community in &self.standard_communities {
                if let Some(community_asn) = standard_community.get_asn()
                    && community_asn == local_asn
                    && let Some(peer_location) =
                        asn_mappings.get_asn_peer_location(local_asn, standard_community)
                {
                    return peer_location;
                }
            }
            debug!(
                "Couldn't get peer location for ASN {} from: {:#?}",
                local_asn, self
            );
            &PeerLocation::NoneFound
        }

        pub fn get_peer_type<'a>(
            &'a self,
            local_asn: &Asn,
            asn_mappings: &'a AsnMappings,
        ) -> &'a PeerType {
            for standard_community in &self.standard_communities {
                if let Some(community_asn) = standard_community.get_asn()
                    && community_asn == local_asn
                    && let Some(peer_type) =
                        asn_mappings.get_asn_peer_type(local_asn, standard_community)
                {
                    return peer_type;
                }
            }
            debug!(
                "Couldn't get peer type for ASN {} from: {:#?}",
                local_asn, self
            );
            &PeerType::NoneFound
        }
    }

    impl FromIterator<Community> for StandardCommunities {
        fn from_iter<T: IntoIterator<Item = Community>>(iter: T) -> Self {
            let mut standard_communities = StandardCommunities::default();
            for item in iter {
                standard_communities.add(StandardCommunity::from(item));
            }
            standard_communities
        }
    }
}
