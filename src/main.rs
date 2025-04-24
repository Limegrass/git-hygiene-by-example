use clap::Parser;
use git_history_by_example::custom_add;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// First number to add
    first: i64, // matches type of git_history_by_example::custom_add;

    /// Second number to add
    second: i64, // matches type of git_history_by_example::custom_add;
}

fn main() {
    let args = Args::parse();
    let total = custom_add(args.first, args.second);
    println!("{total}");
}
