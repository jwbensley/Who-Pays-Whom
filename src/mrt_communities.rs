pub mod standard_communities {
    use crate::comm_mappings::community_mappings::{PeerLocation, PeerType};
    use bgpkit_parser::models::Asn;
    use bgpkit_parser::models::Community;
    use std::hash::Hash;
    use std::vec::Vec;

    use crate::comm_mappings::community_mappings::AsnMappings;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct StandardCommunity {
        community: Community,
    }

    impl Hash for StandardCommunity {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            core::mem::discriminant(self).hash(state);
        }
    }

    impl StandardCommunity {
        pub fn new(asn: u32, value: u16) -> StandardCommunity {
            StandardCommunity {
                community: Community::Custom(Asn::new_32bit(asn), value),
            }
        }

        pub fn from(c: Community) -> StandardCommunity {
            StandardCommunity { community: c }
        }

        pub fn get_asn(&self) -> Asn {
            if let Community::Custom(asn, _) = self.community {
                return asn;
            } else {
                panic!("Couldn't unpack community valies from: {}", self.community);
            }
        }

        // pub fn get_values(&self) -> (Asn, u16) {
        //     if let Community::Custom(asn, value) = self.community {
        //         return (asn, value);
        //     } else {
        //         panic!("Couldn't unpack community valies from: {}", self.community);
        //     }
        // }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct StandardCommunities {
        communities: Vec<StandardCommunity>,
    }

    impl StandardCommunities {
        pub fn new() -> Self {
            StandardCommunities {
                communities: Vec::<StandardCommunity>::new(),
            }
        }

        pub fn add(&mut self, c: StandardCommunity) {
            self.communities.push(c);
        }

        pub fn get_peer_location<'a>(
            &'a self,
            local_asn: &Asn,
            asn_mappings: &'a AsnMappings,
        ) -> &PeerLocation {
            for standard_community in &self.communities {
                let community_asn = standard_community.get_asn();
                if &community_asn == local_asn {
                    return asn_mappings
                        .get_asn_peer_location(local_asn, standard_community)
                        .unwrap();
                }
            }
            panic!(
                "Couldn't get peer location for ASN {} from: {:#?}",
                local_asn, self
            )
        }

        pub fn get_peer_type<'a>(
            &'a self,
            local_asn: &Asn,
            asn_mappings: &'a AsnMappings,
        ) -> &PeerType {
            for standard_community in &self.communities {
                let community_asn = standard_community.get_asn();
                if &community_asn == local_asn {
                    return asn_mappings
                        .get_asn_peer_type(local_asn, standard_community)
                        .unwrap();
                }
            }
            panic!(
                "Couldn't get peer type for ASN {} from: {:#?}",
                local_asn, self
            )
        }
    }

    impl FromIterator<Community> for StandardCommunities {
        fn from_iter<T: IntoIterator<Item = Community>>(iter: T) -> Self {
            let mut sc = StandardCommunities::new();
            for i in iter {
                sc.add(StandardCommunity::from(i));
            }
            sc
        }
    }
}
