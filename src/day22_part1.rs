use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 22 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day22_input_demo1.txt");
    let input = include_str!("../assets/day22_input.txt");
    let moves = 10000;

    if let Some(answer) = get_answer(input, moves) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, moves: usize) -> Option<usize> {
    let mut cells: HashMap<(isize, isize), char> = HashMap::new();
    input.lines().enumerate().for_each(|(l, line)| {
        line.chars().enumerate().for_each(|(c, ch)| {
            cells.insert((l as isize, c as isize), ch);
        })
    });
    let width = input.lines().next().unwrap().len();
    let length = input.lines().count();

    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    let mut current_dir = 0; // default dir is up
    let mut current_pos = ((length / 2) as isize, (width / 2) as isize);
    let mut total = 0;
    // print_cells(&cells, -3, 6, -3, 6, current_pos);
    for _ in 0..moves {
        let current_state = cells.get(&current_pos).unwrap();
        // println!("in ({:?}) : '{}'", current_pos, current_state);
        match current_state {
            '.' => {
                current_dir = (current_dir + 3) % 4;
                cells.insert(current_pos, '#');
                total += 1;
                current_pos = (
                    current_pos.0 + directions[current_dir].0,
                    current_pos.1 + directions[current_dir].1,
                );
                // println!("moving left to ({:?})", current_pos);
            }
            '#' => {
                current_dir = (current_dir + 1) % 4;
                cells.insert(current_pos, '.');
                current_pos = (
                    current_pos.0 + directions[current_dir].0,
                    current_pos.1 + directions[current_dir].1,
                );
                // println!("moving right to ({:?})", current_pos);
            }
            _ => unreachable!(),
        }
        if cells.get(&current_pos).is_none() {
            cells.insert(current_pos, '.');
        }
        // print_cells(&cells, -3, 6, -3, 6, current_pos);
    }
    Some(total)
}

fn _print_cells(
    cells: &HashMap<(isize, isize), char>,
    l0: isize,
    l1: isize,
    c0: isize,
    c1: isize,
    current: (isize, isize),
) {
    for l in l0..l1 {
        for c in c0..c1 {
            if (l, c) == (current) {
                print!(":");
            } else {
                print!(" ")
            }
            if let Some(ch) = cells.get(&(l, c)) {
                print!("{ch}");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day22_input_demo1.txt"), 7),
            Some(5)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day22_input_demo1.txt"), 70),
            Some(41)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day22_input_demo1.txt"), 10000),
            Some(5587)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day22_input.txt"), 10000),
            Some(5462)
        );
    }
}
