pub mod standard_communities {
    use crate::comm_mappings::community_mappings::AsnMappings;
    use crate::mrt_asn::asn::{self, MrtAsn};
    use crate::mrt_peer::peer;
    use crate::peer_attrs::peer_data::{PeerLocation, PeerType};
    use bgpkit_parser::models::Community;
    use log::debug;
    use serde::ser::{SerializeSeq, SerializeTuple};
    use serde::{Serialize, Serializer};
    use std::hash::Hash;
    use std::vec::Vec;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct StandardCommunity {
        asn: MrtAsn,
        value: u16,
    }

    impl Serialize for StandardCommunity {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut tup = serializer.serialize_tuple(2)?;
            tup.serialize_element(&self.get_asn().clone().to_u32())?;
            tup.serialize_element(&self.get_value())?;
            tup.end()
        }
    }

    impl Hash for StandardCommunity {
        fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
            core::mem::discriminant(&self).hash(state);
        }
    }

    impl StandardCommunity {
        pub fn new(asn: u32, value: u16) -> Self {
            Self {
                asn: MrtAsn::from_u32(asn),
                value,
            }
        }

        pub fn from(community: Community) -> Self {
            if let Community::Custom(asn, value) = community {
                Self::new(asn.to_u32(), value)
            } else {
                panic!(
                    "Couldn't unpack Community into StandardCommunity: {}",
                    community
                );
            }
        }

        pub fn get_asn(&self) -> &MrtAsn {
            &self.asn
        }

        fn get_value(&self) -> &u16 {
            &self.value
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct StandardCommunities<'a> {
        standard_communities: Vec<StandardCommunity>,
        peer_mappings: &'a AsnMappings,
    }

    impl Serialize for StandardCommunities<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut seq = serializer.serialize_seq(Some(self.standard_communities.len()))?;
            for e in &self.standard_communities {
                seq.serialize_element(e)?;
            }
            seq.end()
        }
    }

    impl<'a> StandardCommunities<'a> {
        pub fn new(
            standard_communities: Vec<StandardCommunity>,
            peer_mappings: &'a AsnMappings,
        ) -> Self {
            Self {
                standard_communities,
                peer_mappings,
            }
        }

        pub fn add(&mut self, c: StandardCommunity) {
            self.standard_communities.push(c);
        }

        fn from_vec(communities: Vec<Community>, asn_mappings: &'a AsnMappings) -> Self {
            let mut standard_communities = Self::new(Vec::<StandardCommunity>::new(), asn_mappings);
            for community in communities {
                standard_communities.add(StandardCommunity::from(community));
            }
            standard_communities
        }

        pub fn get_peer_location(
            &'a self,
            local_asn: &MrtAsn,
            asn_mappings: &'a AsnMappings,
        ) -> &'a PeerLocation {
            for standard_community in &self.standard_communities {
                if standard_community.get_asn() == local_asn
                    && let Some(peer_location) =
                        asn_mappings.get_asn_peer_location(local_asn, standard_community)
                {
                    return peer_location;
                }
            }
            debug!(
                "Couldn't get peer location for ASN {:?} from: {:#?}",
                local_asn, self
            );
            &PeerLocation::NoneFound
        }

        pub fn get_peer_type(
            &'a self,
            local_asn: &MrtAsn,
            asn_mappings: &'a AsnMappings,
        ) -> &'a PeerType {
            for standard_community in &self.standard_communities {
                if standard_community.get_asn() == local_asn
                    && let Some(peer_type) =
                        asn_mappings.get_asn_peer_type(local_asn, standard_community)
                {
                    return peer_type;
                }
            }
            debug!(
                "Couldn't get peer type for ASN {:?} from: {:#?}",
                local_asn, self
            );
            &PeerType::NoneFound
        }
    }
}
