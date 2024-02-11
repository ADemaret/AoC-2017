use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 09 - Part 1 --");
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
    let stream = clean_stream(input);
    Some(get_score(&stream)) //, 0, 0))
}

fn get_score(stream: &[u8]) -> usize {
    let mut i = 0;
    let mut score = 0;
    let mut depth = 0;
    loop {
        match stream[i] {
            b'{' => {
                depth += 1;
            }
            b'}' => {
                score += depth;
                depth -= 1;
            }
            _ => {}
        }
        i += 1;
        if i >= stream.len() {
            break;
        }
    }
    score
}

fn clean_stream(input: &str) -> Vec<u8> {
    let input_chars = input.as_bytes();
    let mut clean_stream = Vec::new();
    let mut i = 0;
    loop {
        match input_chars[i] {
            b'<' => {
                if let Some(skip) = skip_garbage(input, i) {
                    i = skip;
                }
            }
            b'{' => {
                clean_stream.push(b'{');
            }
            b'}' => {
                clean_stream.push(b'}');
            }
            _ => {}
        }
        i += 1;
        if i >= input.len() {
            break;
        }
    }
    clean_stream
}

///
/// returns the index of the first char after the garbage
///
fn skip_garbage(input: &str, start: usize) -> Option<usize> {
    let mut in_garbage = false;
    let input_chars = input.as_bytes();
    let mut i = start;
    loop {
        match input_chars[i] {
            b'<' => in_garbage = true,
            b'!' => i += 1,
            b'>' => {
                if in_garbage {
                    return Some(i);
                }
            }
            _ => {}
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
    fn test_garbage() {
        assert_eq!(skip_garbage("<>", 0), Some(1));
        assert_eq!(skip_garbage("<random characters>", 0), Some(18));
        assert_eq!(skip_garbage("<<<<>", 0), Some(4));
        assert_eq!(skip_garbage("<{!>}>", 0), Some(5));
        assert_eq!(skip_garbage("<!!>", 0), Some(3));
        assert_eq!(skip_garbage("<!!!>>", 0), Some(5));
        assert_eq!(skip_garbage("<{o\"i!a,<{i<a>", 0), Some(13));
    }

    #[test]
    fn test_clean_stream() {
        assert_eq!(clean_stream("{}"), "{}".as_bytes());
        assert_eq!(clean_stream("{{{}}}"), "{{{}}}".as_bytes());
        assert_eq!(clean_stream("{{},{}}"), "{{}{}}".as_bytes());
        assert_eq!(clean_stream("{{{},{},{{}}}}"), "{{{}{}{{}}}}".as_bytes());
        assert_eq!(clean_stream("{<a>,<a>,<a>,<a>}"), "{}".as_bytes());
        assert_eq!(
            clean_stream("{{<ab>},{<ab>},{<ab>},{<ab>}}"),
            "{{}{}{}{}}".as_bytes()
        );
        assert_eq!(
            clean_stream("{{<!!>},{<!!>},{<!!>},{<!!>}}"),
            "{{}{}{}{}}".as_bytes()
        );
        assert_eq!(
            clean_stream("{{<a!>},{<a!>},{<a!>},{<ab>}}"),
            "{{}}".as_bytes()
        );
    }

    #[test]
    fn test_total() {
        assert_eq!(get_answer("{}"), Some(1));
        assert_eq!(get_answer("{{{}}}"), Some(6));
        assert_eq!(get_answer("{{},{}}"), Some(5));
        assert_eq!(get_answer("{{{},{},{{}}}}"), Some(16));
        assert_eq!(get_answer("{<a>,<a>,<a>,<a>}"), Some(1));
        assert_eq!(get_answer("{{<ab>},{<ab>},{<ab>},{<ab>}}"), Some(9));
        assert_eq!(get_answer("{{<!!>},{<!!>},{<!!>},{<!!>}}"), Some(9));
        assert_eq!(get_answer("{{<a!>},{<a!>},{<a!>},{<ab>}}"), Some(3));
        assert_eq!(
            get_answer(include_str!("../assets/day09_input.txt")),
            Some(21037)
        );
    }
}
