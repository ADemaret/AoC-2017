use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day07_input_demo1.txt");
    let input = include_str!("../assets/day07_input.txt");

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
    let mut discs = Vec::new();
    let mut disc_under = HashMap::new();
    input.lines().for_each(|line| {
        let parts = line
            .split([' ', ','])
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        discs.push(parts[0].to_string());
        for i in 3..parts.len() {
            disc_under.insert(String::from(parts[i]), parts[0].to_string());
        }
    });

    // find the element that is not in any "above" vec
    return discs.into_iter().find(|d| disc_under.get(d).is_none());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day07_input_demo1.txt")),
            Some("tknk".to_string())
        );
        assert_eq!(
            get_answer(include_str!("../assets/day07_input.txt")),
            Some("uownj".to_string())
        );
    }
}
