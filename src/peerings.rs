pub mod global_peerings {
    use crate::mrt_asn::asn::MrtAsn;
    use crate::mrt_route::route::{IpVersion, Route};
    use crate::peer_attrs::peer_data::{PeerLocation, PeerType};
    use log::info;
    use serde::Serialize;
    use serde_json;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::BufWriter;

    #[derive(Debug, Serialize)]
    pub struct PeeringsByVersion {
        peerings_by_ver: HashMap<IpVersion, Route>,
    }

    impl Default for PeeringsByVersion {
        fn default() -> Self {
            Self::new(HashMap::<IpVersion, Route>::new())
        }
    }

    impl PeeringsByVersion {
        pub fn new(peerings_by_ver: HashMap<IpVersion, Route>) -> Self {
            Self { peerings_by_ver }
        }

        pub fn from(route: Route) -> Self {
            Self::new(HashMap::from([(route.get_ip_version().clone(), route)]))
        }

        pub fn add_peering(&mut self, route: Route) {
            if self.has_peering(&route) {
                return;
            }
            self.peerings_by_ver
                .insert(route.get_ip_version().clone(), route);
        }

        pub fn has_peering(&self, route: &Route) -> bool {
            self.peerings_by_ver.contains_key(&route.get_ip_version())
        }
    }

    /// All peerings for a single location, keyed by peer type
    #[derive(Debug, Serialize)]
    pub struct PeeringsInLocation {
        peerings_in_loc: HashMap<PeerType, PeeringsByVersion>,
    }

    impl Default for PeeringsInLocation {
        fn default() -> Self {
            Self::new(HashMap::<PeerType, PeeringsByVersion>::new())
        }
    }

    impl PeeringsInLocation {
        pub fn new(peerings_in_loc: HashMap<PeerType, PeeringsByVersion>) -> Self {
            Self { peerings_in_loc }
        }

        pub fn from(route: Route) -> Self {
            Self::new(HashMap::from([(
                route.get_peer_type().clone(),
                PeeringsByVersion::from(route),
            )]))
        }

        pub fn add_peering(&mut self, route: Route) {
            if !self.has_peerings_with_type(&route) {
                self.peerings_in_loc.insert(
                    route.get_peer_type().clone(),
                    PeeringsByVersion::from(route),
                );
            } else {
                self.get_type_peerings_mut(&route).add_peering(route);
            }
        }

        fn get_type_peerings(&self, route: &Route) -> &PeeringsByVersion {
            self.peerings_in_loc.get(route.get_peer_type()).unwrap()
        }

        fn get_type_peerings_mut(&mut self, route: &Route) -> &mut PeeringsByVersion {
            self.peerings_in_loc.get_mut(route.get_peer_type()).unwrap()
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
    #[derive(Debug, Serialize)]
    pub struct LocationPeerings {
        location_peerings: HashMap<PeerLocation, PeeringsInLocation>,
    }

    impl Default for LocationPeerings {
        fn default() -> Self {
            Self::new(HashMap::<PeerLocation, PeeringsInLocation>::new())
        }
    }

    impl LocationPeerings {
        pub fn new(location_peerings: HashMap<PeerLocation, PeeringsInLocation>) -> Self {
            Self { location_peerings }
        }

        pub fn from(route: Route) -> Self {
            Self::new(HashMap::from([(
                route.get_peer_location().clone(),
                PeeringsInLocation::from(route),
            )]))
        }

        pub fn add_peering(&mut self, route: Route) {
            if !self.has_peerings_in_location(&route) {
                self.location_peerings.insert(
                    route.get_peer_location().clone(),
                    PeeringsInLocation::from(route),
                );
            } else {
                self.get_peerings_in_mut(&route).add_peering(route);
            }
        }

        fn get_peerings_in(&self, route: &Route) -> &PeeringsInLocation {
            self.location_peerings
                .get(route.get_peer_location())
                .unwrap()
        }

        fn get_peerings_in_mut(&mut self, route: &Route) -> &mut PeeringsInLocation {
            self.location_peerings
                .get_mut(route.get_peer_location())
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
    #[derive(Debug, Serialize)]
    pub struct AsnPeerings {
        peers: HashMap<MrtAsn, LocationPeerings>,
    }

    impl Default for AsnPeerings {
        fn default() -> Self {
            Self::new(HashMap::<MrtAsn, LocationPeerings>::new())
        }
    }

    impl AsnPeerings {
        pub fn new(peers: HashMap<MrtAsn, LocationPeerings>) -> Self {
            Self { peers }
        }

        pub fn from(route: Route) -> Self {
            Self::new(HashMap::from([(
                route.get_peer_as().to_owned(),
                LocationPeerings::from(route),
            )]))
        }

        pub fn add_peering(&mut self, route: Route) {
            if !self.has_peering_with(&route) {
                self.peers.insert(
                    route.get_peer_as().to_owned(),
                    LocationPeerings::from(route),
                );
            } else {
                self.get_peerings_for_mut(&route).add_peering(route);
            }
        }

        pub fn get_peerings_for(&self, route: &Route) -> &LocationPeerings {
            self.peers.get(route.get_peer_as()).unwrap()
        }

        pub fn get_peerings_for_mut(&mut self, route: &Route) -> &mut LocationPeerings {
            self.peers.get_mut(route.get_peer_as()).unwrap()
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
    #[derive(Debug, Serialize)]
    pub struct GlobalPeerings {
        global_peerings: HashMap<MrtAsn, AsnPeerings>,
    }

    impl Default for GlobalPeerings {
        fn default() -> Self {
            Self::new(HashMap::<MrtAsn, AsnPeerings>::new())
        }
    }

    impl GlobalPeerings {
        pub fn new(global_peerings: HashMap<MrtAsn, AsnPeerings>) -> Self {
            Self { global_peerings }
        }

        pub fn add_peering(&mut self, route: Route) {
            if !self.has_data_for(&route) {
                self.global_peerings
                    .insert(route.get_local_as().to_owned(), AsnPeerings::from(route));
            } else {
                self.get_asn_data_mut(&route).add_peering(route);
            }
        }

        fn get_asn_data(&self, route: &Route) -> &AsnPeerings {
            self.global_peerings.get(route.get_local_as()).unwrap()
        }

        fn get_asn_data_mut(&mut self, route: &Route) -> &mut AsnPeerings {
            self.global_peerings.get_mut(route.get_local_as()).unwrap()
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

        pub fn to_file(&self, filename: &String) {
            let writer = BufWriter::new(File::create(filename).unwrap());
            serde_json::to_writer_pretty(writer, self).unwrap();
            info!("Wrote JSON to {}", filename);
        }
    }
}
