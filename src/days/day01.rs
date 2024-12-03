use std::path::Path;

use crate::utils;

pub fn run(path: &Path) {
    let lines = match utils::read_file(&path) {
        Ok(lines) => lines,
        Err(msg) => {
            eprintln!("Error Occured: {msg}");
            std::process::exit(1);
        }
    };
    /* println!("{:?}", lines); */
    let size = lines.len();
    let mut a: Vec<usize> = vec![0; size];
    let mut b: Vec<usize> = vec![0; size];

    for (i, s) in lines.into_iter().enumerate() {
        let twos = s
            .split("   ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        a[i] = twos[0];
        b[i] = twos[1];
    }
    a.sort();
    b.sort();
    part1(a.clone(), b.clone());
    part2(a, b);
}

fn part1(a: Vec<usize>, b: Vec<usize>) {
    let mut total = 0;
    for i in 0..a.len() {
        total += a[i].abs_diff(b[i]);
    }
    println!("Day 1 Part 1: {}", total);
}

fn part2(a: Vec<usize>, b: Vec<usize>) {
    let mut total = 0;
    for left in &a {
        let mut count = 0;
        for right in &b {
            if left == right {
                count += 1;
            }
        }
        total += left * count;
    }
    println!("Day 1 Part 2: {}", total);
}
