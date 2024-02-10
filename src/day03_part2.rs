use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 03 - Part 2 --");
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
    let mut values = HashMap::new();
    // println!("1 is at (0,0)");
    values.insert((0, 0), 1);
    let mut spiral = 1;
    while start <= value {
        next_square(&mut start, spiral, value, &mut values);
        spiral += 1;
    }
    start
}

fn get_val(values: &mut HashMap<(isize, isize), usize>, line: isize, col: isize) -> usize {
    let mut val = 0;
    for l in line - 1..=line + 1 {
        for c in col - 1..=col + 1 {
            if let Some(add) = values.get(&(l, c)) {
                val += add;
            }
        }
    }
    val
}

fn next_square(
    start: &mut usize,
    x: isize,
    value: usize,
    values: &mut HashMap<(isize, isize), usize>,
) -> usize {
    // up
    for line in (-x..=(x - 1)).rev() {
        *start = get_val(values, line, x);
        // println!("{} is at ({line},{x})", start);
        values.insert((line, x), *start);
        if value < *start {
            return *start;
        }
        *start += 1;
    }
    // left
    for col in (-x..=(x - 1)).rev() {
        *start = get_val(values, -x, col);
        // println!("{} is at ({},{col})", start, -x);
        values.insert((-x, col), *start);
        if value < *start {
            return *start;
        }
        *start += 1;
    }
    // down
    for line in (-x + 1)..=x {
        *start = get_val(values, line, -x);
        // println!("{} is at ({line},{})", start, -x);
        values.insert((line, -x), *start);
        if value < *start {
            return *start;
        }
        *start += 1;
    }
    // right
    for col in (-x + 1)..=x {
        *start = get_val(values, x, col);
        // println!("{} is at ({},{col})", start, x);
        values.insert((x, col), *start);
        if value < *start {
            return *start;
        }
        *start += 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day03_input.txt")),
            330785
        );
    }
}
