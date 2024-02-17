use std::time::Instant;

use crate::utils::grid2d::Grid2D;

// personal functions
//use crate::utils::grid2d;
//use crate::utils::pause;
//use crate::utils::math;

pub fn main() {
    println!("-- Advent of Code - Day 21 - Part 2 --");
    let now = Instant::now();

    // let input = include_str!("../assets/day21_input_demo1.txt");
    let input = include_str!("../assets/day21_input.txt");

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
    // parse
    let mut swaps = input
        .lines()
        .map(|line| line.split_once(" => ").unwrap())
        .map(|(l, r)| {
            let l2 = l
                .chars()
                .map(|c| if c == '/' { '\n' } else { c })
                .collect::<String>();
            let left = Grid2D::new(l2.as_str());
            // left.print();
            let r2 = r
                .chars()
                .map(|c| if c == '/' { '\n' } else { c })
                .collect::<String>();
            let right = Grid2D::new(r2.as_str());
            // right.print();
            (left, right)
        })
        .collect::<Vec<_>>();
    //println!("{:#?}", swaps);

    let mut new_swaps = Vec::new();
    for s in &swaps {
        add_swaps_variations(s, &mut new_swaps);
    }
    swaps.append(&mut new_swaps);
    swaps.sort();
    // println!("---- swaps : ");
    // for s in &swaps {
    //     s.0.print();
    // }
    // println!("----");

    let mut grid = Grid2D::new(".#.\n..#\n###");
    // grid.print();

    for _ in 0..18 {
        let mut nbr_parts = 0;
        if grid.max_c % 2 == 0 {
            nbr_parts = grid.max_c / 2;
        } else if grid.max_c % 3 == 0 {
            nbr_parts = grid.max_c / 3;
        }
        let size_parts = grid.max_c / nbr_parts;
        let mut new_big_grid = Grid2D::new_by_sizes(
            nbr_parts * (size_parts + 1),
            nbr_parts * (size_parts + 1),
            '.',
        );
        // divide in squares
        for sub_l in 0..nbr_parts {
            for sub_c in 0..nbr_parts {
                // get sub_grid
                let sub_grid = grid.get_portion(
                    sub_l * size_parts,
                    (sub_l + 1) * size_parts,
                    sub_c * size_parts,
                    (sub_c + 1) * size_parts,
                );
                // println!("the subgrid");
                // sub_grid.print();
                // find swap
                let mut found = false;
                for s in &swaps {
                    if sub_grid == s.0 {
                        // println!("becomes");
                        // s.1.print();
                        // assembly in big grid
                        let c0 = sub_c * (size_parts + 1);
                        let l0 = sub_l * (size_parts + 1);
                        let c1 = c0 + size_parts + 1;
                        let l1 = l0 + size_parts + 1;
                        for l in l0..l1 {
                            for c in c0..c1 {
                                new_big_grid.grid[l][c] = s.1.grid[l - l0][c - c0];
                            }
                        }
                        found = true;
                        break;
                    }
                }
                if !found {
                    panic!("not swap found");
                }
            }
        }
        grid = new_big_grid;
        // grid.print();
    }
    // grid.print();
    Some(grid.count_occurences('#'))
}

fn add_swaps_variations(s: &(Grid2D, Grid2D), new_swaps: &mut Vec<(Grid2D, Grid2D)>) {
    new_swaps.push((s.clone().0.horizontal_symetry(), s.clone().1));
    new_swaps.push((s.clone().0.horizontal_symetry().rotate(), s.clone().1));
    new_swaps.push((
        s.clone().0.horizontal_symetry().rotate().rotate(),
        s.clone().1,
    ));
    new_swaps.push((
        s.clone().0.horizontal_symetry().rotate().rotate().rotate(),
        s.clone().1,
    ));
    new_swaps.push((s.clone().0.vertical_symetry(), s.clone().1));
    new_swaps.push((s.clone().0.vertical_symetry().rotate(), s.clone().1));
    new_swaps.push((
        s.clone().0.vertical_symetry().rotate().rotate(),
        s.clone().1,
    ));
    new_swaps.push((
        s.clone().0.vertical_symetry().rotate().rotate().rotate(),
        s.clone().1,
    ));
    new_swaps.push((s.clone().0.rotate(), s.clone().1));
    new_swaps.push((s.clone().0.rotate().rotate(), s.clone().1));
    new_swaps.push((s.clone().0.rotate().rotate().rotate(), s.clone().1));
    new_swaps.sort();
    new_swaps.dedup();
    // println!("---- new swaps : ");
    // s.0.print();
    // for ns in new_swaps {
    //     ns.0.print();
    // }
    // println!("----");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total() {
        assert_eq!(
            get_answer(include_str!("../assets/day21_input.txt")),
            Some(3389823)
        );
    }
}
