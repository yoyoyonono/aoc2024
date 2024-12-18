use std::cmp::min;

use fxhash::FxHashMap;
use memoize::memoize;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../input_test.txt");
    // let input = include_str!("../input_test1.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = [grid.len() - 2, 1];
    let end = [1, grid[1].len() - 2];

    let score = solve(&grid, start, end);

    println!("{}", score);
}

fn solve(grid: &Vec<Vec<char>>, start: [usize; 2], end: [usize; 2]) -> i32 {
    let mut min_score = i32::MAX;
    let mut best_scores_squares: Vec<Vec<i32>> = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
    let mut num_iters = 0;
    if grid[start[0]][start[1] + 1] != '#' {
        solve_helper(
            grid,
            [start[0], start[1] + 1],
            end,
            &mut min_score,
            &mut vec![],
            Direction::Right,
            1,
            &mut best_scores_squares,
            &mut num_iters,
        );
    }
    if grid[start[0] - 1][start[1]] != '#' {
        solve_helper(
            grid,
            [start[0] - 1, start[1]],
            end,
            &mut min_score,
            &mut vec![],
            Direction::Up,
            1001,
            &mut best_scores_squares,
            &mut num_iters,
        );
    }
    print_vec(best_scores_squares);
    println!("{} iterations", num_iters);
    min_score
}

fn solve_helper(
    grid: &Vec<Vec<char>>,
    pos: [usize; 2],
    end: [usize; 2],
    min_score: &mut i32,
    visited: &mut Vec<[usize; 2]>,
    last_direction: Direction,
    current_score: i32,
    best_scores_squares: &mut Vec<Vec<i32>>,
    num_iters: &mut i32,
) {
    if pos == end {
        let score = current_score;
        print_solution(grid, visited);
        println!("Score: {}", score);
        propogate_score(visited, best_scores_squares);
        if score < *min_score {
            *min_score = score;
        }
        return;
    }

    if grid[pos[0]][pos[1]] == '#' || visited.contains(&pos) {
        return;
    } 
    
    if current_score > *min_score  {
        return;
    }
    
    if best_scores_squares[pos[0]][pos[1]] != i32::MAX && best_scores_squares[pos[0]][pos[1]] + current_score > *min_score {
        return;
    }

    *num_iters += 1;

    visited.push(pos);

    match last_direction {
        Direction::Up => {
            solve_helper(
                grid,
                [pos[0] - 1, pos[1]],
                end,
                min_score,
                visited,
                Direction::Up,
                current_score + 1,
                best_scores_squares,
                num_iters,
            );
            solve_helper(
                grid,
                [pos[0], pos[1] + 1],
                end,
                min_score,
                visited,
                Direction::Right,
                current_score + 1001,
                best_scores_squares,
                num_iters,
            );
            solve_helper(
                grid,
                [pos[0], pos[1] - 1],
                end,
                min_score,
                visited,
                Direction::Left,
                current_score + 1001,
                best_scores_squares,
                num_iters,
            );
        }
        Direction::Down => {
            solve_helper(
                grid,
                [pos[0] + 1, pos[1]],
                end,
                min_score,
                visited,
                Direction::Down,
                current_score + 1,
                best_scores_squares,
                num_iters,
            );
            solve_helper(
                grid,
                [pos[0], pos[1] + 1],
                end,
                min_score,
                visited,
                Direction::Right,
                current_score + 1001,
                best_scores_squares,
                num_iters,
            );
            solve_helper(
                grid,
                [pos[0], pos[1] - 1],
                end,
                min_score,
                visited,
                Direction::Left,
                current_score + 1001,
                best_scores_squares,
                num_iters,
            );
        }
        Direction::Left => {
            solve_helper(
                grid,
                [pos[0], pos[1] - 1],
                end,
                min_score,
                visited,
                Direction::Left,
                current_score + 1,
                best_scores_squares,
                num_iters,
            );
            solve_helper(
                grid,
                [pos[0] + 1, pos[1]],
                end,
                min_score,
                visited,
                Direction::Down,
                current_score + 1001,
                best_scores_squares,
                num_iters,
            );
            solve_helper(
                grid,
                [pos[0] - 1, pos[1]],
                end,
                min_score,
                visited,
                Direction::Up,
                current_score + 1001,
                best_scores_squares,
                num_iters,
            );
        }
        Direction::Right => {
            solve_helper(
                grid,
                [pos[0], pos[1] + 1],
                end,
                min_score,
                visited,
                Direction::Right,
                current_score + 1,
                best_scores_squares,
                num_iters,
            );
            solve_helper(
                grid,
                [pos[0] + 1, pos[1]],
                end,
                min_score,
                visited,
                Direction::Down,
                current_score + 1001,
                best_scores_squares,
                num_iters,
            );
            solve_helper(
                grid,
                [pos[0] - 1, pos[1]],
                end,
                min_score,
                visited,
                Direction::Up,
                current_score + 1001,
                best_scores_squares,
                num_iters,
            );
        }
    }
    visited.pop();
}

