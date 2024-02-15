use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 16 - Part 2 --");
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

#[derive(Debug, Default)]
struct Instruction {
    instr: char,
    val1: usize,
    val2: usize,
    ch1: char,
    ch2: char,
}

fn get_answer(input: &str) -> Option<String> {
    // let mut programs = vec!['a', 'b', 'c', 'd', 'e'];
    let programs = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
    ];

    // parse
    let instructions = input
        .split(',')
        .map(|s| match &s[0..1] {
            "s" => {
                let val = &s[1..].parse::<usize>().unwrap();
                Instruction {
                    instr: 's',
                    val1: programs.len() - *val,
                    ..Default::default()
                }
            }
            "x" => {
                let vals = s
                    .split(['x', '/'])
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect::<Vec<_>>();
                Instruction {
                    instr: 'x',
                    val1: vals[0],
                    val2: vals[1],
                    ..Default::default()
                }
            }
            "p" => {
                let p1 = s.chars().skip(1).take(1).collect::<Vec<_>>();
                let p2 = s.chars().skip(3).take(1).collect::<Vec<_>>();
                Instruction {
                    instr: 'p',
                    ch1: p1[0],
                    ch2: p2[0],
                    ..Default::default()
                }
            }
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    //

    let mut cycle = 1;
    let mut tortue = programs.clone();
    let mut lievre = programs.clone();
    for i in 1..1000 {
        multiple_danse(&instructions, &mut tortue, 1);
        multiple_danse(&instructions, &mut lievre, 2);
        // comparaison
        if tortue == lievre {
            cycle = i;
            break;
        }
    }
    // println!("cycle found !! : {}", cycle);
    let remaing_cycles = 1_000_000_000 % cycle;
    multiple_danse(&instructions, &mut tortue, remaing_cycles);

    Some(tortue.iter().collect::<String>())
}

fn multiple_danse(instructions: &Vec<Instruction>, programs: &mut Vec<char>, count: usize) {
    for _ in 0..count {
        one_danse(instructions, programs);
    }
}

fn one_danse(instructions: &Vec<Instruction>, programs: &mut Vec<char>) {
    for instr in instructions {
        match instr.instr {
            's' => {
                let (left, right) = programs.split_at_mut(instr.val1);
                *programs = [right, left].concat();
            }
            'x' => {
                programs.swap(instr.val1, instr.val2);
            }
            'p' => {
                let pos1 = programs.iter().position(|&x| x == instr.ch1).unwrap();
                let pos2 = programs.iter().position(|&x| x == instr.ch2).unwrap();
                programs.swap(pos1, pos2);
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day16_input.txt")),
            Some("hklecbpnjigoafmd".to_string())
        );
    }
}
