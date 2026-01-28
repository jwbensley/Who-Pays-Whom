pub mod triple_t1_paths {
    use crate::{mrt_asn::asn::MrtAsn, mrt_route::route::Route};
    use std::collections::HashMap;

    pub struct TripleT1Paths {
        triple_t1_paths: HashMap<Vec<MrtAsn>, Route>,
    }

    impl Default for TripleT1Paths {
        fn default() -> Self {
            Self::new(HashMap::<Vec<MrtAsn>, Route>::new())
        }
    }

    impl TripleT1Paths {
        pub fn new(triple_t1_paths: HashMap<Vec<MrtAsn>, Route>) -> Self {
            Self { triple_t1_paths }
        }

        pub fn has_path(&self, triple_t1_path: &Vec<MrtAsn>) -> bool {
            self.triple_t1_paths.contains_key(triple_t1_path)
        }

        pub fn add_path(&mut self, triple_t1_path: Vec<MrtAsn>, route: Route) {
            self.triple_t1_paths.insert(triple_t1_path, route);
        }
    }
}
