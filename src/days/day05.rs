use crate::utils;
use std::path::Path;

pub fn run(path: &Path) {
    let lines = match utils::read_file(path) {
        Ok(lines) => lines,
        Err(msg) => {
            eprintln!("Error Occured: {}", msg);
            std::process::exit(1);
        }
    };
    let ans = day5(lines.clone(), false);
    println!("Day 5 Part 1: {}", ans);
    let ans2 = day5(lines, true);
    println!("Day 5 Part 2: {}", ans2);
}

fn day5(input: Vec<String>, part2: bool) -> usize {
    let mut restrictions: Vec<(usize, usize)> = Vec::new();
    let mut updates_start_idx = 0usize;
    for line in &input {
        updates_start_idx += 1;
        if line.is_empty() {
            break; // done with restrictions
        }
        let pair: Vec<usize> = line
            .split("|")
            .map(|s| s.parse::<usize>().unwrap())
            .collect();
        restrictions.push((pair[0], pair[1]));
    }
    // updates:
    let mut middle_total = 0usize;
    for update in input.iter().skip(updates_start_idx) {
        let update = update
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        if !part2 {
            if in_order(&update, &restrictions) {
                let mid = update[(update.len() - 1) / 2];
                middle_total += mid;
            }
        } else {
            // part 2:
            if !in_order(&update, &restrictions) {
                let mut fixing_update = update.clone();
                let mut fault: Option<(usize, usize)> = fault_pos(&fixing_update, &restrictions);
                while fault != None {
                    // swap
                    let positions = fault.unwrap();
                    fixing_update.swap(positions.0, positions.1);
                    fault = fault_pos(&fixing_update, &restrictions);
                }
                let mid = fixing_update[(fixing_update.len() - 1) / 2];
                middle_total += mid;
            }
        }
    }
    middle_total
}

fn fault_pos(update: &[usize], res: &[(usize, usize)]) -> Option<(usize, usize)> {
    for ord in res {
        let pos_1: usize = match update.iter().position(|page| *page == ord.0) {
            Some(pos) => pos,
            None => continue,
        };
        let pos_2: usize = match update.iter().position(|page| *page == ord.1) {
            Some(pos) => pos,
            None => continue,
        };
        if pos_2 > pos_1 {
            return Some((pos_1, pos_2));
        }
    }
    None
}

fn in_order(update: &[usize], res: &[(usize, usize)]) -> bool {
    for ord in res {
        let pos_1: usize = match update.iter().position(|page| *page == ord.0) {
            Some(pos) => pos,
            None => continue,
        };
        let pos_2: usize = match update.iter().position(|page| *page == ord.1) {
            Some(pos) => pos,
            None => continue,
        };

        // broken ordering
        if pos_1 > pos_2 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provided() {
        let input = "47|53\n97|13\n97|61\n97|47\n75|29\n61|13\n75|53\n29|13\n97|29\n53|29\n61|53\n97|53\n61|29\n47|13\n75|47\n97|75\n47|61\n75|61\n47|29\n75|13\n53|13\n\n75,47,61,53,29\n97,61,53,29,13\n75,29,13\n75,97,47,61,53\n61,13,29\n97,13,75,29,47".split("\n")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
        assert_eq!(part1(input), 143);
    }

    /* #[test]
    fn test_p2() {
        let input = ".M.S......\n..A..MSMS.\n.M.S.MAA..\n..A.ASMSM.\n.M.S.M....\n..........\nS.S.S.S.S.\n.A.A.A.A..\nM.M.M.M.M.\n.........."
            .split("\n").map(|ln| ln.chars().collect::<Vec<char>>()).collect();
        // assert_eq!(part2(input), 9);
    } */
}
