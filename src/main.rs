use clap::Parser;
use git_history_by_example::custom_add_2025_04_24_16_29_50_817463682;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// First number to add
    first: f64, // matches type of git_history_by_example::custom_add_2025_04_24_16_29_50_817463682;

    /// Second number to add
    second: f64, // matches type of git_history_by_example::custom_add_2025_04_24_16_29_50_817463682;
}

fn main() {
    let args = Args::parse();
    let total = custom_add_2025_04_24_16_29_50_817463682(args.first, args.second);
    println!("{total}");
}
