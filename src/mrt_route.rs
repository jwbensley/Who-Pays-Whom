pub mod route {
    use crate::mrt_asn::asn::MrtAsn;
    use crate::mrt_communities::standard_communities::StandardCommunities;
    use crate::mrt_peer::peer::Peer;
    use crate::peer_attrs::peer_data::{PeerLocation, PeerType};
    use ipnet::IpNet;
    use serde::ser::SerializeStruct as _;
    use serde::{Serialize, Serializer};
    use std::hash::Hash;
    use std::net::IpAddr;

    #[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize)]
    pub enum IpVersion {
        Ipv4,
        Ipv6,
    }

    /// Store a route pulled from an MRT file
    #[derive(Clone, Debug)]
    pub struct Route {
        local_as: MrtAsn,
        peer_as: MrtAsn,
        peer_type: PeerType,
        peer_location: PeerLocation,
        as_path: Vec<MrtAsn>,
        filename: String,
        next_hop: IpAddr,
        peer: Peer,
        prefix: IpNet,
        communities: StandardCommunities,
    }

    impl Serialize for Route {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut state = serializer.serialize_struct("Route", 11)?;
            state.serialize_field("local_as", &self.local_as)?;
            state.serialize_field("peer_as", &self.peer_as)?;
            state.serialize_field("peer_type", &self.peer_type)?;
            state.serialize_field("peer_location", &self.peer_location)?;
            state.serialize_field("as_path", &self.as_path)?;
            state.serialize_field("filename", &self.filename)?;
            state.serialize_field("next_hop", &self.next_hop)?;
            state.serialize_field("peer", &self.peer)?;
            state.serialize_field("prefix", &self.prefix.to_string())?;
            state.serialize_field("communities", &self.communities)?;
            state.end()
        }
    }

    impl Route {
        pub fn new(
            local_as: MrtAsn,
            peer_as: MrtAsn,
            peer_type: PeerType,
            peer_location: PeerLocation,
            as_path: Vec<MrtAsn>,
            filename: String,
            next_hop: IpAddr,
            peer: Peer,
            prefix: IpNet,
            communities: StandardCommunities,
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
                communities,
            }
        }

        pub fn get_ip_version(&self) -> IpVersion {
            if self.prefix.to_string().contains(".") {
                IpVersion::Ipv4
            } else {
                IpVersion::Ipv6
            }
        }

        pub fn get_local_as(&self) -> &MrtAsn {
            &self.local_as
        }

        pub fn get_peer_as(&self) -> &MrtAsn {
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
