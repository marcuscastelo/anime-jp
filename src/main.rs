use clap::Parser;
use clap::{command, arg};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    anime_name: String,
}

fn main() {
    let args = Args::parse();
    println!("{:#?}", args);
}
