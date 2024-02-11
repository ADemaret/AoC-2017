use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 09 - Part 2 --");
    let now = Instant::now();

    let input = include_str!("../assets/day09_input.txt");

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
    count_garbage(input)
}

fn count_garbage(input: &str) -> Option<usize> {
    let input_chars = input.as_bytes();
    let mut garbage = 0;
    let mut i = 0;
    loop {
        if input_chars[i] == b'<' {
            if let Some((skip, count)) = count_in_garbage(input, i) {
                i = skip;
                garbage += count;
            }
        }
        i += 1;
        if i >= input.len() {
            break;
        }
    }
    Some(garbage)
}

///
/// count the garbage chars in the garbage part
///
fn count_in_garbage(input: &str, start: usize) -> Option<(usize, usize)> {
    let mut count = 0;
    let mut in_garbage = false;
    let input_chars = input.as_bytes();
    let mut i = start;
    loop {
        match input_chars[i] {
            b'<' => {
                if !in_garbage {
                    in_garbage = true;
                } else {
                    count += 1;
                }
            }
            b'!' => i += 1,
            b'>' => {
                if in_garbage {
                    return Some((i, count));
                }
            }
            _ => {
                count += 1;
            }
        }
        i += 1;
        if i == input.len() {
            break;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_answer() {
        assert_eq!(get_answer("<>"), Some(0));
        assert_eq!(get_answer("<random characters>"), Some(17));
        assert_eq!(get_answer("<<<<>"), Some(3));
        assert_eq!(get_answer("<{!>}>"), Some(2));
        assert_eq!(get_answer("<!!>"), Some(0));
        assert_eq!(get_answer("<!!!>>"), Some(0));
        assert_eq!(get_answer("<{o\"i!a,<{i<a>"), Some(10));
        assert_eq!(
            get_answer(include_str!("../assets/day09_input.txt")),
            Some(9495)
        );
    }
}
