use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 01 - Part 1 --");
    let now = Instant::now();

    let input = include_str!("../assets/day01_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    let input0 = &input;
    let input1 = format!("{}{}", &input[1..], &input[..1]);
    let input0_chars = input0.chars();
    let input1_chars = input1.chars();
    input0_chars
        .zip(input1_chars)
        .filter(|(x, y)| x == y)
        .map(|(x, _)| x.to_digit(10).unwrap() as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("1122"), 3);
        assert_eq!(get_answer("1111"), 4);
        assert_eq!(get_answer("1234"), 0);
        assert_eq!(get_answer("91212129"), 9);

        assert_eq!(get_answer(include_str!("../assets/day01_input.txt")), 1047);
    }
}
