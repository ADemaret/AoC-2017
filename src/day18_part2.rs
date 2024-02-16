use std::{
    collections::{HashMap, VecDeque},
    time::Instant,
};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 18 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day18_input_demo2.txt");
    let input = include_str!("../assets/day18_input.txt");

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

    let mut reg0 = HashMap::new();
    let mut reg1 = HashMap::new();
    for instr in &instructions {
        reg0.insert(instr.reg1, 0_isize);
        reg1.insert(instr.reg1, 0_isize);
    }
    reg0.insert('p', 0);
    reg1.insert('p', 1);

    // to be read by program0 = written by 1
    let mut queue0: VecDeque<isize> = VecDeque::new();
    // to be read by program1 = written by 0
    let mut queue1: VecDeque<isize> = VecDeque::new();

    // let mut last_sound_played = 0;
    let mut i0 = 0_isize;
    let mut i1 = 0_isize;
    let mut wait0 = false;
    let mut wait1 = false;
    let mut total0 = 0;
    let mut total1 = 0;
    //let mut ret = None;
    loop {
        one_step(
            &instructions,
            &mut reg0,
            &mut i0,
            &mut queue0,
            &mut queue1,
            &mut wait0,
            &mut total0,
        );
        one_step(
            &instructions,
            &mut reg1,
            &mut i1,
            &mut queue1,
            &mut queue0,
            &mut wait1,
            &mut total1,
        );
        if wait0 && wait1 {
            return Some(total1);
        }
    }
}

fn one_step(
    instructions: &[Instructions],
    reg: &mut HashMap<char, isize>,
    i: &mut isize,
    w_queue: &mut VecDeque<isize>,
    r_queue: &mut VecDeque<isize>,
    wait: &mut bool,
    total: &mut usize,
) {
    let instr = instructions[*i as usize].clone();
    match instr.action.as_str() {
        // snd X sends the value of X to the other program
        "snd" => {
            w_queue.push_back(get_val(1, &instr, reg));
            *total += 1;
            *i += 1;
        }
        // set X Y sets register X to the value of Y.
        "set" => {
            reg.insert(instr.reg1, get_val(2, &instr, reg));
            *i += 1;
        }
        // add X Y increases register X by the value of Y.
        "add" => {
            reg.insert(
                instr.reg1,
                get_val(1, &instr, reg) + get_val(2, &instr, reg),
            );
            *i += 1;
        }
        // mul X Y sets register X to the result of multiplying the value contained in register X by the value of Y.
        "mul" => {
            reg.insert(
                instr.reg1,
                get_val(1, &instr, reg) * get_val(2, &instr, reg),
            );
            *i += 1;
        }
        // mod X Y sets register X to the remainder of dividing the value contained in register X by the value of Y (that is, it sets X to the result of X modulo Y).
        "mod" => {
            reg.insert(
                instr.reg1,
                get_val(1, &instr, reg) % get_val(2, &instr, reg),
            );
            *i += 1;
        }
        // rcv X receives the next value and stores it in register X
        "rcv" => {
            if !r_queue.is_empty() {
                let rcv = r_queue.pop_front().unwrap();
                reg.insert(instr.reg1, rcv);
                *i += 1;
                *wait = false;
            } else {
                *wait = true;
            }
        }
        // jgz X Y jumps with an offset of the value of Y, but only if the value of X is greater than zero. (An offset of 2 skips the next instruction, an offset of -1 jumps to the previous instruction, and so on.)
        "jgz" => {
            let current_val = get_val(1, &instr, reg);
            if current_val > 0 {
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
            if !["snd", "snd", "rcv"].contains(&parts[0]) {
                if let Ok(val2) = parts[2].parse::<isize>() {
                    instruction.val2 = Some(val2);
                } else {
                    instruction.reg2 = parts[2].chars().next().unwrap();
                }
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
            get_answer(include_str!("../assets/day18_input_demo2.txt")),
            Some(3)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day18_input.txt")),
            Some(7493)
        );
    }
}
