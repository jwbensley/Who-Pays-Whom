pub mod community_mappings {
    use crate::mrt_communities::standard_communities::StandardCommunity;
    use bgpkit_parser::models::Asn;
    use std::collections::HashMap;

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum PeerType {
        Customer,
        Peer,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum PeerLocation {
        Africa,
        AsiaPac,
        Europe,
        NorthAmerica,
        SouthAmerica,
    }

    struct CommMappings {
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

    /// Map ASNs to communitiy sets
    pub struct AsnMappings {
        asn_mappings: HashMap<Asn, CommMappings>,
    }

    impl AsnMappings {
        pub fn new() -> AsnMappings {
            let mut asn_mappings = HashMap::<Asn, CommMappings>::new();

            asn_mappings.insert(
                Asn::new_32bit(174),
                CommMappings::new(
                    HashMap::from([
                        (StandardCommunity::new(174, 21001), PeerType::Customer),
                        (StandardCommunity::new(174, 21101), PeerType::Customer),
                        (StandardCommunity::new(174, 21201), PeerType::Customer),
                        (StandardCommunity::new(174, 21301), PeerType::Customer),
                        (StandardCommunity::new(174, 21401), PeerType::Customer),
                        (StandardCommunity::new(174, 21501), PeerType::Customer),
                    ]),
                    HashMap::from([
                        (
                            StandardCommunity::new(174, 21001),
                            PeerLocation::NorthAmerica,
                        ),
                        (StandardCommunity::new(174, 21101), PeerLocation::Europe),
                        (StandardCommunity::new(174, 21201), PeerLocation::AsiaPac),
                        (
                            StandardCommunity::new(174, 21301),
                            PeerLocation::SouthAmerica,
                        ),
                        (StandardCommunity::new(174, 21401), PeerLocation::AsiaPac),
                        (StandardCommunity::new(174, 21501), PeerLocation::Africa),
                    ]),
                ),
            );

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
