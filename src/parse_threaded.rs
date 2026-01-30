pub mod threaded_parser {
    use crate::args::cli_args::CliArgs;
    use crate::comm_mappings::community_mappings::AsnMappings;
    use crate::parse_mrt::mrt_parser::{MrtData, get_peer_id_map, parse_mrt_entry};
    use crate::peerings::peering_data::PeeringData;
    use crate::ribs::rib_getter::RibFile;
    use crate::triple_paths::triple_t1_paths::TripleT1Paths;
    use bgpkit_parser::BgpkitParser;
    use log::{debug, info};
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use rayon::prelude::*;
    use std::sync::{Arc, RwLock};

    /// Setup parallel parsing of RIB files
    pub fn parse_rib_files(rib_files: &Vec<RibFile>, args: &CliArgs) {
        info!("Going to parse {} RIB files", rib_files.len());
        debug!(
            "{:?}",
            rib_files
                .iter()
                .map(|x| x.get_filename())
                .collect::<Vec<&String>>()
        );

        let asn_mappings = AsnMappings::default();
        let peering_data = Arc::new(RwLock::new(PeeringData::default()));
        let triple_t1_paths = Arc::new(RwLock::new(TripleT1Paths::default()));

        if rib_files.len() == 1 {
            parse_rib_file_threaded(
                rib_files[0].get_filename(),
                &asn_mappings,
                Arc::clone(&peering_data),
                Arc::clone(&triple_t1_paths),
            );
        } else {
            rib_files.into_par_iter().for_each(|rib_file| {
                parse_rib_file(
                    rib_file.get_filename(),
                    &asn_mappings,
                    Arc::clone(&peering_data),
                    Arc::clone(&triple_t1_paths),
                )
            });
        }

        debug! {"{:#?}", peering_data.read().unwrap()};
        peering_data.read().unwrap().to_file(&args.peering_data);

        debug! {"{:#?}", triple_t1_paths.read().unwrap()};
        triple_t1_paths
            .read()
            .unwrap()
            .to_file(&args.triple_t1_paths);
    }

    /// Per thread loop over a single file
    fn parse_rib_file(
        fp: &String,
        asn_mappings: &AsnMappings,
        peering_data: Arc<RwLock<PeeringData>>,
        triple_t1_paths: Arc<RwLock<TripleT1Paths>>,
    ) {
        info!("Parsing {}", fp);

        let peer_id_map = get_peer_id_map(fp);
        debug!("Peer Map for {}: {:#?}\n", fp, peer_id_map);

        let parser =
            BgpkitParser::new(fp.as_str()).unwrap_or_else(|_| panic!("Unable to parse {}", fp));

        parser.into_record_iter().skip(1).for_each(|mrt_entry| {
            parse_mrt_entry(MrtData::new(
                &mrt_entry,
                &Arc::clone(&peering_data),
                &Arc::clone(&triple_t1_paths),
                &peer_id_map,
                asn_mappings,
                fp,
            ))
        });

        info!("Parsed {}", fp,);
    }

    /// Parse a single file across multiple threads
    pub fn parse_rib_file_threaded(
        fp: &String,
        asn_mappings: &AsnMappings,
        peering_data: Arc<RwLock<PeeringData>>,
        triple_t1_paths: Arc<RwLock<TripleT1Paths>>,
    ) {
        info!("Parsing {}", fp);

        let peer_id_map = get_peer_id_map(fp);
        debug!("Peer Map for {}: {:#?}\n", fp, peer_id_map);

        let parser =
            BgpkitParser::new(fp.as_str()).unwrap_or_else(|_| panic!("Unable to parse {}", fp));

        parser
            .into_record_iter()
            .skip(1)
            .par_bridge()
            .for_each(|mrt_entry| {
                parse_mrt_entry(MrtData::new(
                    &mrt_entry,
                    &Arc::clone(&peering_data),
                    &Arc::clone(&triple_t1_paths),
                    &peer_id_map,
                    &asn_mappings,
                    fp,
                ))
            });
    }
}
