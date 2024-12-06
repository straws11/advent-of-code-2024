use std::path::Path;

use crate::utils;

pub fn run(path: &Path) {
    let grid: Vec<Vec<char>> = match utils::read_file(path) {
        Ok(lines) => lines
            .into_iter()
            .map(|ln| ln.chars().collect::<Vec<char>>())
            .collect(),
        Err(msg) => {
            eprintln!("Error Occured: {}", msg);
            std::process::exit(1);
        }
    };

    let ans = part1(grid.clone());
    println!("Day 4 Part 1: {}", ans);

    let part2_ans = part2(grid);
    println!("Day 4 Part 2: {}", part2_ans);
}

fn part1(board: Vec<Vec<char>>) -> usize {
    let mut count = 0usize;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let ch = board[i][j];
            if ch == 'X' || ch == 'S' {
                count += count_occurences(&board, i, j, ch);
            }
        }
    }
    count
}

fn count_occurences(board: &Vec<Vec<char>>, row: usize, col: usize, letter: char) -> usize {
    let word_len = 4 - 1; // XMAS len is 4
    let word = "XMAS".to_string();
    let row_max = board.len() - 1;
    let col_max = board[0].len() - 1;
    let mut count = 0usize;
    let direction = if letter == 'X' {
        false
    } else if letter == 'S' {
        true
    } else {
        eprintln!("Invalid letter: {}", letter);
        std::process::exit(1);
    };

    // L->R
    if col + word_len <= col_max {
        let mut test_word = board[row][col..=col + word_len].iter().collect::<String>();
        if direction {
            test_word = test_word.chars().rev().collect::<String>();
        }
        if test_word == word {
            count += 1;
        }
    }
    // T->B
    if row + word_len <= row_max {
        let mut test_word = String::new();
        for i in row..=row + word_len {
            test_word.push(board[i][col]);
        }
        if direction {
            test_word = test_word.chars().rev().collect::<String>();
        }
        if test_word == word {
            count += 1;
        }
    }
    // Diag: RU->LD
    if row + word_len <= row_max && col + word_len <= col_max {
        let mut test_word = String::new();
        for i in 0..=word_len {
            test_word.push(board[i + row][i + col]);
        }
        if direction {
            test_word = test_word.chars().rev().collect::<String>();
        }
        if test_word == word {
            count += 1;
        }
    }

    // Diag: RD->LU
    if row >= word_len && col + word_len <= col_max {
        let mut test_word = String::new();
        for i in 0..=word_len {
            test_word.push(board[row - i][i + col]);
        }
        if direction {
            test_word = test_word.chars().rev().collect::<String>();
        }
        if test_word == word {
            count += 1;
        }
    }

    count
}

fn part2(board: Vec<Vec<char>>) -> usize {
    let mut count = 0usize;
    for i in 0..board.len() {
        for j in 0..board[0].len() {
            let ch = board[i][j];
            if ch == 'A' && is_cross(&board, i, j) {
                count += 1;
            }
        }
    }
    count
}

fn is_cross(board: &[Vec<char>], row: usize, col: usize) -> bool {
    let row_max = board.len() - 1;
    let col_max = board[0].len() - 1;
    // check for being on the edge: early return
    if !(1..row_max).contains(&row) || !(1..col_max).contains(&col) {
        return false;
    }

    let cross_1: String = [board[row - 1][col - 1], 'A', board[row + 1][col + 1]]
        .iter()
        .collect::<String>();
    let cross_2: String = [board[row + 1][col - 1], 'A', board[row - 1][col + 1]]
        .iter()
        .collect::<String>();

    let cross_1_valid = cross_1 == "MAS" || cross_1 == "SAM";
    let cross_2_valid = cross_2 == "MAS" || cross_2 == "SAM";

    cross_2_valid && cross_1_valid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided() {
        let input = "MMMSXXMASM\nMSAMXMSMSA\nAMXSXMAAMM\nMSAMASMSMX\nXMASAMXAMM\nXXAMMXXAMA\nSMSMSASXSS\nSAXAMASAAA\nMAMMMXMMMM\nMXMXAXMASX"
            .split("\n")
            .map(|ln| ln.chars().collect::<Vec<char>>())
            .collect();
        assert_eq!(part1(input), 18);
    }

    #[test]
    fn test_p2() {
        let input = ".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n.........."
            .split("\n").map(|ln| ln.chars().collect::<Vec<char>>()).collect();
        assert_eq!(part2(input), 9);
    }

    /* #[test]
    fn test_provided_part2() {
        let input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))".to_string();
        assert_eq!(day3(input, true), 48);
    } */
}
