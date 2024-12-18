use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

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
    // let input = include_str!("../input_test2.txt");
    // let input = include_str!("../input_test3.txt");
    // let input = include_str!("../input_test4.txt");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let start = [grid.len() - 2, 1];
    let end = [1, grid[1].len() - 2];

    let score = solve(&grid, start, end);

    println!("{}", score);
}

fn solve(grid: &Vec<Vec<char>>, start: [usize; 2], end: [usize; 2]) -> i32 {
    // create an attempt at the starting position

    let mut attempts = BinaryHeap::new();
    attempts.push((Reverse(0), vec![start]));
    let mut min_score = i32::MAX;
    let mut num_iters = 0;
    let mut visited_scores = vec![vec![i32::MAX; grid[0].len()]; grid.len()];

    while attempts.len() > 0 {
        num_iters += 1;

        let (_, mut current_attempt) = attempts.pop().unwrap();
        // println!("Current attempt: {:?} \t {}", current_attempt, calculate_score(&current_attempt));
        // print_solution(grid, &current_attempt);
        let mut current_pos = *current_attempt.last().unwrap();
        println!(
            "Attempts: {} \t Current: {}, {:?}",
            attempts.len() + 1,
            current_attempt.len(),
            current_pos
        );

        // if at the end, calculate the score and continue

        if current_pos == end {
            print_solution(grid, &current_attempt);
            let score = calculate_score(&current_attempt[1..current_attempt.len() - 1].to_vec());
            println!("Score: {}", score);
            if score < min_score {
                min_score = score;
            }
            continue;
        }

        // find neighbor empty spaces which haven't been visited

        let mut neighbors = vec![];

        if current_pos[0] > 0
            && grid[current_pos[0] - 1][current_pos[1]] == '.'
            && !current_attempt.contains(&[current_pos[0] - 1, current_pos[1]])
        {
            neighbors.push([current_pos[0] - 1, current_pos[1]]);
        }

        if current_pos[0] < grid.len() - 1
            && grid[current_pos[0] + 1][current_pos[1]] == '.'
            && !current_attempt.contains(&[current_pos[0] + 1, current_pos[1]])
        {
            neighbors.push([current_pos[0] + 1, current_pos[1]]);
        }

        if current_pos[1] > 0
            && grid[current_pos[0]][current_pos[1] - 1] == '.'
            && !current_attempt.contains(&[current_pos[0], current_pos[1] - 1])
        {
            neighbors.push([current_pos[0], current_pos[1] - 1]);
        }

        if current_pos[1] < grid[0].len() - 1
            && grid[current_pos[0]][current_pos[1] + 1] == '.'
            && !current_attempt.contains(&[current_pos[0], current_pos[1] + 1])
        {
            neighbors.push([current_pos[0], current_pos[1] + 1]);
        }

        // if there are no neighbors, continue

        if neighbors.len() == 0 {
            continue;
        }

        // if there are neighbors, create new attempts with the neighbors

        for neighbor in neighbors {
            let mut new_attempt = current_attempt.clone();
            new_attempt.push(neighbor);
            let score = calculate_score(&new_attempt);
            if score > min_score {
                continue;
            }
            if (score >= visited_scores[neighbor[0]][neighbor[1]]) && (score >= visited_scores[neighbor[0]][neighbor[1]] + 1000) {
                continue;
            } else {
                visited_scores[neighbor[0]][neighbor[1]] = score;
            }
            attempts.push((Reverse(score), new_attempt));
        }
    }
    println!("Num iters: {}", num_iters);
    // print_vec(visited_scores);
    min_score
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

    for i in 2..visited.len() {
        let last = visited[i - 2];
        let this = visited[i - 1];
        let next = visited[i];

        if get_direction(last, this) != get_direction(this, next) {
            turns += 1;
        }
    }

    score + 1 + (1000 * turns)
}

fn print_vec(thing: Vec<Vec<i32>>) {
    for row in thing {
        for col in row {
            print!("{:>10},", col);
        }
        println!();
    }
}
