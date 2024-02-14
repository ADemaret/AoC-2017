use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 13 - Part 2 --");
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
    // parse
    let filters = input
        .lines()
        .map(|line| {
            line.split_once(": ")
                .map(|(d, r)| {
                    let depth = d.parse::<usize>().unwrap();
                    let range = r.parse::<usize>().unwrap();
                    (depth, range)
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut delay = 0;
    loop {
        let mut ok = true;
        for f in &filters {
            if (f.0 + delay) % (2 * f.1 - 2) == 0 {
                //println!("if delay is {delay}, caught by filter {}", f.0);
                ok = false;
                break;
            }
        }
        if ok {
            return Some(delay);
        }
        delay += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day13_input.txt")),
            Some(3897604)
        );
    }
}
