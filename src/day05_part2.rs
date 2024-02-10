use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 05 - Part 2 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day05_input_demo1.txt");
    let input = include_str!("../assets/day05_input.txt");

    println!("La réponse est {}", get_answer(input));

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> usize {
    let mut instr = input
        .lines()
        .map(|line| line.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let mut i = 0_isize;
    let mut steps = 0;
    loop {
        if i < 0 || i >= instr.len() as isize {
            break;
        }
        let next = instr[i as usize];
        if next >= 3 {
            instr[i as usize] -= 1;
        } else {
            instr[i as usize] += 1;
        }
        i += next;
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day05_input_demo1.txt")),
            10
        );
        assert_eq!(
            get_answer(include_str!("../assets/day05_input.txt")),
            24315397
        );
    }
}
