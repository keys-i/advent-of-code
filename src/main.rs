#[allow(unused_imports)]
use advent_of_code::year2024;
use aoc_runner_derive::aoc_main;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <year>", args[0]);
        std::process::exit(1);
    }

    let year = &args[1];

    match year.as_str() {
        "2024" => {
            aoc_main! { lib = advent_of_code::year2024 }
        }
        // Add more years as needed
        _ => {
            eprintln!("Solutions for year {} are not available.", year);
            std::process::exit(1);
        }
    }
}
