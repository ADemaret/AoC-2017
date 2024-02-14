use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 13 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day13_input_demo1.txt");
    let input = include_str!("../assets/day13_input.txt");

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
    let severity = input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(d, r)| {
                    let depth = d.parse::<usize>().unwrap();
                    let range = r.parse::<usize>().unwrap();
                    if depth % (2 * range - 2) == 0 {
                        // println!("ok  : depth :{}, range: {}", depth, range);
                        depth * range
                    } else {
                        // println!("NOT : depth :{}, range: {}", depth, range);
                        0
                    }
                })
                .unwrap()
        })
        // .inspect(|x| println!("{x}"))
        .sum::<usize>();

    Some(severity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_ne!(get_answer("2: 2"), None);
        assert_ne!(get_answer("4: 2"), None);
        assert_ne!(get_answer("6: 2"), None);
        assert_ne!(get_answer("4: 3"), None);
        assert_ne!(get_answer("8: 3"), None);
        assert_ne!(get_answer("8: 5"), None);
        assert_eq!(
            get_answer(include_str!("../assets/day13_input_demo1.txt")),
            Some(24)
        );
        // assert_eq!(get_answer(include_str!("../assets/day13_input.txt")), None);
    }
}
