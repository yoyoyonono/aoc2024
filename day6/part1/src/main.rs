use std::collections::HashSet;

#[derive(Debug)]
enum GridSpace {
    Guard,
    Obstruction,
}

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

        visited_spaces.insert([guard_pos[0], guard_pos[1]]);

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

    println!("{}", visited_spaces.len());
}
