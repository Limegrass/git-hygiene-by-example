use clap::Parser;
use git_history_by_example::custom_add_2025_04_24_16_26_36_619244218;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// First number to add
    first: i64, // matches type of git_history_by_example::custom_add_2025_04_24_16_26_36_619244218;

    /// Second number to add
    second: i64, // matches type of git_history_by_example::custom_add_2025_04_24_16_26_36_619244218;
}

fn main() {
    let args = Args::parse();
    let total = custom_add_2025_04_24_16_26_36_619244218(args.first, args.second);
    println!("{total}");
}
