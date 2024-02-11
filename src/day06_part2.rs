use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 06 - Part 2 --");
    let now = Instant::now();

    let input = include_str!("../assets/day06_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    let mut banks = input
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let mut dejavu = vec![banks.clone()];
    let mut counter = 0;
    let mut seen = 0;
    let mut first_seen = Vec::new();
    loop {
        redistribute(&mut banks);
        counter += 1;
        if seen == 1 && banks == first_seen {
            break;
        }
        if seen != 1 && dejavu.contains(&banks) {
            seen = 1;
            first_seen = banks.clone();
            counter = 0;
        }
        dejavu.push(banks.clone());
    }
    counter
}

fn redistribute(banks: &mut Vec<usize>) {
    let mut max = *banks.iter().max().unwrap() as isize;
    let mut found = false;
    let mut new_banks = banks.clone();
    for (i, b) in banks.iter().enumerate().cycle() {
        if !found && *b as isize == max {
            found = true;
            new_banks[i] = 0;
            continue;
        }
        if found && max > 0 {
            new_banks[i] += 1;
            max -= 1;
        }
        if max == 0 {
            break;
        }
    }
    *banks = new_banks;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("0 2 7 0"), 4);
        //assert_eq!(get_answer(include_str!("../assets/day06_input.txt")), 7864);
    }
}
