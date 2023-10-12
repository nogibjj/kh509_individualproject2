/*A cli that generates random fruits */

use calc_cli_with_tests::{get_fruits, add_fruit, remove_fruit};
use clap::Parser;

/// CLI tool to return random fruits
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The quantity of fruits to return
    #[clap(short, long, default_value = "1")]
    count: u32,

    /// The fruit to add to the list
    #[clap(short, long)]
    add: Option<String>,

    /// The fruit to remove from the list
    #[clap(short, long)]
    remove: Option<String>,
}

fn main() {
    let args = Args::parse();

    // Handle adding a new fruit
    if let Some(fruit) = args.add {
        add_fruit(&fruit);
        println!("Added {} to the list of fruits.", fruit);
    }

    // Handle removing a fruit
    if let Some(fruit) = args.remove {
        remove_fruit(&fruit);
        println!("Removed {} from the list of fruits.", fruit);
    }

    // Get and print random fruits based on the count
    let fruits = get_fruits(args.count);
    println!("fruits: {:?}", fruits);
}
