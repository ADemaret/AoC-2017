use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 23 - Part 1 --");
    let now = Instant::now();

    //let input = include_str!("../assets/day23_input_demo1.txt");
    let input = include_str!("../assets/day23_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, Default, Clone)]
struct Instructions {
    action: String,
    reg1: char,
    reg2: char,
    val1: Option<isize>,
    val2: Option<isize>,
}

fn get_answer(input: &str) -> Option<usize> {
    // parse
    let instructions = parse(input);

    let mut reg = HashMap::new();
    for instr in &instructions {
        reg.insert(instr.reg1, 0_isize);
    }

    let mut i = 0_isize;
    let mut total = 0;
    loop {
        one_step(&instructions, &mut reg, &mut i, &mut total);
        if i < 0 || i >= instructions.len() as isize {
            break;
        }
    }
    Some(total)
}

fn one_step(
    instructions: &[Instructions],
    reg: &mut HashMap<char, isize>,
    i: &mut isize,
    total: &mut usize,
) {
    let instr = instructions[*i as usize].clone();
    match instr.action.as_str() {
        // set X Y sets register X to the value of Y.
        "set" => {
            reg.insert(instr.reg1, get_val(2, &instr, reg));
            *i += 1;
        }
        // add X Y increases register X by the value of Y.
        "sub" => {
            reg.insert(
                instr.reg1,
                get_val(1, &instr, reg) - get_val(2, &instr, reg),
            );
            *i += 1;
        }
        // mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
        "mul" => {
            reg.insert(
                instr.reg1,
                get_val(1, &instr, reg) * get_val(2, &instr, reg),
            );
            *total += 1;
            *i += 1;
        }
        // jgz X Y jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
        "jnz" => {
            let current_val = get_val(1, &instr, reg);
            if current_val != 0 {
                *i += get_val(2, &instr, reg);
            } else {
                *i += 1;
            }
        }
        _ => unreachable!(),
    }
}

fn get_val(i: usize, instr: &Instructions, reg: &HashMap<char, isize>) -> isize {
    match i {
        1 => {
            if let Some(value) = instr.val1 {
                value
            } else {
                *reg.get(&instr.reg1).unwrap()
            }
        }
        2 => {
            if let Some(value) = instr.val2 {
                value
            } else {
                *reg.get(&instr.reg2).unwrap()
            }
        }
        _ => unreachable!(),
    }
}

fn parse(input: &str) -> Vec<Instructions> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_whitespace().collect::<Vec<_>>();
            let mut instruction = Instructions {
                action: parts[0].to_string(),
                ..Default::default()
            };
            if let Ok(val1) = parts[1].parse::<isize>() {
                instruction.val1 = Some(val1);
            } else {
                instruction.reg1 = parts[1].chars().next().unwrap();
            }
            if let Ok(val2) = parts[2].parse::<isize>() {
                instruction.val2 = Some(val2);
            } else {
                instruction.reg2 = parts[2].chars().next().unwrap();
            }
            instruction
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day23_input.txt")),
            Some(6724)
        );
    }
}
