use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 11 - Part 1 --");
    let now = Instant::now();

    // let input = "se,se,se,n,n,n,n,ne";
    let input = include_str!("../assets/day11_input.txt");

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
    let mut count_hexes = [0_isize; 6];
    input.split(',').for_each(|x| match x {
        "nw" => count_hexes[0] += 1,
        "n" => count_hexes[1] += 1,
        "ne" => count_hexes[2] += 1,
        "sw" => count_hexes[3] += 1,
        "s" => count_hexes[4] += 1,
        "se" => count_hexes[5] += 1,
        _ => unreachable!(),
    });
    let mut count_dirs = [0_isize; 3];
    count_dirs[0] = count_hexes[0] - count_hexes[5]; // nw & se
    count_dirs[1] = count_hexes[1] - count_hexes[4]; // n & s
    count_dirs[2] = count_hexes[2] - count_hexes[3]; // ne & sw

    //println!("{:?} => {:?}", input, count_dirs);
    simplify(&mut count_dirs);

    Some(count_dirs.iter().map(|&x| x.unsigned_abs()).sum())
}

fn simplify(count_dirs: &mut [isize; 3]) {
    // group nw & ne (0 & 2)
    if count_dirs[0] > 0 && count_dirs[2] > 0 {
        if count_dirs[0] >= count_dirs[2] {
            // println!("  simplify nw & ne");
            count_dirs[0] -= count_dirs[2];
            count_dirs[1] += count_dirs[2];
            count_dirs[2] -= count_dirs[2];
            // println!("  => {:?}", count_dirs);
        } else {
            // println!("  simplify ne & nw");
            count_dirs[2] -= count_dirs[0];
            count_dirs[1] += count_dirs[0];
            count_dirs[0] -= count_dirs[0];
            // println!("  => {:?}", count_dirs);
        }
    }
    // group sw & se (0 & 2)
    if count_dirs[0] < 0 && count_dirs[2] < 0 {
        if -count_dirs[0] >= -count_dirs[2] {
            // println!("  simplify sw & se");
            count_dirs[0] -= count_dirs[2];
            count_dirs[1] += count_dirs[2];
            count_dirs[2] -= count_dirs[2];
            // println!("  => {:?}", count_dirs);
        } else {
            // println!("  simplify se & sw");
            count_dirs[2] -= count_dirs[0];
            count_dirs[1] += count_dirs[0];
            count_dirs[0] -= count_dirs[0];
            // println!("  => {:?}", count_dirs);
        }
    }
    // group nw & s (0 & 1)
    if count_dirs[0] > 0 && count_dirs[1] < 0 {
        if count_dirs[0] >= -count_dirs[1] {
            // println!("  simplify nw & s");
            count_dirs[0] += count_dirs[1];
            count_dirs[2] += count_dirs[1];
            count_dirs[1] -= count_dirs[1];
            // println!("  => {:?}", count_dirs);
        } else {
            // println!("  simplify s & nw");
            count_dirs[1] -= count_dirs[0];
            count_dirs[2] += count_dirs[0];
            count_dirs[0] += count_dirs[0];
            // println!("  => {:?}", count_dirs);
        }
    }
    // group ne & s (2 & 1)
    if count_dirs[2] > 0 && count_dirs[1] < 0 {
        if count_dirs[2] >= -count_dirs[1] {
            // println!("  simplify ne & s");
            count_dirs[2] += count_dirs[1];
            count_dirs[0] += count_dirs[1];
            count_dirs[1] -= count_dirs[1];
            // println!("  => {:?}", count_dirs);
        } else {
            // println!("  simplify s & ne");
            count_dirs[1] -= count_dirs[2];
            count_dirs[0] += count_dirs[2];
            count_dirs[2] += count_dirs[2];
            // println!("  => {:?}", count_dirs);
        }
    }
    // group sw & n (2 & 1)
    if count_dirs[2] < 0 && count_dirs[1] > 0 {
        if -count_dirs[2] >= count_dirs[1] {
            // println!("  simplify sw & n");
            count_dirs[2] += count_dirs[1];
            count_dirs[0] += count_dirs[1];
            count_dirs[1] -= count_dirs[1];
            // println!("  => {:?}", count_dirs);
        } else {
            // println!("  simplify n & sw");
            count_dirs[1] -= count_dirs[2];
            count_dirs[0] += count_dirs[2];
            count_dirs[2] += count_dirs[2];
            // println!("  => {:?}", count_dirs);
        }
    }
    // group se & n (0 & 1)
    if count_dirs[0] < 0 && count_dirs[1] > 0 {
        if -count_dirs[0] >= count_dirs[1] {
            // println!("  simplify se & n");
            count_dirs[0] += count_dirs[1];
            count_dirs[2] += count_dirs[1];
            count_dirs[1] -= count_dirs[1];
            // println!("  => {:?}", count_dirs);
        } else {
            // println!("  simplify n & se");
            count_dirs[1] += count_dirs[0];
            count_dirs[2] -= count_dirs[0];
            count_dirs[0] -= count_dirs[0];
            // println!("  => {:?}", count_dirs);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(get_answer("ne,nw"), Some(1));
        assert_eq!(get_answer("se,sw"), Some(1));
        assert_eq!(get_answer("nw,s"), Some(1));
        assert_eq!(get_answer("ne,s"), Some(1));
        assert_eq!(get_answer("sw,n"), Some(1));
        assert_eq!(get_answer("se,n"), Some(1));
        assert_eq!(get_answer("ne,ne,ne"), Some(3));
        assert_eq!(get_answer("ne,ne,sw,sw"), Some(0));
        assert_eq!(get_answer("ne,ne,s,s"), Some(2));
        assert_eq!(get_answer("se,sw,se,sw,sw"), Some(3));
        assert_eq!(
            get_answer(include_str!("../assets/day11_input.txt")),
            Some(664)
        );
    }
}
