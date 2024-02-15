use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 17 - Part 2 --");
    let now = Instant::now();

    // let input = "3";
    let input = include_str!("../assets/day17_input.txt");

    if let Some(answer) = get_answer(input, 50000000) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str, max: usize) -> Option<usize> {
    // parse
    let steps = input.parse::<usize>().unwrap();

    let mut pos = 0;
    let mut len = 1;
    let mut answer = 0;
    for i in 0..max {
        pos = (pos + steps) % len + 1;
        if pos == 1 {
            answer = i + 1;
        }
        len += 1;
    }
    Some(answer)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("3", 1), Some(1));
        assert_eq!(get_answer("3", 2), Some(2));
        assert_eq!(get_answer("3", 3), Some(2));
        assert_eq!(get_answer("3", 4), Some(2));
        assert_eq!(get_answer("3", 5), Some(5));
        assert_eq!(get_answer("3", 6), Some(5));
        assert_eq!(get_answer("3", 7), Some(5));
        assert_eq!(get_answer("3", 8), Some(5));
        assert_eq!(get_answer("3", 9), Some(9));
        assert_eq!(get_answer("3", 2017), Some(1226));
        assert_eq!(
            get_answer(include_str!("../assets/day17_input.txt"), 50000000),
            Some(46038988)
        );
    }
}
