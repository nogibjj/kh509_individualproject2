use fruits::get_fruits;
use fruits::add_fruit;
use fruits::remove_fruit;
use clap::Parser;

/// CLI tool to return random fruits
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The quantity of fruits to return
    #[clap(short, long, default_value = "1")]
    count: u32,
}

fn main() {
    let args = Args::parse();
    let fruits = get_fruits(args.count);
    println!("fruits: {:?}", fruits);
}

