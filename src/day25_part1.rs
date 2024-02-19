use std::{collections::HashMap, time::Instant};

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 25 - Part 1 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day25_input_demo1.txt");
    let input = include_str!("../assets/day25_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, Clone)]
struct OneRule {
    write: usize,
    dir: isize,
    goto: usize,
}

#[derive(Debug, Clone)]
struct Rules {
    rule_if_0: OneRule,
    rule_if_1: OneRule,
}

fn get_answer(input: &str) -> Option<usize> {
    // parse
    let mut steps = 0;
    let mut rules = Vec::new();
    input.split("\n\n").enumerate().for_each(|(block, txt)| {
        let mut state = 0;
        let mut if_val = 0;
        let mut write = 0;
        let mut dir = 0;
        let mut goto = 0;
        let mut if_val2 = 0;
        let mut write2 = 0;
        let mut dir2 = 0;
        let mut goto2 = 0;
        txt.lines().enumerate().for_each(|(l, i2)| {
            if block == 0 && l == 1 {
                steps = i2
                    .split(' ')
                    .filter_map(|x| x.parse::<usize>().ok())
                    .collect::<Vec<_>>()[0];
            } else if block > 0 {
                match l {
                    0 => {
                        let ch = i2[(i2.len() - 2)..(i2.len() - 1)].chars().next().unwrap();
                        state = ch as u32 - 'A' as u32;
                    }
                    1 => {
                        if_val = i2
                            .split([' ', ':'])
                            .filter_map(|x| x.parse::<usize>().ok())
                            .collect::<Vec<_>>()[0];
                    }
                    2 => {
                        write = i2
                            .split([' ', '.'])
                            .filter_map(|x| x.parse::<usize>().ok())
                            .collect::<Vec<_>>()[0];
                    }
                    3 => {
                        let (_, dir0) = i2.split_once("to the ").unwrap();
                        if dir0 == "right." {
                            dir = 1;
                        }
                        if dir0 == "left." {
                            dir = -1;
                        }
                    }
                    4 => {
                        let ch = i2[(i2.len() - 2)..(i2.len() - 1)].chars().next().unwrap();
                        goto = ch as u32 - 'A' as u32;
                    }
                    5 => {
                        if_val2 = i2
                            .split([' ', ':'])
                            .filter_map(|x| x.parse::<usize>().ok())
                            .collect::<Vec<_>>()[0];
                    }
                    6 => {
                        write2 = i2
                            .split([' ', '.'])
                            .filter_map(|x| x.parse::<usize>().ok())
                            .collect::<Vec<_>>()[0];
                    }
                    7 => {
                        let (_, dir0) = i2.split_once("to the ").unwrap();
                        if dir0 == "right." {
                            dir2 = 1;
                        }
                        if dir0 == "left." {
                            dir2 = -1;
                        }
                    }
                    8 => {
                        let ch = i2[(i2.len() - 2)..(i2.len() - 1)].chars().next().unwrap();
                        goto2 = ch as u32 - 'A' as u32;
                    }
                    _ => {}
                }
            }
        });
        let rule1 = OneRule {
            write,
            dir,
            goto: goto as usize,
        };
        let rule2 = OneRule {
            write: write2,
            dir: dir2,
            goto: goto2 as usize,
        };

        rules.insert(
            state as usize,
            Rules {
                rule_if_0: rule1,
                rule_if_1: rule2,
            },
        );
    });

    let mut tape = HashMap::new();
    tape.insert(0, 0);
    let mut current = 0;
    let mut state = 0;
    for _ in 0..steps {
        let rule = if *tape.get(&current).unwrap_or(&0) == 0 {
            rules[state].rule_if_0.clone()
        } else {
            rules[state].rule_if_1.clone()
        };
        tape.insert(current, rule.write);
        current += rule.dir;
        state = rule.goto;
    }
    Some(tape.iter().filter(|(_, &x)| x == 1).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day25_input_demo1.txt")),
            Some(3)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day25_input.txt")),
            Some(2474)
        );
    }
}
