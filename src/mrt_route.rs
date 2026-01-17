pub mod route {
    use crate::mrt_communities::communities::Communities;
    use crate::mrt_large_communities::large_communities::LargeCommunities;
    use bgpkit_parser::models::{Asn, LargeCommunity, Peer};
    use ipnet::IpNet;
    use std::hash::Hash;
    use std::net::IpAddr;

    /// Store a route pulled from an MRT file (one route object per prefix)
    #[derive(Clone, Debug)]
    pub struct Route {
        as_path: Vec<Asn>,
        filename: String,
        next_hop: IpAddr,
        peer: Peer,
        prefix: IpNet,
        communities: Communities,
        large_communities: LargeCommunities,
    }

    // impl PartialEq for Route {
    //     fn eq(&self, other: &Self) -> bool {
    //         (self.as_path == other.as_path)
    //             // && (self.filename == other.filename)
    //             && (self.next_hop == other.next_hop)
    //             && (self.peer == other.peer)
    //             && (self.prefix == other.prefix)
    //             && (self.communities == other.communities)
    //             && (self.large_communities == other.large_communities)
    //     }
    // }

    // impl Hash for Route {
    //     fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    //         self.as_path.hash(state);
    //         // self.filename.hash(state);
    //         self.next_hop.hash(state);
    //         self.peer.hash(state);
    //         self.prefix.hash(state);
    //     }
    // }

    impl Route {
        pub fn new(
            as_path: Vec<Asn>,
            filename: String,
            next_hop: IpAddr,
            peer: Peer,
            prefix: IpNet,
            communities: Communities,
            large_communities: LargeCommunities,
        ) -> Self {
            Self {
                as_path,
                filename,
                next_hop,
                peer,
                prefix,
                communities,
                large_communities,
            }
        }

        // pub fn get_communities(&self) -> &Vec<Community> {
        //     &self.communities
        // }

        // pub fn get_large_communities(&self) -> &Vec<LargeCommunity> {
        //     &self.large_communities
        // }
    }
}
