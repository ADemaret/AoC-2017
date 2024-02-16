use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
// use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 19 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day19_input_demo1.txt");
    let input = include_str!("../assets/day19_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<String> {
    let grid = Grid2D::new(input);
    // grid.print();

    let mut start_node = (0, 0);
    for c in 0..grid.max_c {
        if grid.grid[0][c] == '|' {
            start_node = (0, c);
            break;
        }
    }
    // println!("we start at ({},{})", start_node.0, start_node.1);
    let mut current_node = start_node;
    let mut direction = (1, 0);
    let mut str = String::new();
    let mut done = false;
    while !done {
        next_step(
            &grid,
            &mut current_node,
            &mut direction,
            &mut str,
            &mut done,
        );
        // pause::pause();
    }
    Some(str)
}

fn next_step(
    grid: &Grid2D,
    current_node: &mut (usize, usize),
    direction: &mut (isize, isize),
    str: &mut String,
    done: &mut bool,
) {
    let l1 = current_node.0 as isize + direction.0;
    let c1 = current_node.1 as isize + direction.1;
    if l1 > 0 && c1 > 0 && (l1 as usize) < grid.max_l && (c1 as usize) < grid.max_c {
        // println!("({},{}) '{}'", l1, c1, grid.grid[l1 as usize][c1 as usize]);
        match grid.grid[l1 as usize][c1 as usize] {
            '+' => {
                /* new direction */
                let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                for dir in directions {
                    if dir.0 != direction.0
                        && dir.1 != direction.1
                        && dir.0 != -direction.0
                        && dir.1 != -direction.1
                    {
                        let mut valid = None;
                        // println!("  test direction {:?}", dir);
                        let mut l2 = l1;
                        let mut c2 = c1;
                        while valid.is_none() {
                            l2 += dir.0;
                            c2 += dir.1;
                            test_valid_direction(grid, &mut (l2, c2), &dir, &mut valid);
                        }
                        if valid.is_some_and(|b| b) {
                            *current_node = (l1 as usize, c1 as usize);
                            *direction = dir;
                            return;
                        }
                    }
                }
                *done = true;
            }
            c if c.is_ascii_alphabetic() => {
                str.push(c);
                *current_node = (l1 as usize, c1 as usize);
            }
            '|' | '-' => {
                *current_node = (l1 as usize, c1 as usize);
            }
            ' ' => *done = true,
            w => {
                println!("unexpected char : {}", w);
                unreachable!();
            }
        }
    } else {
        *done = true;
    }
}

fn test_valid_direction(
    grid: &Grid2D,
    current_node: &mut (isize, isize),
    dir: &(isize, isize),
    valid: &mut Option<bool>,
) {
    let l1 = current_node.0;
    let c1 = current_node.1;
    if l1 > 0 && c1 > 0 && (l1 as usize) < grid.max_l && (c1 as usize) < grid.max_c {
        match grid.grid[l1 as usize][c1 as usize] {
            '+' => {
                // println!("  ({l1},{c1}) (+) => valid");
                *valid = Some(true);
            }
            c if c.is_ascii_alphabetic() => {
                *valid = Some(true);
            }
            '|' => {
                if dir.0 == 0 {
                    *valid = Some(true);
                }
            }
            '-' => {
                if dir.1 == 0 {
                    *valid = Some(true);
                }
            }
            ' ' => {
                // println!("  ({l1},{c1}) ( ) => invalid");
                *valid = Some(false);
            }
            w => {
                println!("  unexpected char : {}", w);
                unreachable!();
            }
        }
    } else {
        *valid = Some(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day19_input_demo1.txt")),
            Some("ABCDEF".to_string())
        );
        assert_eq!(
            get_answer(include_str!("../assets/day19_input.txt")),
            Some("LXWCKGRAOY".to_string())
        );
    }
}
