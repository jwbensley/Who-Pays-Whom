pub mod triple_t1_paths {
    use crate::file::ensure_dir;
    use crate::{mrt_asn::asn::MrtAsn, mrt_route::route::Route};
    use itertools::Itertools;
    use log::info;
    use serde::{Serialize, Serializer, ser::SerializeMap};
    use std::{collections::HashMap, fs::File, io::BufWriter};

    #[derive(Debug)]
    pub struct TripleT1Paths {
        triple_t1_paths: HashMap<Vec<MrtAsn>, Route>,
    }

    impl Serialize for TripleT1Paths {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let mut map = serializer.serialize_map(Some(self.triple_t1_paths.len()))?;
            for (k, v) in self.triple_t1_paths.iter() {
                map.serialize_entry(&k.iter().join(","), &v)?;
            }
            map.end()
        }
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

        pub fn add_path(&mut self, triple_t1_path: Vec<MrtAsn>, route: Route) {
            self.triple_t1_paths.insert(triple_t1_path, route);
        }

        pub fn has_path(&self, triple_t1_path: &Vec<MrtAsn>) -> bool {
            self.triple_t1_paths.contains_key(triple_t1_path)
        }

        pub fn to_file(&self, filename: &String) {
            ensure_dir(filename);
            let writer = BufWriter::new(File::create(filename).unwrap());
            serde_json::to_writer_pretty(writer, &self).unwrap();
            info!("Wrote JSON to {}", filename);
        }
    }
}
