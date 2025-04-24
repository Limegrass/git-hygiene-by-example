use clap::Parser;
use git_history_by_example::custom_add;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// First number to add
    first: u128,

    /// Second number to add
    second: u128,
}

fn main() {
    let args = Args::parse();
    let total = custom_add(args.first, args.second);
    println!("{total}");
}
