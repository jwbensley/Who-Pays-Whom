pub mod args;
pub mod comm_mappings;
pub mod http;
pub mod logging;
pub mod mrt_asn;
pub mod mrt_communities;
pub mod mrt_large_communities;
pub mod mrt_route;
pub mod parse;
pub mod peerings;
pub mod ribs;

use crate::ribs::rib_getter::RibFile;

fn main() {
    let args = args::cli_args::parse_cli_arg();
    if args.debug {
        logging::setup_loggin("debug");
    } else {
        logging::setup_loggin("info");
    }

    let rib_files: Vec<RibFile> = if args.download() {
        ribs::rib_getter::download_ribs_for_day(args.get_ribs_ymd(), args.get_ribs_path())
    } else {
        args.get_rib_files()
            .iter()
            .map(|filename| RibFile {
                url: String::new(),
                filename: filename.clone(),
            })
            .collect()
    };

    parse::rib_parser::find_peer_data(&rib_files, &args.threads);
}
