pub mod threaded_parser {
    use crate::comm_mappings::community_mappings::AsnMappings;
    use crate::parse_mrt::mrt_parser::{get_peer_id_map, parse_mrt_entry};
    use crate::peerings::global_peerings::GlobalPeerings;
    use crate::ribs::rib_getter::RibFile;
    use bgpkit_parser::BgpkitParser;
    use log::{debug, info};
    use rayon::iter::{IntoParallelIterator, ParallelIterator};
    use rayon::prelude::*;
    use std::sync::{Arc, RwLock};

    /// Parse a list of RIB files in parallel
    pub fn parse_rib_files(rib_files: &Vec<RibFile>) {
        info!("Going to parse {} RIB files", rib_files.len());
        debug!(
            "{:?}",
            rib_files
                .iter()
                .map(|x| &x.filename)
                .collect::<Vec<&String>>()
        );

        let asn_mappings = AsnMappings::default();
        let global_peerings = Arc::new(RwLock::new(GlobalPeerings::default()));

        rib_files.into_par_iter().for_each(|rib_file| {
            parse_rib_file(
                &rib_file.filename,
                &asn_mappings,
                Arc::clone(&global_peerings),
            )
        });

        info! {"{:#?}", global_peerings.read().unwrap()};
    }

    /// Per thread loop over a single file
    fn parse_rib_file(
        fp: &String,
        asn_mappings: &AsnMappings,
        global_peerings: Arc<RwLock<GlobalPeerings>>,
    ) {
        info!("Parsing {}", fp);

        let peer_id_map = get_peer_id_map(fp);
        debug!("Peer Map for {}: {:#?}\n", fp, peer_id_map);

        let parser =
            BgpkitParser::new(fp.as_str()).unwrap_or_else(|_| panic!("Unable to parse {}", fp));

        parser.into_record_iter().skip(1).for_each(|mrt_entry| {
            parse_mrt_entry(&mrt_entry, &global_peerings, &peer_id_map, asn_mappings, fp)
        });

        info!("Parsed {}", fp,);
    }

    /// Parse a single file across multiple threads
    pub fn parse_rib_file_threaded(fp: &String) {
        info!("Parsing sinlge file {}", fp);

        let asn_mappings = AsnMappings::default();
        let global_peerings = Arc::new(RwLock::new(GlobalPeerings::default()));
        let peer_id_map = get_peer_id_map(fp);
        debug!("Peer Map for {}: {:#?}\n", fp, peer_id_map);

        let parser =
            BgpkitParser::new(fp.as_str()).unwrap_or_else(|_| panic!("Unable to parse {}", fp));

        parser
            .into_record_iter()
            .skip(1)
            .par_bridge()
            .for_each(|mrt_entry| {
                parse_mrt_entry(
                    &mrt_entry,
                    &Arc::clone(&global_peerings),
                    &peer_id_map,
                    &asn_mappings,
                    fp,
                )
            });

        info! {"{:#?}", global_peerings.read().unwrap()};
    }
}
