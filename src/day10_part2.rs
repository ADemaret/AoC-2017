use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 10 - Part 2 --");
    let now = Instant::now();

    // let input = "";
    // let input = "AoC 2017";
    let input = include_str!("../assets/day10_input.txt");

    let answer = get_answer(input);
    if !answer.is_empty() {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

pub fn get_answer(input: &str) -> String {
    // parse
    let (mut hash_list, lengths) = parse(input);

    let mut current = 0;
    let mut skip = 0;
    for _ in 0..64 {
        for l in &lengths {
            hash(&mut hash_list, &mut current, &mut skip, *l);
        }
    }

    // dense hash
    let mut dense = Vec::new();
    for round in 0..16 {
        let mut val = hash_list[round * 16];
        for i in 1..16 {
            val ^= hash_list[round * 16 + i];
        }
        dense.push(val);
    }
    // println!("{:?}", dense);

    // convert to hexa
    let mut str = "".to_string();
    for d in dense {
        str.push_str(format!("{:02x}", d).as_str());
    }
    str
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    // hash
    let mut hash_list = Vec::new();
    for i in 0..256 {
        hash_list.push(i);
    }

    // input
    let mut lengths = input
        .as_bytes()
        .iter()
        .map(|&x| x as usize)
        .collect::<Vec<usize>>();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

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

fn hash(hash_list: &mut Vec<usize>, current: &mut usize, skip: &mut usize, length: usize) {
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
    *current += length + *skip;
    *current %= hash_list.len();
    // increase
    *skip += 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer(""), "a2582a3a0e66e6e86e3812dcb672a272");
        assert_eq!(get_answer("AoC 2017"), "33efeb34ea91902bb2f59c9920caa6cd");
        assert_eq!(get_answer("1,2,3"), "3efbe78a8d82f29979031a4aa0b16a9d");
        assert_eq!(get_answer("1,2,4"), "63960835bcdc130f0b66d7ff4f6a5a8e");
        assert_eq!(
            get_answer(include_str!("../assets/day10_input.txt")),
            "dc7e7dee710d4c7201ce42713e6b8359"
        );
    }
}
