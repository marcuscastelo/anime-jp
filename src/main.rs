use clap::{Parser, ValueEnum};
use clap::{command, arg};

mod raws;
mod subs;
mod core;

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
}

fn main() {
    let args = Args::parse();
    println!("Args: {:#?}", args);

    if args.search_type == SearchType::Raw || args.search_type == SearchType::Both {
        let result = raws::search::search_anime_raw(args.anime_name.as_str());
        println!("Search RAW Result: {:#?}", result);
    }

    if args.search_type == SearchType::Subtitles || args.search_type == SearchType::Both {
        let result = subs::search::fetch_best_indexers_for(args.anime_name.as_str());
        println!("Search Subs Result: {:#?}", result);
    }
}