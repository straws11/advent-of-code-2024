mod days;
mod utils;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <day_number>");
        std::process::exit(1);
    }

    let day = args[1].parse::<usize>().unwrap_or_else(|_| {
        eprintln!("Invalid day number: {}", args[1]);
        std::process::exit(1);
    });

    match day {
        1 => days::day01::run(),
        2 => days::day02::run(),
        3 => days::day03::run(),
        // 4 => days::day04::run(),
        _ => {
            eprintln!("Day {} not implemented yet!", day);
            std::process::exit(1);
        }
    }
}
