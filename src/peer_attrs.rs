pub mod peer_data {
    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum PeerType {
        Customer,
        Peer,
        PaidPeer,
        Upstream,
        NoneFound,
    }

    #[derive(Clone, Debug, Eq, Hash, PartialEq)]
    pub enum PeerLocation {
        Africa,
        AsiaPac,
        EuropeMiddleEast,
        NorthAmerica,
        SouthAmerica,
        NoneFound,
    }
}
