use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 10 - Part 1 --");
    let now = Instant::now();

    // let hash_len = 5;
    // let input = "3, 4, 1, 5";

    let hash_len = 256;
    let input = include_str!("../assets/day10_input.txt");

    if let Some(answer) = get_answer(hash_len, input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

fn get_answer(hash_len: usize, input: &str) -> Option<usize> {
    // parse
    let (mut hash_list, lengths) = parse(hash_len, input);

    // hash
    let mut current = 0;
    for (skip, l) in lengths.iter().enumerate() {
        hash(&mut hash_list, &mut current, skip, *l);
    }
    Some(hash_list[0] * hash_list[1])
}

fn parse(hash_len: usize, input: &str) -> (Vec<usize>, Vec<usize>) {
    // hash
    let mut hash_list = Vec::new();
    for i in 0..hash_len {
        hash_list.push(i);
    }

    // input
    let lengths = input
        .split([',', ' '])
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    // return
    (hash_list, lengths)
}

// fn print_hash(hash_list: &[usize], current: &mut usize, length: usize) {
//     let hash_len = hash_list.len();
//     for (l, i) in hash_list.iter().enumerate() {
//         if l == *current {
//             print!("([{i}] ");
//         } else if l == (*current + length) % hash_len - 1 {
//             print!("{i}) ");
//         } else {
//             print!("{i} ");
//         }
//     }
//     println!();
// }

fn hash(hash_list: &mut Vec<usize>, current: &mut usize, skip: usize, length: usize) {
    // print
    // print_hash(hash_list, current, length);
    // reverse
    let mut to_rev = hash_list.clone();
    to_rev.append(&mut hash_list.clone());
    let mut rev = to_rev
        .iter()
        .skip(*current)
        .copied()
        .take(length)
        .collect::<Vec<usize>>();
    rev.reverse();

    let mut to_keep = hash_list.clone();
    to_keep.append(&mut hash_list.clone());
    let keep = to_keep
        .iter()
        .skip((*current + length) % hash_list.len())
        .copied()
        .take(hash_list.len() - length)
        .collect::<Vec<usize>>();

    let mut new_hash = keep.clone();
    new_hash.append(&mut rev.clone());
    new_hash.append(&mut keep.clone());
    new_hash.append(&mut rev.clone());

    let start = if (*current + length) < hash_list.len() {
        keep.len() - *current
    } else {
        keep.len() + length - (*current + length - hash_list.len())
    };
    let final_hash = new_hash
        .iter()
        .skip(start)
        .take(hash_list.len())
        .copied()
        .collect::<Vec<_>>();
    hash_list.clear();
    *hash_list = final_hash.clone();
    // print
    // print_hash(hash_list, current, length);
    // move
    *current += length + skip;
    *current %= hash_list.len();
    // increase
    // skip in increased in the calling loop
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        let mut hash_list = vec![0, 1, 2, 3, 4];
        let lengths = [3, 4, 1, 5];
        let mut current = 0;
        for (skip, l) in lengths.iter().enumerate() {
            hash(&mut hash_list, &mut current, skip, *l);
            match skip {
                0 => {
                    assert_eq!(hash_list, vec![2, 1, 0, 3, 4]);
                }
                1 => {
                    assert_eq!(hash_list, vec![4, 3, 0, 1, 2]);
                }
                2 => {
                    assert_eq!(hash_list, vec![4, 3, 0, 1, 2]);
                }
                3 => {
                    assert_eq!(hash_list, vec![3, 4, 2, 1, 0]);
                }
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_total() {
        assert_eq!(get_answer(5, "3, 4, 1, 5"), Some(12));
        assert_eq!(
            get_answer(256, include_str!("../assets/day10_input.txt")),
            Some(1935)
        );
    }
}
