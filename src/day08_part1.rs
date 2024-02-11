use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 08 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day08_input_demo1.txt");
    let input = include_str!("../assets/day08_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(input: &str) -> Option<isize> {
    let instructions = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // first get all register names
    let mut registers = HashMap::new();
    for instr in &instructions {
        registers.insert(instr[0], 0);
    }

    for instr in &instructions {
        let org_reg = registers.get(&instr[0]).unwrap();
        let dx = instr[2].parse::<isize>().unwrap();
        let signed_dx = if instr[1] == "inc" { dx } else { -dx };
        let reg_val = registers.get(&instr[4]).unwrap();
        let val = instr[6].parse::<isize>().unwrap();
        let mut test = false;
        if (instr[5] == "<" && *reg_val < val)
            || (instr[5] == "<=" && *reg_val <= val)
            || (instr[5] == ">" && *reg_val > val)
            || (instr[5] == ">=" && *reg_val >= val)
            || (instr[5] == "==" && *reg_val == val)
            || (instr[5] == "!=" && *reg_val != val)
        {
            test = true
        }
        if test {
            registers.insert(instr[0], *org_reg + signed_dx);
        }
    }
    registers.iter().map(|(_, &b)| b).max()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day08_input_demo1.txt")),
            Some(1)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day08_input.txt")),
            Some(4832)
        );
    }
}
