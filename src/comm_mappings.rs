pub mod community_mappings {
    use crate::mrt_communities::standard_communities::StandardCommunity;
    use bgpkit_parser::models::Asn;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum PeerType {
        Customer,
        Peer,
        Upstream,
        NoneFound,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum PeerLocation {
        Africa,
        AsiaPac,
        Europe,
        NorthAmerica,
        SouthAmerica,
        NoneFound,
    }

    pub struct CommMappings {
        peer_type: HashMap<StandardCommunity, PeerType>,
        peer_location: HashMap<StandardCommunity, PeerLocation>,
    }

    /// Map community values to peer type and peer location
    impl CommMappings {
        pub fn new(
            peer_type: HashMap<StandardCommunity, PeerType>,
            peer_location: HashMap<StandardCommunity, PeerLocation>,
        ) -> CommMappings {
            CommMappings {
                peer_type,
                peer_location,
            }
        }

        pub fn get_peer_type(&self, community: &StandardCommunity) -> Option<&PeerType> {
            self.peer_type.get(community)
        }

        pub fn get_peer_location(&self, community: &StandardCommunity) -> Option<&PeerLocation> {
            self.peer_location.get(community)
        }
    }

    /// Map ASNs to community sets
    pub struct AsnMappings {
        asn_mappings: HashMap<Asn, CommMappings>,
    }

    impl Default for AsnMappings {
        fn default() -> Self {
            let mut asn_mappings = HashMap::<Asn, CommMappings>::new();

            asn_mappings.insert(
                Asn::new_32bit(174),
                CommMappings::new(
                    HashMap::from([
                        (StandardCommunity::new(174, 21000), PeerType::Peer),
                        (StandardCommunity::new(174, 21001), PeerType::Customer),
                        (StandardCommunity::new(174, 21100), PeerType::Peer),
                        (StandardCommunity::new(174, 21101), PeerType::Customer),
                        (StandardCommunity::new(174, 21200), PeerType::Peer),
                        (StandardCommunity::new(174, 21201), PeerType::Customer),
                        (StandardCommunity::new(174, 21300), PeerType::Peer),
                        (StandardCommunity::new(174, 21301), PeerType::Customer),
                        (StandardCommunity::new(174, 21400), PeerType::Peer),
                        (StandardCommunity::new(174, 21401), PeerType::Customer),
                        (StandardCommunity::new(174, 21500), PeerType::Peer),
                        (StandardCommunity::new(174, 21501), PeerType::Customer),
                    ]),
                    HashMap::from([
                        (
                            StandardCommunity::new(174, 21000),
                            PeerLocation::NorthAmerica,
                        ),
                        (
                            StandardCommunity::new(174, 21001),
                            PeerLocation::NorthAmerica,
                        ),
                        (StandardCommunity::new(174, 21100), PeerLocation::Europe),
                        (StandardCommunity::new(174, 21101), PeerLocation::Europe),
                        (StandardCommunity::new(174, 21200), PeerLocation::AsiaPac),
                        (StandardCommunity::new(174, 21201), PeerLocation::AsiaPac),
                        (
                            StandardCommunity::new(174, 21300),
                            PeerLocation::SouthAmerica,
                        ),
                        (
                            StandardCommunity::new(174, 21301),
                            PeerLocation::SouthAmerica,
                        ),
                        (StandardCommunity::new(174, 21400), PeerLocation::AsiaPac),
                        (StandardCommunity::new(174, 21401), PeerLocation::AsiaPac),
                        (StandardCommunity::new(174, 21500), PeerLocation::Africa),
                        (StandardCommunity::new(174, 21501), PeerLocation::Africa),
                    ]),
                ),
            );

            asn_mappings.insert(
                Asn::new_32bit(701),
                CommMappings::new(
                    HashMap::from([
                        (StandardCommunity::new(0, 201), PeerType::Customer),
                        (StandardCommunity::new(0, 203), PeerType::Peer),
                    ]),
                    HashMap::new(),
                ),
            );

            asn_mappings.insert(
                Asn::new_32bit(1299),
                CommMappings::new(
                    HashMap::from([
                        (StandardCommunity::new(1299, 20000), PeerType::Peer),
                        (StandardCommunity::new(1299, 25000), PeerType::Peer),
                        (StandardCommunity::new(1299, 27000), PeerType::Peer),
                        (StandardCommunity::new(1299, 30000), PeerType::Customer),
                        (StandardCommunity::new(1299, 35000), PeerType::Customer),
                        (StandardCommunity::new(1299, 37000), PeerType::Customer),
                    ]),
                    HashMap::from([
                        (StandardCommunity::new(1299, 20000), PeerLocation::Europe),
                        (
                            StandardCommunity::new(1299, 25000),
                            PeerLocation::NorthAmerica,
                        ),
                        (StandardCommunity::new(1299, 27000), PeerLocation::AsiaPac),
                        (StandardCommunity::new(1299, 30000), PeerLocation::Europe),
                        (
                            StandardCommunity::new(1299, 35000),
                            PeerLocation::NorthAmerica,
                        ),
                        (StandardCommunity::new(1299, 37000), PeerLocation::AsiaPac),
                    ]),
                ),
            );

            asn_mappings.insert(
                Asn::new_32bit(2914),
                CommMappings::new(
                    HashMap::from([
                        (StandardCommunity::new(2914, 410), PeerType::Customer),
                        (StandardCommunity::new(2914, 420), PeerType::Peer),
                    ]),
                    HashMap::from([
                        (
                            StandardCommunity::new(2914, 3000),
                            PeerLocation::NorthAmerica,
                        ),
                        (
                            StandardCommunity::new(2914, 3075),
                            PeerLocation::NorthAmerica,
                        ),
                        (StandardCommunity::new(2914, 3200), PeerLocation::Europe),
                        (StandardCommunity::new(2914, 3275), PeerLocation::Europe),
                        (StandardCommunity::new(2914, 3400), PeerLocation::AsiaPac),
                        (StandardCommunity::new(2914, 3475), PeerLocation::AsiaPac),
                        (
                            StandardCommunity::new(2914, 3600),
                            PeerLocation::SouthAmerica,
                        ),
                        (
                            StandardCommunity::new(2914, 3675),
                            PeerLocation::SouthAmerica,
                        ),
                    ]),
                ),
            );

            Self::new(asn_mappings)
        }
    }

    impl AsnMappings {
        pub fn new(asn_mappings: HashMap<Asn, CommMappings>) -> AsnMappings {
            AsnMappings { asn_mappings }
        }

        pub fn get_asn_peer_location(
            &self,
            asn: &Asn,
            community: &StandardCommunity,
        ) -> Option<&PeerLocation> {
            if self.asn_mappings.contains_key(asn) {
                let community_mappings = self.asn_mappings.get(asn).unwrap();
                return community_mappings.get_peer_location(community);
            }
            None
        }

        pub fn get_asn_peer_type(
            &self,
            asn: &Asn,
            community: &StandardCommunity,
        ) -> Option<&PeerType> {
            if self.asn_mappings.contains_key(asn) {
                let community_mappings = self.asn_mappings.get(asn).unwrap();
                return community_mappings.get_peer_type(community);
            }
            None
        }
    }
}
