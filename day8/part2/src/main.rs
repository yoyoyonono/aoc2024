use itertools::Itertools;
use rayon::iter::*;
use std::collections::*;

fn main() {
    let input = include_str!("../input.txt");
    //let input = include_str!("../input_test.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut antennae = HashMap::new();

    for (line_index, line) in grid.iter().enumerate() {
        for (space_index, space) in line.iter().enumerate() {
            if space == &'.' {
                continue;
            }
            if !antennae.contains_key(&space) {
                antennae.insert(space, vec![[line_index, space_index]]);
            } else {
                antennae
                    .get_mut(&space)
                    .unwrap()
                    .push([line_index, space_index]);
            }
        }
    }

    println!("{:?}", antennae);

    let out = antennae
        .iter()
        .map(|(freq, spaces)| {
            spaces
                .iter()
                .combinations(2)
                .map(|pair| {
                    let first = pair[0];
                    let second = pair[1];
                    try_pair(&grid, first, second)
                })
                .flatten()
                .collect_vec()
        })
        .flatten()
        .collect_vec();

    println!("{:?}", out);

    let mut unique = HashSet::new();

    for point in out {
        unique.insert(point);
    }

    println!("{}", unique.len());

    for (l, line) in grid.iter().enumerate() {
        for (q, c) in line.iter().enumerate() {
            if unique.contains(&[l, q]) {
                print!("#");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn try_pair(grid: &Vec<Vec<char>>, first: &[usize; 2], second: &[usize; 2]) -> Vec<[usize; 2]> {
    let mut answer = Vec::new();
    let first_to_second = get_distance(*first, *second);

    let grid_rows = grid.len().try_into().unwrap();
    let grid_cols = grid[0].len().try_into().unwrap();
    // check negative direction
    let mut pos: [isize; 2] = [first[0].try_into().unwrap(), first[1].try_into().unwrap()];
    while 0 <= pos[0] && pos[0] < grid_rows && 0 <= pos[1] && pos[1] < grid_cols {
        answer.push([pos[0].try_into().unwrap(), pos[1].try_into().unwrap()]);
        pos[0] -= first_to_second[0];
        pos[1] -= first_to_second[1];
    }

    // check positive direction
    let mut pos: [isize; 2] = [second[0].try_into().unwrap(), second[1].try_into().unwrap()];
    while 0 <= pos[0] && pos[0] < grid_rows && 0 <= pos[1] && pos[1] < grid_cols {
        answer.push([pos[0].try_into().unwrap(), pos[1].try_into().unwrap()]);
        pos[0] += first_to_second[0];
        pos[1] += first_to_second[1];
    }
    answer
}

fn get_distance(p1: [usize; 2], p2: [usize; 2]) -> [isize; 2] {
    let p1_0: isize = p1[0].try_into().unwrap();
    let p1_1: isize = p1[1].try_into().unwrap();
    let p2_0: isize = p2[0].try_into().unwrap();
    let p2_2: isize = p2[1].try_into().unwrap();

    [p1_0 - p2_0, p1_1 - p2_2]
}
