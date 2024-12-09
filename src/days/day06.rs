use std::path::Path;

use crate::utils;

#[derive(Default, Debug)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

#[derive(Default, Debug)]
struct Guard {
    pub direction: Direction,
    pub position: (usize, usize),
    pub finished: bool,
}

impl Guard {
    fn r#move(&mut self, grid: &mut [Vec<char>]) {
        match self.direction {
            Direction::Up => {
                if self.obstacle_incoming(grid) {
                    self.direction = Direction::Right;
                    return;
                }
                self.position.1 -= 1;
                grid[self.position.1][self.position.0] = 'X';
                if self.position.1 == 0 {
                    self.finished = true;
                }
            }
            Direction::Down => {
                if self.obstacle_incoming(grid) {
                    self.direction = Direction::Left;
                    return;
                }
                self.position.1 += 1;
                grid[self.position.1][self.position.0] = 'X';
                if self.position.1 == grid.len() - 1 {
                    self.finished = true;
                }
            }
            Direction::Left => {
                if self.obstacle_incoming(grid) {
                    self.direction = Direction::Up;
                    return;
                }
                self.position.0 -= 1;
                grid[self.position.1][self.position.0] = 'X';
                if self.position.0 == 0 {
                    self.finished = true;
                }
            }
            Direction::Right => {
                if self.obstacle_incoming(grid) {
                    self.direction = Direction::Down;
                    return;
                }
                self.position.0 += 1;
                grid[self.position.1][self.position.0] = 'X';
                if self.position.0 == grid[0].len() - 1 {
                    self.finished = true;
                }
            }
        }
    }

    fn obstacle_incoming(&self, grid: &[Vec<char>]) -> bool {
        match self.direction {
            Direction::Up => grid[self.position.1 - 1][self.position.0] == '#',
            Direction::Down => grid[self.position.1 + 1][self.position.0] == '#',
            Direction::Left => grid[self.position.1][self.position.0 - 1] == '#',
            Direction::Right => grid[self.position.1][self.position.0 + 1] == '#',
        }
    }
}

pub fn run(path: &Path) {
    let lines: Vec<String> = match utils::read_file(path) {
        Ok(lines) => lines,
        Err(msg) => {
            eprintln!("Error Occured: {}", msg);
            std::process::exit(1);
        }
    };
    let grid = lines
        .iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let ans = part1(&mut grid.clone());
    println!("Day 6 Part 1: {}", ans);
    // let ans_2 = part1(&mut grid);
    // println!("Day 6 Part 1: {}", ans_2);
}

fn part1(grid: &mut [Vec<char>]) -> usize {
    // get position of arrow
    let mut guard = Guard::default();
    'outer: for (i, row) in grid.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if *item == '^' {
                guard.position = (j, i);
                grid[i][j] = 'X';
                break 'outer;
            }
        }
    }
    // move across board
    while !guard.finished {
        guard.r#move(grid);
    }

    // count visited slots
    let mut count = 0usize;
    for row in grid {
        for item in row {
            if *item == 'X' {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided() {
        let mut input = "....#.....\n.........#\n..........\n..#.......\n.......#..\n..........\n.#..^.....\n........#.\n#.........\n......#...".split("\n").map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
        assert_eq!(part1(&mut input), 41);
    }
}
