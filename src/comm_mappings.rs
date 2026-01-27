pub mod community_mappings {
    use crate::comm_data::insert_comm_mapping;
    use crate::mrt_asn::asn::MrtAsn;
    use crate::mrt_communities::standard_communities::StandardCommunity;
    use crate::peer_attrs::peer_data::{PeerLocation, PeerType};
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq)]
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

        pub fn add_peer_location(
            &mut self,
            community: StandardCommunity,
            peer_location: PeerLocation,
        ) {
            self.peer_location.insert(community, peer_location);
        }

        pub fn add_peer_type(&mut self, community: StandardCommunity, peer_type: PeerType) {
            self.peer_type.insert(community, peer_type);
        }

        pub fn get_peer_type(&self, community: &StandardCommunity) -> Option<&PeerType> {
            self.peer_type.get(community)
        }

        pub fn get_peer_location(&self, community: &StandardCommunity) -> Option<&PeerLocation> {
            self.peer_location.get(community)
        }
    }

    /// Map ASNs to community sets
    #[derive(Debug, PartialEq, Eq)]
    pub struct AsnMappings {
        asn_mappings: HashMap<MrtAsn, CommMappings>,
    }

    impl Default for AsnMappings {
        fn default() -> Self {
            let mut asn_mappings = HashMap::<MrtAsn, CommMappings>::new();
            insert_comm_mapping(&mut asn_mappings);
            Self::new(asn_mappings)
        }
    }

    impl AsnMappings {
        pub fn new(asn_mappings: HashMap<MrtAsn, CommMappings>) -> AsnMappings {
            AsnMappings { asn_mappings }
        }

        pub fn get_asn_peer_location(
            &self,
            asn: &MrtAsn,
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
            asn: &MrtAsn,
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
