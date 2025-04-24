use clap::Parser;
use git_history_by_example::custom_add_2025_04_24_16_29_50_517072995;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// First number to add
    first: f64, // matches type of git_history_by_example::custom_add_2025_04_24_16_29_50_517072995;

    /// Second number to add
    second: f64, // matches type of git_history_by_example::custom_add_2025_04_24_16_29_50_517072995;
}

fn main() {
    let args = Args::parse();
    let total = custom_add_2025_04_24_16_29_50_517072995(args.first, args.second);
    println!("{total}");
}
