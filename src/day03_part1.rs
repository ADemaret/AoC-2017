use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 03 - Part 1 --");
    let now = Instant::now();

    let input = include_str!("../assets/day03_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    let value = input.parse::<usize>().unwrap();
    let mut start = 1;
    // println!("1 is at (0,0)");
    start += 1;
    let mut spiral = 1;
    while start < value {
        if let Some(result) = next_square(&mut start, spiral, value) {
            return result;
        }
        spiral += 1;
    }
    0
}

fn next_square(start: &mut usize, x: isize, value: usize) -> Option<usize> {
    // up
    for line in (-x..=(x - 1)).rev() {
        // println!("{} is at ({line},{x})", start);
        if value == *start {
            return Some(line.unsigned_abs() + x as usize);
        }
        *start += 1;
    }
    // left
    for col in (-x..=(x - 1)).rev() {
        // println!("{} is at ({},{col})", start, -x);
        if value == *start {
            return Some(x as usize + col.unsigned_abs());
        }
        *start += 1;
    }
    // down
    for line in (-x + 1)..=x {
        // println!("{} is at ({line},{})", start, -x);
        if value == *start {
            return Some(line.unsigned_abs() + x as usize);
        }
        *start += 1;
    }
    // right
    for col in (-x + 1)..=x {
        // println!("{} is at ({},{col})", start, x);
        if value == *start {
            return Some(x as usize + col.unsigned_abs());
        }
        *start += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("1"), 0);
        assert_eq!(get_answer("12"), 3);
        assert_eq!(get_answer("23"), 2);
        assert_eq!(get_answer("27"), 4);
        assert_eq!(get_answer("1024"), 31);
        assert_eq!(get_answer(include_str!("../assets/day03_input.txt")), 552);
    }
}
