mod days;
mod utils;

use std::{env, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: cargo run <day_number> [<input_file_path>]");
        std::process::exit(1);
    }

    let day = args[1].parse::<usize>().unwrap_or_else(|_| {
        eprintln!("Invalid day number: {}", args[1]);
        std::process::exit(1);
    });

    let str_path = format!("./input/input_day{:02}.txt", day).to_string();
    let file_path = if args.len() >= 3 {
        Path::new(&args[2])
    } else {
        Path::new(&str_path)
    };

    match day {
        1 => days::day01::run(file_path),
        2 => days::day02::run(file_path),
        3 => days::day03::run(file_path),
        4 => days::day04::run(file_path),
        5 => days::day05::run(file_path),
        6 => days::day06::run(file_path),
        // 7 => days::day07::run(file_path),
        // 8 => days::day08::run(file_path),
        // 9 => days::day09::run(file_path),
        _ => {
            eprintln!("Day {} not implemented yet!", day);
            std::process::exit(1);
        }
    }
}
