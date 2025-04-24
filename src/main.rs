use clap::Parser;
use git_history_by_example::custom_add_2025_04_24_16_26_36_519127145;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// First number to add
    first: i64, // matches type of git_history_by_example::custom_add_2025_04_24_16_26_36_519127145;

    /// Second number to add
    second: i64, // matches type of git_history_by_example::custom_add_2025_04_24_16_26_36_519127145;
}

fn main() {
    let args = Args::parse();
    let total = custom_add_2025_04_24_16_26_36_519127145(args.first, args.second);
    println!("{total}");
}
