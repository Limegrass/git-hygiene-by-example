use clap::Parser;
use git_history_by_example::custom_add_2025_04_24_16_29_55_338453858;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// First number to add
    first: f64, // matches type of git_history_by_example::custom_add_2025_04_24_16_29_55_338453858;

    /// Second number to add
    second: f64, // matches type of git_history_by_example::custom_add_2025_04_24_16_29_55_338453858;
}

fn main() {
    let args = Args::parse();
    let total = custom_add_2025_04_24_16_29_55_338453858(args.first, args.second);
    println!("{total}");
}
