pub mod args;
pub mod comm_data;
pub mod comm_mappings;
pub mod http;
pub mod logging;
pub mod mrt_asn;
pub mod mrt_communities;
pub mod mrt_peer;
pub mod mrt_route;
pub mod parse_mrt;
pub mod parse_threaded;
pub mod peer_attrs;
pub mod peerings;
pub mod ribs;

use crate::parse_threaded::threaded_parser::{parse_rib_file_threaded, parse_rib_files};
use crate::peerings::global_peerings::GlobalPeerings;
use crate::ribs::rib_getter::download_ribs_for_day;
use crate::{args::cli_args::RibsSource, ribs::rib_getter::RibFile};
use rayon::ThreadPoolBuilder;

fn main() {
    let args = args::cli_args::parse_cli_arg();
    if args.debug {
        logging::setup_logging("debug");
    } else {
        logging::setup_logging("info");
    }

    ThreadPoolBuilder::new()
        .num_threads((args.threads).try_into().unwrap())
        .build_global()
        .unwrap();

    let global_peerings: GlobalPeerings = match args.ribs_source {
        // Download MRT files and then parse them - one file per thread
        RibsSource::Download(_) => {
            let rib_files = download_ribs_for_day(args.get_ribs_ymd(), args.get_ribs_path());
            parse_rib_files(&rib_files)
        }

        // Parse a single existing file - split across multiple threads
        RibsSource::File(_) => parse_rib_file_threaded(args.get_rib_file()),

        // Parse multiple existing files - one file per thread
        RibsSource::Files(_) => {
            let rib_files: Vec<RibFile> = args
                .get_rib_files()
                .iter()
                .map(|filename| RibFile {
                    url: String::new(),
                    filename: filename.clone(),
                })
                .collect();

            parse_rib_files(&rib_files)
        }
    };

    global_peerings.to_file(&args.json);
}
