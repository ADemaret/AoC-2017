use std::time::Instant;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 20 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day20_input_demo1.txt");
    let input = include_str!("../assets/day20_input.txt");

    if let Some(answer) = get_answer(input) {
        println!("The answer is : {}", answer);
    } else {
        println!("No answer found");
    }

    let elapsed1 = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed1);
}

//

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Particle {
    px: isize,
    py: isize,
    pz: isize,
    id: usize,
    vx: isize,
    vy: isize,
    vz: isize,
    ax: isize,
    ay: isize,
    az: isize,
    md: usize, // manhattan distance
}

fn get_answer(input: &str) -> Option<usize> {
    // parse
    let mut particles = input
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let parts = line
                .split(['p', 'v', 'a', '=', ',', '<', '>', ' '])
                .filter(|&s| !s.is_empty())
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            Particle {
                id: i,
                px: parts[0],
                py: parts[1],
                pz: parts[2],
                vx: parts[3],
                vy: parts[4],
                vz: parts[5],
                ax: parts[6],
                ay: parts[7],
                az: parts[8],
                md: 0,
            }
        })
        .collect::<Vec<_>>();
    // println!("{:#?}", particles);

    // brute force
    for _ in 0..10000 {
        tick(&mut particles);
        particles.sort();
        remove_collided(&mut particles);
    }
    Some(particles.len())
}

fn remove_collided(particles: &mut Vec<Particle>) {
    let mut to_remove = Vec::new();
    for i in 0..particles.len() - 1 {
        if particles[i].px == particles[i + 1].px
            && particles[i].py == particles[i + 1].py
            && particles[i].pz == particles[i + 1].pz
        {
            to_remove.push(i);
            to_remove.push(i + 1);
        }
    }
    to_remove.sort();
    to_remove.dedup();
    to_remove.reverse();
    for i in to_remove {
        particles.remove(i);
    }
    //println!("--");
}

fn tick(particles: &mut Vec<Particle>) {
    for part in particles {
        // Increase the X velocity by the X acceleration.
        part.vx += part.ax;
        // Increase the Y velocity by the Y acceleration.
        part.vy += part.ay;
        // Increase the Z velocity by the Z acceleration.
        part.vz += part.az;
        // Increase the X position by the X velocity.
        part.px += part.vx;
        // Increase the Y position by the Y velocity.
        part.py += part.vy;
        // Increase the Z position by the Z velocity.
        part.pz += part.vz;
        // manhattan distance
        part.md = part.px.unsigned_abs() + part.py.unsigned_abs() + part.pz.unsigned_abs();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day20_input_demo2.txt")),
            Some(1)
        );
        assert_eq!(
            get_answer(include_str!("../assets/day20_input.txt")),
            Some(448)
        );
    }
}
