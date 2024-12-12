use colored::Colorize;
use rayon::iter::*;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../input_test.txt");

    let grid: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();

    let mut regions: Vec<HashSet<[usize; 2]>> = vec![];

    for (row_index, row) in grid.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if any_has(&regions, [row_index, col_index]) {
                continue;
            }
            regions.push(find_region(&grid, [row_index, col_index]));
        }
    }

    println!("{:?}", regions);

    println!(
        "Max size: {}, {}",
        regions
            .iter()
            .map(|x| convert_region(x))
            .map(|x| get_bottom(&x) - get_top(&x))
            .max()
            .unwrap(),
        regions
            .iter()
            .map(|x| convert_region(x))
            .map(|x| get_right(&x) - get_left(&x))
            .max()
            .unwrap()
    );

    let sum: u32 = regions
        // .par_iter()
        .iter()
        .map(|region| check_sides_region(region) * (region.len() as u32))
        .sum();
    println!("{sum}");
}

fn find_region(grid: &Vec<Vec<char>>, start_pos: [usize; 2]) -> HashSet<[usize; 2]> {
    unique(search(&grid, start_pos, &mut vec![start_pos]))
}

fn search(
    grid: &Vec<Vec<char>>,
    pos: [usize; 2],
    mut visited: &mut Vec<[usize; 2]>,
) -> Vec<[usize; 2]> {
    let original_value = grid[visited[0][0]][visited[0][1]];
    if grid[pos[0]][pos[1]] != original_value {
        return vec![];
    }

    let mut positions = vec![pos];

    visited.push(pos);

    // search up
    let new_up = [pos[0].checked_sub(1).unwrap_or_default(), pos[1]];
    if pos[0] > 0 && !visited.contains(&new_up) {
        positions.append(search(grid, new_up, &mut visited).as_mut());
    }

    // search down
    let new_down = [pos[0].checked_add(1).unwrap_or_default(), pos[1]];
    if pos[0] + 1 < grid.len() && !visited.contains(&new_down) {
        positions.append(search(grid, new_down, &mut visited).as_mut());
    }

    // search left
    let new_left = [pos[0], pos[1].checked_sub(1).unwrap_or_default()];
    if pos[1] > 0 && !visited.contains(&new_left) {
        positions.append(search(grid, new_left, &mut visited).as_mut());
    }

    // search right
    let new_right = [pos[0], pos[1].checked_add(1).unwrap_or_default()];
    if pos[1] + 1 < grid.len() && !visited.contains(&new_right) {
        positions.append(search(grid, new_right, &mut visited).as_mut());
    }

    positions
}

fn unique(a: Vec<[usize; 2]>) -> HashSet<[usize; 2]> {
    let mut seen = HashSet::new();
    for thing in a {
        seen.insert(thing);
    }
    seen
}

fn any_has(sets: &Vec<HashSet<[usize; 2]>>, pos: [usize; 2]) -> bool {
    for set in sets {
        if set.contains(&pos) {
            return true;
        }
    }
    false
}

fn check_sides_region(region: &HashSet<[usize; 2]>) -> u32 {
    let region_conv = convert_region(region);
    let leftmost: isize = get_left(&region_conv);
    let rightmost: isize = get_right(&region_conv);
    let top_row: isize = get_top(&region_conv);
    let bottom_row: isize = get_bottom(&region_conv);

    // find all points adjacent to the region
    let mut adjacent_points = HashSet::new();
    for row_index in top_row - 1..=bottom_row + 1 {
        for col_index in leftmost - 1..=rightmost + 1 {
            if region_conv.contains(&[row_index, col_index]) {
                continue;
            }
            if region_conv.contains(&[row_index - 1, col_index])
                // || region_conv.contains(&[row_index - 1, col_index + 1])
                || region_conv.contains(&[row_index, col_index + 1])
                // || region_conv.contains(&[row_index + 1, col_index + 1])
                || region_conv.contains(&[row_index + 1, col_index])
                // || region_conv.contains(&[row_index + 1, col_index - 1])
                || region_conv.contains(&[row_index, col_index - 1])
            // || region_conv.contains(&[row_index - 1, col_index - 1])
            {
                adjacent_points.insert([row_index, col_index]);
            }
        }
    }

    let mut checked_points = HashSet::new();
    let mut total_sides = 0;

    for point in adjacent_points {
        if checked_points.contains(&point) {
            continue;
        }
        let (sides, checked) = traverse_sides(&region_conv, point);
        total_sides += sides;
        for checked_point in checked {
            checked_points.insert(checked_point);
        }
    }

    total_sides
}

