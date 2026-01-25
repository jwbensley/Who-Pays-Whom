pub mod peer_data {
    use serde::Serialize;

    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
    pub enum PeerType {
        Customer,
        Peer,
        PaidPeer,
        Upstream,
        NoneFound,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize)]
    pub enum PeerLocation {
        Africa,
        AsiaPac,
        Europe,
        MiddleEast,
        NorthAmerica,
        SouthAmerica,
        NoneFound,
    }
}
