use rayon::iter::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridSpace {
    Guard,
    Obstruction,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = include_str!("../input.txt");

    let grid: Vec<Vec<Option<GridSpace>>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| match c {
                    '^' => Some(GridSpace::Guard),
                    '#' => Some(GridSpace::Obstruction),
                    _ => None,
                })
                .collect()
        })
        .collect();

    let affected_spaces = find_spaces(grid.clone());

    let sum: i32 = (0..grid.len())
        .par_bridge()
        .map(|row| {
            (0..grid[row].len())
                .map(|col| {
                    if !affected_spaces.contains(&[row, col]) {
                        return 0;
                    }
                    let mut change = grid.clone();
                    if !(change[row][col] == None || change[row][col] == Some(GridSpace::Guard)) {
                        return 0;
                    }
                    change[row][col] = Some(GridSpace::Obstruction);
                    return match does_loop(change) {
                        true => 1,
                        false => 0,
                    };
                })
                .sum::<i32>()
        })
        .sum();
    println!("{sum}");
}

fn find_spaces(grid: Vec<Vec<Option<GridSpace>>>) -> HashSet<[usize; 2]> {
    // find guard position
    let mut guard_pos: [isize; 2] = [0, 0];

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, space) in row.iter().enumerate() {
            match space {
                Some(GridSpace::Guard) => {
                    guard_pos = [row_index.try_into().unwrap(), col_index.try_into().unwrap()];
                }
                _ => {}
            }
        }
    }

    let mut current_direction = Direction::Up;
    let mut visited_spaces = HashSet::new();

    while (guard_pos[0] >= 0)
        && (guard_pos[0] < grid.len().try_into().unwrap())
        && (guard_pos[1] >= 0)
        && (guard_pos[1] < grid[0].len().try_into().unwrap())
    {
        let guard_row: usize = guard_pos[0].try_into().unwrap();
        let guard_col: usize = guard_pos[1].try_into().unwrap();

        // check
        match grid[guard_row][guard_col] {
            Some(GridSpace::Obstruction) => match current_direction {
                Direction::Up => {
                    guard_pos[0] += 1;
                    current_direction = Direction::Right;
                }
                Direction::Down => {
                    guard_pos[0] -= 1;
                    current_direction = Direction::Left;
                }
                Direction::Left => {
                    guard_pos[1] += 1;
                    current_direction = Direction::Up;
                }
                Direction::Right => {
                    guard_pos[1] -= 1;
                    current_direction = Direction::Down;
                }
            },
            _ => {}
        }

        visited_spaces.insert([
            guard_pos[0].try_into().unwrap(),
            guard_pos[1].try_into().unwrap(),
        ]);

        // move
        match current_direction {
            Direction::Up => {
                guard_pos[0] -= 1;
            }
            Direction::Down => {
                guard_pos[0] += 1;
            }
            Direction::Left => {
                guard_pos[1] -= 1;
            }
            Direction::Right => {
                guard_pos[1] += 1;
            }
        };
    }

    visited_spaces
}

fn does_loop(grid: Vec<Vec<Option<GridSpace>>>) -> bool {
    // find guard position
    'outer: for (row_index, row) in grid.iter().enumerate() {
        for (col_index, space) in row.iter().enumerate() {
            if let Some(GridSpace::Guard) = space {
                guard_pos = [row_index as isize, col_index as isize];
                break 'outer;
            }
        }
    }
    let mut current_direction = Direction::Up;
    let mut visited_spaces = HashSet::with_capacity(grid.len() * grid[0].len() * 4);
    let rows = grid.len() as isize;
    let cols = grid[0].len() as isize;
    while guard_pos[0] >= 0 && guard_pos[0] < rows && guard_pos[1] >= 0 && guard_pos[1] < cols {
        let guard_row = guard_pos[0] as usize;
        let guard_col = guard_pos[1] as usize;
        if let Some(GridSpace::Obstruction) = grid[guard_row][guard_col] {
            match current_direction {
        // check
                Direction::Up => {
                    guard_pos[0] += 1;
                    current_direction = Direction::Right;
                }
                Direction::Down => {
                    guard_pos[0] -= 1;
                    current_direction = Direction::Left;
                }
                Direction::Left => {
                    guard_pos[1] += 1;
                    current_direction = Direction::Up;
                }
                Direction::Right => {
                    guard_pos[1] -= 1;
                    current_direction = Direction::Down;
                }
            }
        }
        let entry = (guard_pos[0], guard_pos[1], current_direction as u8);
        if !visited_spaces.insert(entry) {
            return true;
        }
        // move
        match current_direction {
            Direction::Up => guard_pos[0] -= 1,
            Direction::Down => guard_pos[0] += 1,
            Direction::Left => guard_pos[1] -= 1,
            Direction::Right => guard_pos[1] += 1,
        }
    }
    false
}
