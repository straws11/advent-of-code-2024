use std::path::Path;

use crate::utils;

pub fn run() {
    let line = match utils::read_file(Path::new("./input/input_day03.txt")) {
        Ok(lines) => lines.concat(),
        Err(msg) => {
            eprintln!("Error Occured: {}", msg);
            std::process::exit(1);
        }
    };
    let answer = day3(line.clone(), false);
    println!("Day 3 Part 1: {}", answer);
    let answer_2 = day3(line, true);
    println!("Day 3 Part 2: {}", answer_2);
}

fn day3(input: String, part2: bool) -> i32 {
    let mut iter = input.chars().peekable();
    let mut total = 0;
    let mut enabled = true;
    while let Some(item) = iter.next() {
        if enabled && part2 {
            if is_word(&mut iter, "don't()".to_string(), item) {
                enabled = false;
                continue;
            }
        } else if part2 {
            // if disabled
            if is_word(&mut iter, "do()".to_string(), item) {
                enabled = true;
            }
            continue;
        }

        if !is_word(&mut iter, "mul(".to_string(), item) {
            continue;
        }
        // can now assume we have read `mul(`
        let mut digits_1 = String::new();
        let mut digits_2 = String::new();
        while let Some(c) = iter.next_if(|ch| ch.is_ascii_digit()) {
            digits_1.push(c);
        }
        if let Some(comma) = iter.next() {
            if comma != ',' {
                continue;
            }
        }
        while let Some(c) = iter.next_if(|ch| ch.is_ascii_digit()) {
            digits_2.push(c);
        }

        if let Some(r_paren) = iter.next() {
            if r_paren != ')' {
                continue;
            }
        }

        // successful
        let num1 = match digits_1.parse::<i32>() {
            Ok(val) => val,
            Err(_) => continue,
        };
        let num2 = match digits_2.parse::<i32>() {
            Ok(val) => val,
            Err(_) => continue,
        };
        total += num1 * num2;
    }

    total
}

fn is_word<I>(iter: &mut I, word: String, item: char) -> bool
where
    I: Iterator<Item = char>,
{
    let mut word_iter = word.chars();
    if let Some(first) = word_iter.next() {
        if item != first {
            return false;
        }
    }

    for c in word_iter {
        if !is_next_eq(iter, c) {
            return false;
        }
    }

    true
}

fn is_next_eq<I>(iter: &mut I, cmp_char: char) -> bool
where
    I: Iterator<Item = char>,
{
    if let Some(item) = iter.next() {
        cmp_char == item
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided() {
        let input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
        assert_eq!(day3(input, false), 161);
    }

    #[test]
    fn test_edge() {
        let input = "don't()do()mul(2,3)mul(3,4)do()mul(2,1)don't()mul(100,2)do()don't()mul(3,5)"
            .to_string();
        assert_eq!(day3(input, true), 20);
    }

    #[test]
    fn test_provided_part2() {
        let input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        assert_eq!(day3(input, true), 48);
    }
}
