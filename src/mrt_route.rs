pub mod route {
    use crate::comm_mappings::community_mappings::{PeerLocation, PeerType};
    use crate::mrt_communities::standard_communities::StandardCommunities;
    use crate::mrt_large_communities::large_communities::LargeCommunities;
    use bgpkit_parser::models::{Asn, Peer};
    use ipnet::IpNet;
    use std::hash::Hash;
    use std::net::IpAddr;

    #[derive(Debug, Clone, Eq, Hash, PartialEq)]
    pub enum IpVersion {
        Ipv4,
        Ipv6,
    }

    /// Store a route pulled from an MRT file
    #[derive(Clone, Debug, PartialEq)]
    pub struct Route {
        local_as: Asn,
        peer_as: Asn,
        peer_type: PeerType,
        peer_location: PeerLocation,
        as_path: Vec<Asn>,
        filename: String,
        next_hop: IpAddr,
        peer: Peer,
        prefix: IpNet,
        ip_version: IpVersion,
        communities: StandardCommunities,
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
            local_as: Asn,
            peer_as: Asn,
            peer_type: PeerType,
            peer_location: PeerLocation,
            as_path: Vec<Asn>,
            filename: String,
            next_hop: IpAddr,
            peer: Peer,
            prefix: IpNet,
            ip_version: IpVersion,
            communities: StandardCommunities,
            large_communities: LargeCommunities,
        ) -> Self {
            Self {
                local_as,
                peer_as,
                peer_type,
                peer_location,
                as_path,
                filename,
                next_hop,
                peer,
                prefix,
                ip_version,
                communities,
                large_communities,
            }
        }

        pub fn get_ip_version(&self) -> &IpVersion {
            &self.ip_version
        }

        pub fn get_local_as(&self) -> &Asn {
            &self.local_as
        }

        pub fn get_peer_as(&self) -> &Asn {
            &self.peer_as
        }

        pub fn get_peer_location(&self) -> &PeerLocation {
            &self.peer_location
        }

        pub fn get_peer_type(&self) -> &PeerType {
            &self.peer_type
        }

        // pub fn get_communities(&self) -> &Vec<Community> {
        //     &self.communities
        // }

        // pub fn get_large_communities(&self) -> &Vec<LargeCommunity> {
        //     &self.large_communities
        // }
    }
}
