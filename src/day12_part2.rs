use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 12 - Part 2 --");
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
                None,
                line.split([' ', ',', '<', '-', '>'])
                    .skip(1)
                    .filter(|x| !x.is_empty())
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    // println!("{:?}", programs);

    for node_id in 0..programs.len() {
        set_group(&mut programs, node_id);
    }
    programs.sort_by_key(|x| x.0);
    programs.dedup_by_key(|x| x.0);
    Some(programs.len())
}

fn set_group(programs: &mut Vec<(Option<usize>, Vec<usize>)>, node_id: usize) {
    if programs[node_id].0.is_none() {
        // programs that are 0
        programs[node_id].0 = Some(node_id);
        // programs that are connected to a node that can reach 0
        loop {
            let mut modified = 0;
            let progs_clone = programs.clone();
            for p in programs.iter_mut() {
                if p.0.is_none() {
                    for c in &p.1 {
                        if progs_clone[*c].0 == Some(node_id) {
                            p.0 = Some(node_id);
                            modified += 1;
                        }
                    }
                }
            }
            if modified == 0 {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day12_input_demo1.txt")),
            Some(2)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day12_input.txt")),
            Some(211)
        );
    }
}
