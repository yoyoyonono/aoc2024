use std::collections::HashSet;
use rayon::iter::*;

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

    let sum: u32 = regions.par_iter().map(|region| get_perimiter(&grid, region) * (region.len() as u32)).sum();
    println!("{sum}");
}

fn find_region(grid: &Vec<Vec<char>>, start_pos: [usize; 2]) -> HashSet<[usize; 2]> {
    unique(search(&grid, start_pos, &mut vec![start_pos]))
}

fn search(grid: &Vec<Vec<char>>, pos: [usize; 2], mut visited: &mut Vec<[usize; 2]>) -> Vec<[usize; 2]> {
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

fn get_perimiter(grid: &Vec<Vec<char>>, region: &HashSet<[usize; 2]>) -> u32 {
    region.iter().map(|pos| perimiter_contribution(&grid, region, *pos)).sum()
}

fn perimiter_contribution(grid: &Vec<Vec<char>>, region: &HashSet<[usize; 2]>, pos: [usize; 2]) -> u32 {

    let mut contribution = 4;

    // search up
    let new_up = [pos[0].checked_sub(1).unwrap_or_default(), pos[1]];
    if pos[0] > 0 && region.contains(&new_up) {
        contribution -= 1;
    }

    // search down
    let new_down = [pos[0].checked_add(1).unwrap_or_default(), pos[1]];
    if pos[0] + 1 < grid.len() && region.contains(&new_down) {
        contribution -= 1;
    }

    // search left
    let new_left = [pos[0], pos[1].checked_sub(1).unwrap_or_default()];
    if pos[1] > 0 && region.contains(&new_left) {
        contribution -= 1;
    }

    // search right
    let new_right = [pos[0], pos[1].checked_add(1).unwrap_or_default()];
    if pos[1] + 1 < grid.len() && region.contains(&new_right) {
        contribution -= 1;
    }
    contribution
}
