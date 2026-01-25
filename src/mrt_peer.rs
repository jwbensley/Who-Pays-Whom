pub mod peer {
    use bgpkit_parser::models::Peer as BgpKit_Peer;
    use serde::{Serialize, Serializer, ser::SerializeStruct};
    use std::collections::HashMap;

    #[derive(Clone, Debug)]
    pub struct Peer(BgpKit_Peer);

    impl Serialize for Peer {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("Peer", 4)?;
            state.serialize_field("peer_bgp_id", &self.0.peer_bgp_id)?;
            state.serialize_field("peer_ip", &self.0.peer_ip)?;
            state.serialize_field("peer_asn", &self.0.peer_asn.to_u32())?;
            state.end()
        }
    }

    impl Peer {
        pub fn new(peer: BgpKit_Peer) -> Self {
            Self(peer)
        }
    }

    #[derive(Debug)]
    pub struct PeerTable {
        peer_table: HashMap<u16, Peer>,
    }

    impl PeerTable {
        pub fn new(peer_table: HashMap<u16, Peer>) -> Self {
            Self { peer_table }
        }

        pub fn from(peer_table: &HashMap<u16, BgpKit_Peer>) -> Self {
            let mut pt = HashMap::<u16, Peer>::new();
            for key in peer_table.keys() {
                pt.insert(key.clone(), Peer::new(peer_table.get(key).unwrap().clone()));
            }
            Self::new(pt)
        }

        pub fn get_peer(&self, id: &u16) -> &Peer {
            &self.peer_table.get(id).unwrap()
        }
    }
}
