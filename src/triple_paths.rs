pub mod triple_t1_paths {
    use log::info;

    use crate::{mrt_asn::asn::MrtAsn, mrt_route::route::Route};
    use std::{collections::HashMap, fs::File, io::BufWriter};

    #[derive(serde::Serialize, Debug)]
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

        pub fn to_file(&self, filename: &String) {
            let writer = BufWriter::new(File::create(filename).unwrap());
            serde_json::to_writer_pretty(writer, self).unwrap();
            info!("Wrote JSON to {}", filename);
        }
    }
}
