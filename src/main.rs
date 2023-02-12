use clap::Parser;
use clap::{command, arg};

mod anime_raw_search;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct AnimeSearchArgs {
    #[arg(short, long, help = "The name of the anime you want to search for")]
    anime_name: String,

    #[arg(short, long, default_value = "false", help = "Prints the results without downloading them")]
    dry_run: bool,
}

fn main() {
    let args = AnimeSearchArgs::parse();
    println!("Args: {:#?}", args);
    let result = anime_raw_search::search_anime_raw(args.anime_name);
    println!("Search Result: {:#?}", result);
}