use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 12 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day12_input_demo1.txt");
    let input = include_str!("../assets/day12_input.txt");

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
    let mut programs = input
        .lines()
        .map(|line| {
            (
                false,
                line.split([' ', ',', '<', '-', '>'])
                    .skip(1)
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    // println!("{:?}", programs);

    // programs that are 0
    programs[0].0 = true;
    // programs that are connected to a node that can reach 0
    loop {
        let mut modified = 0;
        let progs_clone = programs.clone();
        for p in programs.iter_mut() {
            if !p.0 {
                for c in &p.1 {
                    if progs_clone[*c].0 {
                        p.0 = true;
                        modified += 1;
                    }
                }
            }
        }
        if modified == 0 {
            break;
        }
    }
    // println!("{:?}", programs);
    Some(programs.iter().filter(|x| x.0).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo1.txt")),
            Some(6)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input.txt")),
            Some(288)
        );
    }
}