fn get_direction(last: [usize; 2], this: [usize; 2]) -> Direction {
    if last[0] == this[0] {
        if last[1] < this[1] {
            Direction::Right
        } else {
            Direction::Left
        }
    } else {
        if last[0] < this[0] {
            Direction::Down
        } else {
            Direction::Up
        }
    }
}

fn print_solution(grid: &Vec<Vec<char>>, visited: &Vec<[usize; 2]>) {
    for (row, line) in grid.iter().enumerate() {
        for (col, c) in line.iter().enumerate() {
            if visited.contains(&[row, col]) {
                print!("O");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn calculate_score(visited: &Vec<[usize; 2]>) -> i32 {
    let score: i32 = visited.len().try_into().unwrap();

    if visited.len() == 1 {
        return 0;
    } else if visited.len() == 2 {
        return 1;
    }
    // if the first direction is up, add 1000

    let mut turns = 0;

    if get_direction(visited[0], visited[1]) == Direction::Up {
        turns += 1;
    }

    // find the number of turns

    for i in 1..visited.len() - 2 {
        let last = visited[i - 1];
        let this = visited[i];
        let next = visited[i + 1];

        if get_direction(last, this) != get_direction(this, next) {
            turns += 1;
        }
    }

    score + 1 + (1000 * turns)
}

fn propogate_score(visited: &Vec<[usize; 2]>, best_scores_squares: &mut Vec<Vec<i32>>) {
    // go through visited backwards and update the best_scores_squares of the corresponding position if the score to the end is better on this path
    let mut cumulative_score = 0; // Start from the end of the path and propagate scores backward.

    // Traverse the visited path in reverse order.
    for i in (0..visited.len()).rev() {
        let pos = visited[i];

        // Update the cumulative score.
        // If this is not the last position, calculate the additional cost.
        if i < visited.len() - 1 {
            let current_dir = get_direction(visited[i], visited[i + 1]);
            let next_dir = if i + 2 < visited.len() {
                get_direction(visited[i + 1], visited[i + 2])
            } else {
                current_dir.clone() // No next direction, maintain the same direction.
            };

            if current_dir == next_dir {
                cumulative_score += 1; // Continuing in the same direction costs 1.
            } else {
                cumulative_score += 1001; // Changing direction costs 1001.
            }
        }

        // Update the best score for this position if the new score is better.
        if best_scores_squares[pos[0]][pos[1]] > cumulative_score || best_scores_squares[pos[0]][pos[1]] == 0 {
            best_scores_squares[pos[0]][pos[1]] = cumulative_score;
        }
    }
}

fn print_vec(thing: Vec<Vec<i32>>) {
    for row in thing {
        for col in row {
            print!("{:>11}, \t", col);
        }
        println!();
    }
}
