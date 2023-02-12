use clap::{Parser, ValueEnum};
use clap::{command, arg};

mod core;
mod raws;
mod subs;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum SearchType {
    Subtitles = 1,
    Raw = 2,
    Both = 3,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, help = "The name of the anime you want to search for")]
    anime_name: String,

    #[arg(short, long, default_value = "false", help = "Prints the results without downloading them")]
    dry_run: bool,

    #[arg(value_enum, short, long, default_value = "subtitles", help = "The type of search you want to perform")]
    search_type: SearchType,

    #[arg(short, long, default_value = "true", help = "Show more logs")]
    verbose: bool,
}

fn main() {
    log::info!("Starting Anime Downloader");
    let args = Args::parse();
    println!("Args: {:#?}", args);

    if args.verbose {
        log::info!("Setting log level to trace");
        log::set_max_level(log::LevelFilter::Trace);
    }

    if args.search_type == SearchType::Raw || args.search_type == SearchType::Both {
        let result = raws::search::search_anime_raws(args.anime_name.as_str());
        println!("Search RAW Result: {:#?}", result);
    }

    if args.search_type == SearchType::Subtitles || args.search_type == SearchType::Both {
        let indexers = subs::search::fetch_best_indexers_for(args.anime_name.as_str());
        println!("Search Subs Result: {:#?}", indexers);

        let indexers = match indexers {
            Ok(indexers) => indexers,
            Err(e) => {
                log::error!("Failed to fetch indexers: {}", e);
                return;
            }
        };

        if !args.dry_run {
            let indexer = indexers.get(0).expect("No indexers found");
            let download_result = subs::search::fetch_sub_files(indexer);
            println!("Download Subs Result: {:#?}", download_result);
        }
    }
}