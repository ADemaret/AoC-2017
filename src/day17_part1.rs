use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 17 - Part 1 --");
    let now = Instant::now();

    // let input = "3";
    let input = include_str!("../assets/day17_input.txt");

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
    let steps = input.parse::<usize>().unwrap();

    let mut buffer = vec![0];
    let mut insert = 1;
    let mut pos = 0;
    for _ in 0..2017 {
        let len = buffer.len();
        pos = (pos + steps) % len + 1;
        buffer.insert(pos, insert);
        insert += 1;
    }
    Some(buffer[pos + 1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("3"), Some(638));
        assert_eq!(
            get_answer(include_str!("../assets/day17_input.txt")),
            Some(419)
        );
    }
}
