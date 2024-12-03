use std::path::Path;

use crate::utils;

enum Safety {
    /// Idx to try and remove
    Unsafe(Vec<usize>),
    Safe,
}

pub fn run() {
    let lines = match utils::read_file(Path::new("./input/input_day02.txt")) {
        Ok(lines) => lines,
        Err(msg) => {
            eprintln!("Error Occured: {}", msg);
            return;
        }
    };

    let answer = day2(lines.clone(), false);
    println!("Day 2 Part 1: {}", answer);
    let answer_part2 = day2(lines, true);
    println!("Day 2 Part 2: {}", answer_part2);
}

pub fn day2(input: Vec<String>, damping: bool) -> usize {
    let mut count: usize = 0;
    for report in &input {
        let rep = report
            .split(" ")
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        match determine_safe(&rep) {
            Safety::Safe => count += 1,
            Safety::Unsafe(try_indices) => {
                if !damping {
                    continue;
                }
                println!("Before: {:?}", rep);
                for i in 0..rep.len() {
                    let mut rep_clone = rep.clone();
                    rep_clone.remove(i);
                    println!("After: {:?}", rep_clone);
                    if let Safety::Safe = determine_safe(&rep_clone) {
                        count += 1;
                        println!("Safe");
                        break;
                    }
                }
                /* for idx in try_indices {
                    let mut rep_clone = rep.clone();
                    rep_clone.remove(idx);
                    println!("After: {:?}", rep_clone);
                    if let Safety::Safe = determine_safe(&rep_clone) {
                        count += 1;
                        println!("Safe");
                        break;
                    }
                } */
            }
        }
    }
    count
}

fn determine_safe(rep: &[i32]) -> Safety {
    // not inc or decreasing
    if rep[0] == rep[1] {
        return Safety::Unsafe([0, 1].to_vec());
    }
    let increasing: bool = rep[1] > rep[0];
    for i in 1..rep.len() {
        let diff = rep[i] - rep[i - 1];

        if !(1..=3).contains(&diff.abs()) {
            return Safety::Unsafe([i, i - 1].to_vec());
        }
        if (increasing && (diff < 0)) || (!increasing && (diff > 0)) {
            return Safety::Unsafe([i, i - 1].to_vec());
        }
    }
    Safety::Safe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided_example() {
        let input = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];
        assert_eq!(day2(input.clone(), false), 2);
    }

    #[test]
    fn test_provided_example_part2() {
        let input = vec![
            "7 6 4 2 1".to_string(),
            "1 2 7 8 9".to_string(),
            "9 7 6 2 1".to_string(),
            "1 3 2 4 5".to_string(),
            "8 6 4 4 1".to_string(),
            "1 3 6 7 9".to_string(),
        ];
        assert_eq!(day2(input, true), 4);
    }

    #[test]
    fn test_edge() {
        let input = ["12 11 14 17".to_string()].to_vec();
        assert_eq!(day2(input, true), 1);
    }
}
