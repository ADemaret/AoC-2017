use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 04 - Part 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day04_input_demo1.txt");
    let input = include_str!("../assets/day04_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let mut parts = line
                .split_whitespace()
                .map(|part| {
                    let mut part = part.chars().collect::<Vec<_>>();
                    part.sort();
                    part
                })
                .collect::<Vec<_>>();
            parts.sort();
            let mut valid = true;
            for i in 0..parts.len() - 1 {
                if parts[i] == parts[i + 1] {
                    valid = false;
                    break;
                }
            }
            if valid {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("abcde fghij"), 1);
        assert_eq!(get_answer("abcde xyz ecdab"), 0);
        assert_eq!(get_answer("a ab abc abd abf abj"), 1);
        assert_eq!(get_answer("iiii oiii ooii oooi oooo"), 1);
        assert_eq!(get_answer("oiii ioii iioi iiio"), 0);
        assert_eq!(get_answer(include_str!("../assets/day04_input.txt")), 223);
    }
}
