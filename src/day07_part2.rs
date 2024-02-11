use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 07 - Part 2 --");
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

fn get_answer(input: &str) -> Option<usize> {
    let mut weights = HashMap::new();
    let mut discs = HashMap::new();
    //let mut disc_under = HashMap::new();
    input.lines().for_each(|line| {
        let parts = line
            .split([' ', ',', '(', ')'])
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();
        let mut above = Vec::new();
        for i in 3..parts.len() {
            //disc_under.insert(String::from(parts[i]), parts[0].to_string());
            above.push(parts[i].to_string());
        }
        weights.insert(parts[0].to_string(), parts[1].parse::<usize>().unwrap());
        discs.insert(parts[0].to_string(), above);
    });

    for disc_name in discs.keys() {
        let mut answer = None;
        let wa = get_weights_above(&weights, &discs, disc_name, &mut answer);
        if answer.is_some() {
            return answer;
        } else if answer.is_none() {
            if let Some(diff) = find_different(&weights, &discs, disc_name, &wa) {
                return Some(diff);
            }
        }
    }
    None
}

fn get_weights_above(
    weights: &HashMap<String, usize>,
    discs: &HashMap<String, Vec<String>>,
    disc_name: &String,
    answer: &mut Option<usize>,
) -> Vec<usize> {
    let mut weights_above = Vec::new();
    if let Some(discs_above) = discs.get(disc_name) {
        for disc_above in discs_above {
            let wa = get_weights_above(weights, discs, disc_above, answer);
            let sum_wa = wa.iter().sum::<usize>();
            let wd = weights.get(disc_above).unwrap();
            weights_above.push(*wd + sum_wa);
        }
        if answer.is_none() {
            if let Some(diff) = find_different(weights, discs, disc_name, &weights_above) {
                *answer = Some(diff);
            }
        }
    }
    weights_above
}

fn find_different(
    weights: &HashMap<String, usize>,
    discs: &HashMap<String, Vec<String>>,
    disc_name: &String,
    weights_above: &[usize],
) -> Option<usize> {
    if !weights_above.is_empty() {
        let min = weights_above.iter().min().unwrap();
        let max = weights_above.iter().max().unwrap();
        if min != max {
            let min_count = weights_above.iter().filter(|&x| x == min).count();
            let pos = if min_count == 1 {
                weights_above.iter().position(|x| x == min).unwrap()
            } else {
                weights_above.iter().position(|x| x == max).unwrap()
            };
            let disks_above = discs.get(disc_name).unwrap();
            let wrong_disc = disks_above[pos].to_string();
            return Some(*weights.get(&wrong_disc).unwrap() - max.abs_diff(*min));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day07_input_demo1.txt")),
            Some(60)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day07_input.txt")),
            Some(596)
        );
    }
}
