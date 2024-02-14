use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;
use crate::day10_part2;

pub fn main() {
    println!("-- Advent of Code - Day 14 - Part 2 --");
    let now = Instant::now();

    // let input = "flqrgnkx";
    let input = include_str!("../assets/day14_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<usize> {
    let mut grid = Grid2D::new_by_sizes(128, 128, '.');

    for line in 0..128 {
        let key = format!("{input}-{line}");
        let hex = day10_part2::get_answer(&key);
        let bin = hex_to_binary(hex.clone());
        for (col, c) in bin.chars().enumerate() {
            grid.grid[line][col] = c;
        }
    }
    //grid.print_portion(0, 0, &mut 8, &mut 8);
    grid.count_areas()
}

fn hex_to_binary(s: String) -> String {
    let mut b = String::new();
    s.chars()
        .map(|i| i.to_digit(16).unwrap())
        .for_each(|x| b.push_str(format!("{:04b}", x).as_str()));
    b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversions() {
        assert_eq!(hex_to_binary("0".to_string()), "0000");
        assert_eq!(hex_to_binary("1".to_string()), "0001");
        assert_eq!(hex_to_binary("e".to_string()), "1110");
    }

    #[test]
    fn test_total() {
        assert_eq!(get_answer("flqrgnkx"), Some(1242));
        assert_eq!(
            get_answer(include_str!("../assets/day14_input.txt")),
            Some(1128)
        );
    }
}
