/*A cli that generates random fruits */
use fruit::get_fruits;
use fruit::add_fruit;
use fruit::remove_fruit;
use clap::Parser;

/// CLI tool to return random fruits
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The quantity of fruits to return
    #[clap(short, long, default_value = "1")]
    count: u32,

    /// The fruit to add
    #[clap(short, long)]
    add: Option<String>,

    /// The fruit to remove
    #[clap(short, long)]
    remove: Option<String>,
}

fn main() {
    let args = Args::parse();

    if let Some(fruit) = args.add {
        add_fruit(&fruit);
        println!("Added fruit: {}", fruit);
    } else if let Some(fruit) = args.remove {
        remove_fruit(&fruit);
        println!("Removed fruit: {}", fruit);
    } else {
        let fruits = get_fruits(args.count);
        if fruits.is_empty() {
            println!("No fruits available.");
        } else {
            println!("Fruits: {:?}", fruits);
        }
    }
}
