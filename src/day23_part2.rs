use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 23 - Part 2 --");
    let now = Instant::now();

    // custom answer !!
    // let input = include_str!("../assets/day23_input_demo1.txt");
    // let input = include_str!("../assets/day23_input.txt");

    if let Some(answer) = get_answer() {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

// thanks to the analysis by "Shaw"
// https://github.com/VisionistInc/advent-of-code-2017/blob/master/shaw/23/analysis.txt
// and datio-p1 on Reddit
// https://www.reddit.com/r/adventofcode/comments/7lms6p/comment/drnmlbk

fn get_answer() -> Option<isize> {
    // parse
    let mut h = 0;
    for b in (108400..=125400).step_by(17) {
        let mut f = 1;

        // the "assembly" code simplifies to
        // for d in 2..=b {
        //     for e in 2..=b {
        //         if d * e == b {
        //             f = 0;
        //         }
        //     }
        // }

        // which counts if b is not prime
        for d in 2..(b / 2) {
            if b % d == 0 {
                f = 0;
                break;
            }
        }
        if f == 0 {
            h += 1
        }
    }
    Some(h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(), Some(903));
    }
}