fn traverse_sides(
    region_conv: &HashSet<[isize; 2]>,
    initial_pos: [isize; 2],
) -> (u32, HashSet<[isize; 2]>) {
    let mut path = HashSet::new();

    let mut pos = initial_pos;
    let initial_direction = get_initial_direction(region_conv, pos);

    let mut sides = 0;
    let mut direction: Direction = initial_direction;

    while !(pos == initial_pos && direction == initial_direction) || sides < 4 {
        print_vizualise(region_conv, pos, sides);
        path.insert(pos);
        match direction {
            Direction::Right => {
                if region_conv.contains(&[pos[0], pos[1] + 1]) {
                    direction = Direction::Up;
                    sides += 1;
                    continue;
                }
                pos[1] += 1;
                if !region_conv.contains(&[pos[0] + 1, pos[1]]) {
                    direction = Direction::Down;
                    sides += 1;
                    continue;
                }
            }
            Direction::Down => {
                if region_conv.contains(&[pos[0] + 1, pos[1]]) {
                    direction = Direction::Right;
                    sides += 1;
                    continue;
                }
                pos[0] += 1;
                if !region_conv.contains(&[pos[0], pos[1] - 1]) {
                    direction = Direction::Left;
                    sides += 1;
                    continue;
                }
            }
            Direction::Left => {
                if region_conv.contains(&[pos[0], pos[1] - 1]) {
                    direction = Direction::Down;
                    sides += 1;
                    continue;
                }
                pos[1] -= 1;
                if !region_conv.contains(&[pos[0] - 1, pos[1]]) {
                    direction = Direction::Up;
                    sides += 1;
                    continue;
                }
            }
            Direction::Up => {
                if region_conv.contains(&[pos[0] - 1, pos[1]]) {
                    direction = Direction::Left;
                    sides += 1;
                    continue;
                }
                pos[0] -= 1;
                if !region_conv.contains(&[pos[0], pos[1] + 1]) {
                    direction = Direction::Right;
                    sides += 1;
                    continue;
                }
            }
        }
    }

    (sides, path)
}

fn get_top(region: &HashSet<[isize; 2]>) -> isize {
    region.iter().map(|x| x[0]).min().unwrap()
}

fn get_bottom(region: &HashSet<[isize; 2]>) -> isize {
    region.iter().map(|x| x[0]).max().unwrap()
}

fn get_left(region: &HashSet<[isize; 2]>) -> isize {
    region.iter().map(|x| x[1]).min().unwrap()
}

fn get_right(region: &HashSet<[isize; 2]>) -> isize {
    region.iter().map(|x| x[1]).max().unwrap()
}

fn get_initial_direction(region_conv: &HashSet<[isize; 2]>, pos: [isize; 2]) -> Direction {
    if region_conv.contains(&[pos[0] - 1, pos[1]]) {
        return Direction::Left;
    }
    if region_conv.contains(&[pos[0], pos[1] - 1]) {
        return Direction::Down;
    }
    if region_conv.contains(&[pos[0] + 1, pos[1]]) {
        return Direction::Right;
    }
    if region_conv.contains(&[pos[0], pos[1] + 1]) {
        return Direction::Up;
    }
    panic!()
}

fn print_vizualise(region_conv: &HashSet<[isize; 2]>, pos: [isize; 2], sides: u32) {
    let leftmost: isize = get_left(region_conv);
    let rightmost: isize = get_right(region_conv);
    let top_row: isize = get_top(region_conv);
    let bottom_row: isize = get_bottom(region_conv);

    print!("\x1B[2J\x1B[1;1H");
    println!("Position: {:?}", pos);
    println!("Sides: {}", sides);
    println!();
    for row in top_row - 1..=bottom_row + 1 {
        for col in leftmost - 1..=rightmost + 1 {
            if [row, col] == pos {
                print!("{}", "\u{2588}".red());
            } else if region_conv.contains(&[row, col]) {
                print!("{}", "\u{2588}".white());
            } else {
                print!(" ");
            }
        }
        println!();
    }

    // std::thread::sleep(std::time::Duration::from_millis(100));
}

fn convert_region(region: &HashSet<[usize; 2]>) -> HashSet<[isize; 2]> {
    region
        .iter()
        .map(|x| [x[0].try_into().unwrap(), x[1].try_into().unwrap()])
        .collect()
}
