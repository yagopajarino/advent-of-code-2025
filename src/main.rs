mod day01;
mod utils;
use clap::Parser;

/// Advent of Code 2025 Solutions
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Day
    #[arg(short, long)]
    day: u8,

    /// Part
    #[arg(short, long, default_value_t = 1)]
    part: u8,
}

fn main() {
    let args = Args::parse();
    let res = match args.day {
        1 => day01::solve(args.part),
        _ => -1,
    };

    println!(
        "Results for day {:?} part {:?}: {}",
        args.day, args.part, res
    )
}
