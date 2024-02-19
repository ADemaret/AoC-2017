use std::{cmp::Ordering, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 24 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day24_input_demo1.txt");
    let input = include_str!("../assets/day24_input.txt");

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
    let mut components = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let (s, e) = line
                .split_once('/')
                .map(|(s, e)| (s.parse::<usize>().unwrap(), e.parse::<usize>().unwrap()))
                .unwrap();
            (i, s, e)
        })
        .collect::<Vec<_>>();
    // test if there are duplicate components => no => ok
    for (i1, s1, e1) in components.iter() {
        for (i2, s2, e2) in components.iter() {
            if ((s1 == s2 && e1 == e2) || (s1 == e2 && e1 == s2)) && i1 != i2 {
                println!("line {} and line{} are the same : {}/{}", i1, i2, s1, e1);
                panic!();
            }
        }
    }
    // we add reverse components with same line number
    let nbr_components = components.len();
    for l in 0..nbr_components {
        components.push((components[l].0, components[l].2, components[l].1));
    }
    // dfs to get the heavier bridge
    let start = 0;
    let mut dejavu = vec![false; components.len()];
    let mut max_l = 0;
    let mut max_w = 0;
    depth_first_search(
        &components,
        start,
        &mut dejavu,
        0,
        &mut max_l,
        0,
        &mut max_w,
    );
    Some(max_w)
}

fn depth_first_search(
    components: &Vec<(usize, usize, usize)>,
    current: usize,
    dejavu: &mut Vec<bool>,
    length: usize,
    max_l: &mut usize,
    weight: usize,
    max_w: &mut usize,
) {
    //
    let mut possible_components = Vec::new();
    for c in components.iter() {
        if c.1 == current && !dejavu[c.0] {
            possible_components.push(c);
        }
    }
    // end
    if possible_components.is_empty() {
        match length.cmp(max_l) {
            Ordering::Greater => {
                *max_l = length;
                *max_w = weight;
            }
            Ordering::Less => {}
            Ordering::Equal => {
                *max_w = weight.max(*max_w);
            }
        }
        // println!("lenght = {length}, weight:{weight}");
        return;
    }

    for node in possible_components {
        if !dejavu[node.0] {
            dejavu[node.0] = true;
            depth_first_search(
                components,
                node.2,
                dejavu,
                length + 1,
                max_l,
                weight + node.1 + node.2,
                max_w,
            );
            dejavu[node.0] = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day24_input_demo1.txt")),
            Some(19)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day24_input.txt")),
            Some(1824)
        );
    }
}
