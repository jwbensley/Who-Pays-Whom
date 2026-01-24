pub mod route {
    use crate::mrt_communities::standard_communities::StandardCommunities;
    use crate::peer_attrs::peer_data::{PeerLocation, PeerType};
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
    }

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
            communities: StandardCommunities,
        ) -> Self {
            let ip_version = if prefix.to_string().contains(".") {
                IpVersion::Ipv4
            } else {
                IpVersion::Ipv6
            };

            Self {
                local_as,
                peer_as,
                peer_type,
                peer_location,
                as_path,
                filename,
                next_hop,
                peer,
                ip_version,
                prefix,
                communities,
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
    }
}
