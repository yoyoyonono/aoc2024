use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../input_test.txt");
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let zero_pos: Vec<[usize; 2]> = grid
        .iter()
        .enumerate()
        .map(|(line_index, line)| {
            line.iter()
                .enumerate()
                .filter(|(space_index, space)| space == &&0)
                .map(move |x| [line_index, x.0])
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let sum: usize = zero_pos
        .iter()
        .map(|pos| (search(&grid, *pos, &Vec::new())).len())
        .sum();
    println!("{:?}", sum);
}

fn search(grid: &Vec<Vec<u32>>, pos: [usize; 2], visited: &Vec<[usize; 2]>) -> Vec<[usize; 2]> {
    if grid[pos[0]][pos[1]] == 9 {
        return vec![pos];
    }

    let value_here = grid[pos[0]][pos[1]];
    let mut positions = vec![];

    let mut new_visited = visited.clone();
    new_visited.push(pos);

    // search up
    let new_up = [pos[0].checked_sub(1).unwrap_or_default(), pos[1]];
    if pos[0] > 0 && !visited.contains(&new_up) && grid[new_up[0]][new_up[1]] == value_here + 1 {
        positions.append(search(grid, new_up, &new_visited).as_mut());
    }

    // search down
    let new_down = [pos[0].checked_add(1).unwrap_or_default(), pos[1]];
    if pos[0] + 1 < grid.len()
        && !visited.contains(&new_down)
        && grid[new_down[0]][new_down[1]] == value_here + 1
    {
        positions.append(search(grid, new_down, &new_visited).as_mut());
    }

    // search left
    let new_left = [pos[0], pos[1].checked_sub(1).unwrap_or_default()];
    if pos[1] > 0
        && !visited.contains(&new_left)
        && grid[new_left[0]][new_left[1]] == value_here + 1
    {
        positions.append(search(grid, new_left, &new_visited).as_mut());
    }

    // search right
    let new_right = [pos[0], pos[1].checked_add(1).unwrap_or_default()];
    if pos[1] + 1 < grid.len()
        && !visited.contains(&new_right)
        && grid[new_right[0]][new_right[1]] == value_here + 1
    {
        positions.append(search(grid, new_right, &new_visited).as_mut());
    }

    positions
}

fn unique(a: Vec<[usize; 2]>) -> Vec<[usize; 2]> {
    let mut seen = HashSet::new();
    for thing in a {
        seen.insert(thing);
    }
    seen.iter().map(|x| *x).collect()
}
