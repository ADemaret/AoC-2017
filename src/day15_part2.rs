use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 15 - Part 2 --");
    let now = Instant::now();

    // let input = "65 8921";
    let input = include_str!("../assets/day15_input.txt");

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
    // parse
    let mut inputs = input
        .split_ascii_whitespace()
        .filter_map(|x| x.parse::<usize>().ok())
        .collect::<Vec<_>>();
    // println!("{:?}", inputs);
    assert_eq!(inputs.len(), 2);

    let mut total = 0;
    for _ in 0..5_000_000 {
        inputs = next_values(&inputs);
        //println!("{:?}", inputs);
        let str1 = &format!("{:032b}", inputs[0])[16..32];
        let str2 = &format!("{:032b}", inputs[1])[16..32];
        // println!("{str1}");
        // println!("{str2}");
        // println!();
        if str1 == str2 {
            total += 1;
        }
    }
    Some(total)
}

fn next_values(inputs: &[usize]) -> Vec<usize> {
    let mut val1 = inputs[0];
    let mut val2 = inputs[1];
    loop {
        val1 = val1 * 16807 % 2147483647;
        if val1 % 4 == 0 {
            break;
        }
    }
    loop {
        val2 = val2 * 48271 % 2147483647;
        if val2 % 8 == 0 {
            break;
        }
    }
    vec![val1, val2]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("65 8921"), Some(309));
        assert_eq!(
            get_answer(include_str!("../assets/day15_input.txt")),
            Some(290)
        );
    }
}
