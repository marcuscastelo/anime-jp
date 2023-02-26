use clap::{arg, command};
use clap::{Parser, ValueEnum};
use error_stack::ResultExt;
use fern::colors::{Color, ColoredLevelConfig};
use indicatif::ProgressBar;
use log::LevelFilter;

use crate::core::download::downloader::{Destination, FileDownloader};
use crate::core::indexer::Indexer;
use crate::qbittorrent::api::torrents::TorrentList;
use crate::raws::download::AnimeRawDownloader;
use crate::subs::download::AnimeSubsDownloader;

mod core;
mod prelude;
mod qbittorrent;
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
        default_value = "raw",
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
                "{}({})[{}:{}][{}] {}",
                chrono::Local::now().format("%H:%M:%S "),
                record.target(),
                record.file().unwrap_or("unknown"),
                record.line().unwrap_or(0),
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

fn search_raws(args: &Args) {
    log::info!("Searching for anime raws for: {}", args.anime_name);
    let result = raws::search::search_anime_raws(args.anime_name.as_str());

    let indexers = match result {
        Ok(result) => result,
        Err(e) => {
            log::error!("\n{e:?}");
            return;
        }
    };

    log::info!(
        "Found {} raws for anime {}",
        indexers.len(),
        args.anime_name
    );
    log::trace!("Found raws: {:#?}", indexers);

    if args.dry_run {
        log::info!("Dry run, not downloading raws");
        return;
    }

    log::trace!("Creating downloader...");
    let downloader = AnimeRawDownloader::new();

    log::info!("Queueing raws...");
    let pb = ProgressBar::new(indexers.len() as u64);
    for raw_data in indexers {
        let dest = Destination::Default;

        //TODO: melhorar essa conversão (ou nem ter conversão)
        let indexer = Indexer::new(&raw_data.anime_name, &raw_data.anime_raw_magnet);
        let result = downloader.download_indexer_to_file(&indexer, &dest);

        match result {
            Ok(_) => log::info!("Enqueued raw: {:#?}", raw_data),
            Err(e) => log::error!("\n{e:?}"),
        }
    }

    pb.finish();

    log::info!("Waiting for downloads to finish...");

    let pb = ProgressBar::new(0);
    let result = downloader.wait_for_completion(|torrents: &TorrentList| {
        let total_bytes = torrents.0.iter().map(|t| t.size().clone() as u64).sum();
        let total_downloaded_bytes = torrents
            .0
            .iter()
            .map(|t| t.downloaded().clone() as u64)
            .sum();
        pb.set_length(total_bytes);
        pb.set_position(total_downloaded_bytes);
    });
    pb.finish();

    match result {
        Ok(_) => log::info!("Finished downloading raws"),
        Err(e) => log::error!("\n{e:?}"),
    }

    log::info!("Finished downloading raws");
}

fn search_subs(args: &Args) {
    log::info!("Searching for anime subtitles for: {}", args.anime_name);
    let indexers = subs::search::fetch_best_indexers_for(args.anime_name.as_str());

    let anime_indexers = match indexers {
        Ok(indexers) => indexers,
        Err(e) => {
            log::error!("Failed to fetch indexers: {}", e);
            return;
        }
    };

    let anime_indexer = anime_indexers.get(0).expect("No indexers found");
    log::debug!("Found anime indexer: {:#?}", anime_indexer);
    log::info!("Found anime!: {:#?}", anime_indexer.name());

    log::debug!("Fetching sub files for anime indexer...");
    let subs_indexers = subs::search::fetch_sub_files(anime_indexer).unwrap();
    log::info!(
        "Found {} subs for anime {}",
        subs_indexers.len(),
        anime_indexer.name()
    );
    log::trace!("Subs indexers: {:#?}", subs_indexers);

    if args.dry_run {
        log::info!("Dry run, not downloading subs");
        return;
    }

    log::trace!("Creating downloader...");
    let downloader = AnimeSubsDownloader::new();

    log::info!("Downloading subs...");
    let pb = ProgressBar::new(subs_indexers.len() as u64);
    for subs_indexer in subs_indexers {
        let result = downloader.download_indexer_to_file(&subs_indexer, &Destination::Default);

        match result {
            Ok(_) => log::trace!("Downloaded subs: {}", subs_indexer.name()),
            Err(err) => log::error!("\n{err:?}"),
        }

        std::thread::sleep(std::time::Duration::from_millis(10));
        pb.inc(1);
    }
    pb.finish();
    log::info!("Finished downloading subs");
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

    log::info!("Search type: {:#?}", args.search_type);

    if args.search_type == SearchType::Raw || args.search_type == SearchType::Both {
        search_raws(&args);
    }

    if args.search_type == SearchType::Subtitles || args.search_type == SearchType::Both {
        search_subs(&args);
    }

    log::info!("Done!");
}
