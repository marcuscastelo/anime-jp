use clap::{arg, command};
use clap::{Parser, ValueEnum};
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

use crate::core::download::downloader::{Destination, FileDownloader};
use crate::subs::download::SubsDownloader;

mod core;
mod raws;
mod subs;
mod qbittorrent;

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

    #[arg(
        short,
        long,
        default_value = "false",
        help = "Prints the results without downloading them"
    )]
    dry_run: bool,

    #[arg(
        value_enum,
        short,
        long,
        default_value = "subtitles",
        help = "The type of search you want to perform"
    )]
    search_type: SearchType,

    #[arg(short, long, default_value = "false", help = "Show more logs")]
    verbose: bool,

    #[arg(short, long, default_value = "false", help = "Show even more logs")]
    trace: bool,
}

fn setup_logger(level: LevelFilter) -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .debug(Color::Cyan);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}

fn main() {
    let args = Args::parse();
    log::info!("Starting Anime Downloader");

    let level = match args.verbose {
        true => LevelFilter::Debug,
        false => LevelFilter::Info,
    };

    let level = match args.trace {
        true => LevelFilter::Trace,
        false => level,
    };

    setup_logger(level).expect("Failed to setup logger");
    log::trace!("Args: {:#?}", args);

    if args.verbose {
        log::info!("Setting log level to trace");
        log::set_max_level(log::LevelFilter::Trace);
    }

    if args.search_type == SearchType::Raw || args.search_type == SearchType::Both {
        log::info!("Searching for anime raws for: {}", args.anime_name);
        let result = raws::search::search_anime_raws(args.anime_name.as_str());
        println!("Search for anime: {:#?}", result);
    }

    if args.search_type == SearchType::Subtitles || args.search_type == SearchType::Both {
        log::info!("Searching for anime subtitles for: {}", args.anime_name);
        let indexers = subs::search::fetch_best_indexers_for(args.anime_name.as_str());
        println!("Search for anime: {:#?}", indexers);
        let anime_indexers = match indexers {
            Ok(indexers) => indexers,
            Err(e) => {
                log::error!("Failed to fetch indexers: {}", e);
                return;
            }
        };

        let anime_indexer = anime_indexers.get(0).expect("No indexers found");
        log::info!("Found anime indexer: {:#?}", anime_indexer);

        log::info!("Fetching sub files for anime indexer...");
        let subs_indexers = subs::search::fetch_sub_files(anime_indexer).unwrap();
        log::info!("Found subs indexers: {:#?}", subs_indexers);

        if !args.dry_run {
            log::trace!("Creating downloader...");
            let downloader = SubsDownloader::new();

            log::info!("Downloading subs...");
            downloader.download_indexers_to_file(&subs_indexers, &Destination::Default).unwrap();
            log::info!("Done!");
        }
    }
}
