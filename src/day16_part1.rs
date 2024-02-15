use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 16 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day16_input_demo1.txt");
    let input = include_str!("../assets/day16_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<String> {
    // let mut programs = vec!['a', 'b', 'c', 'd', 'e'];
    let mut programs = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];
        input.split(',').for_each(|s| match &s[0..1] {
            "s" => {
                let val = &s[1..].parse::<usize>().unwrap();
                let program_len = programs.len();
                let (left, right) = programs.split_at_mut(program_len - *val);
                programs = [right, left].concat();
            }
            "x" => {
                let vals = s
                    .split(['x', '/'])
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect::<Vec<_>>();
                programs.swap(vals[0], vals[1]);
            }
            "p" => {
                let p1 = s.chars().skip(1).take(1).collect::<Vec<_>>();
                let p2 = s.chars().skip(3).take(1).collect::<Vec<_>>();
                let pos1 = programs.iter().position(|&x| x == p1[0]).unwrap();
                let pos2 = programs.iter().position(|&x| x == p2[0]).unwrap();
                programs.swap(pos1, pos2);
            }
            _ => unreachable!(),
        });
    Some(programs.iter().collect::<String>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day16_input.txt")),
            Some("lgpkniodmjacfbeh".to_string())
        );
    }
}
