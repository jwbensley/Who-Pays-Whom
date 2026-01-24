pub mod global_peerings {
    use crate::mrt_route::route::{IpVersion, Route};
    use crate::peer_attrs::peer_data::{PeerLocation, PeerType};
    use bgpkit_parser::models::Asn;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub struct PeeringsByVersion {
        peerings_by_ver: HashMap<IpVersion, Route>,
    }

    impl Default for PeeringsByVersion {
        fn default() -> Self {
            Self::new(HashMap::<IpVersion, Route>::new())
        }
    }

    impl PeeringsByVersion {
        pub fn new(peerings_by_ver: HashMap<IpVersion, Route>) -> PeeringsByVersion {
            PeeringsByVersion { peerings_by_ver }
        }

        pub fn add_peering(&mut self, route: Route) {
            if self.has_peering(&route) {
                return;
            }
            self.peerings_by_ver
                .insert(route.get_ip_version().clone(), route);
        }

        // pub fn get_route_by_version(&self, route: &Route) -> &Route {
        //     self.peerings_by_ver.get(route.get_ip_version()).unwrap()
        // }

        pub fn has_peering(&self, route: &Route) -> bool {
            self.peerings_by_ver.contains_key(route.get_ip_version())
        }
    }

    /// All peerings for a single location, keyed by peer type
    #[derive(Debug)]
    pub struct PeeringsInLocation {
        peerings_in_loc: HashMap<PeerType, PeeringsByVersion>,
    }

    impl Default for PeeringsInLocation {
        fn default() -> Self {
            Self::new(HashMap::<PeerType, PeeringsByVersion>::new())
        }
    }

    impl PeeringsInLocation {
        pub fn new(peerings_in_loc: HashMap<PeerType, PeeringsByVersion>) -> PeeringsInLocation {
            PeeringsInLocation { peerings_in_loc }
        }

        pub fn add_peering(&mut self, route: Route) {
            if self.has_peering(&route) {
                return;
            }

            let peer_type = route.get_peer_type().clone();
            let mut peerings_by_ver = PeeringsByVersion::default();
            peerings_by_ver.add_peering(route);
            self.peerings_in_loc.insert(peer_type, peerings_by_ver);
        }

        fn get_type_peerings(&self, route: &Route) -> &PeeringsByVersion {
            self.peerings_in_loc.get(route.get_peer_type()).unwrap()
        }

        pub fn has_peerings_with_type(&self, route: &Route) -> bool {
            self.peerings_in_loc.contains_key(route.get_peer_type())
        }

        pub fn has_peering(&self, route: &Route) -> bool {
            if self.has_peerings_with_type(route) {
                let type_peerings = self.get_type_peerings(route);
                return type_peerings.has_peering(route);
            }
            false
        }
    }

    /// All peerings for a single peer ASN, keyed by peer location
    #[derive(Debug)]
    pub struct LocationPeerings {
        location_peerings: HashMap<PeerLocation, PeeringsInLocation>,
    }

    impl Default for LocationPeerings {
        fn default() -> Self {
            Self::new(HashMap::<PeerLocation, PeeringsInLocation>::new())
        }
    }

    impl LocationPeerings {
        pub fn new(
            location_peerings: HashMap<PeerLocation, PeeringsInLocation>,
        ) -> LocationPeerings {
            LocationPeerings { location_peerings }
        }

        pub fn add_peering(&mut self, route: Route) {
            if self.has_peering(&route) {
                return;
            }

            let peer_location = route.get_peer_location().clone();
            let mut peerings_in_loc = PeeringsInLocation::default();
            peerings_in_loc.add_peering(route);
            self.location_peerings
                .insert(peer_location, peerings_in_loc);
        }

        fn get_peerings_in(&self, route: &Route) -> &PeeringsInLocation {
            self.location_peerings
                .get(route.get_peer_location())
                .unwrap()
        }

        pub fn has_peerings_in_location(&self, route: &Route) -> bool {
            self.location_peerings
                .contains_key(route.get_peer_location())
        }

        pub fn has_peering(&self, route: &Route) -> bool {
            if self.has_peerings_in_location(route) {
                let location_peerings = self.get_peerings_in(route);
                return location_peerings.has_peering(route);
            }
            false
        }
    }

    /// Peering data for an ASN, keyed by peer ASN
    #[derive(Debug)]
    pub struct AsnPeerings {
        peers: HashMap<Asn, LocationPeerings>,
    }

    impl Default for AsnPeerings {
        fn default() -> Self {
            Self::new(HashMap::<Asn, LocationPeerings>::new())
        }
    }

    impl AsnPeerings {
        pub fn new(peers: HashMap<Asn, LocationPeerings>) -> AsnPeerings {
            AsnPeerings { peers }
        }

        pub fn add_peering(&mut self, route: Route) {
            if self.has_peering(&route) {
                return;
            }

            let peer_asn = route.get_peer_as().to_owned();
            let mut location_peering = LocationPeerings::default();
            location_peering.add_peering(route);
            self.peers.insert(peer_asn, location_peering);
        }

        pub fn get_peerings_for(&self, route: &Route) -> &LocationPeerings {
            self.peers.get(route.get_peer_as()).unwrap()
        }

        pub fn has_peering_with(&self, route: &Route) -> bool {
            self.peers.contains_key(route.get_peer_as())
        }

        pub fn has_peering(&self, route: &Route) -> bool {
            if self.has_peering_with(route) {
                let location_peerings = self.get_peerings_for(route);
                return location_peerings.has_peering(route);
            }
            false
        }
    }

    /// Public API which provides access to all peerings, keyed by local ASN
    #[derive(Debug)]
    pub struct GlobalPeerings {
        global_peerings: HashMap<Asn, AsnPeerings>,
    }

    impl Default for GlobalPeerings {
        fn default() -> Self {
            Self::new(HashMap::<Asn, AsnPeerings>::new())
        }
    }

    impl GlobalPeerings {
        pub fn new(global_peerings: HashMap<Asn, AsnPeerings>) -> Self {
            GlobalPeerings { global_peerings }
        }

        pub fn add_peering(&mut self, route: Route) {
            if self.has_peering(&route) {
                return;
            }

            let local_asn = route.get_local_as().to_owned();
            let mut asn_peerings = AsnPeerings::default();
            asn_peerings.add_peering(route);
            self.global_peerings.insert(local_asn, asn_peerings);
        }

        fn get_asn_data(&self, route: &Route) -> &AsnPeerings {
            self.global_peerings.get(route.get_local_as()).unwrap()
        }

        fn has_data_for(&self, route: &Route) -> bool {
            self.global_peerings.contains_key(route.get_local_as())
        }

        pub fn has_peering(&self, route: &Route) -> bool {
            if self.has_data_for(route) {
                let asn_data = self.get_asn_data(route);
                return asn_data.has_peering(route);
            }
            false
        }
    }
}
