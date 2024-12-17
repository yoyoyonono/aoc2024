use std::{collections::HashSet, f32::MIN_POSITIVE};
use colored::Colorize;

#[derive(Debug)]
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

    let split = input.split_at(input.find("\n\n").unwrap());

    let grid: Vec<Vec<char>> = split.0.lines().map(|line| line.chars().collect()).collect();

    let size: [i32; 2] = [
        grid.len().try_into().unwrap(),
        grid[0].len().try_into().unwrap(),
    ];

    let mut robot_pos = [-1, -1];

    let mut boxes = HashSet::new();
    let mut walls = HashSet::new();

    for (line_index, line) in grid.iter().enumerate() {
        for (char_index, char) in line.iter().enumerate() {
            let pos: [i32; 2] = [
                line_index.try_into().unwrap(),
                char_index.try_into().unwrap(),
            ];
            match char {
                '@' => {
                    robot_pos = pos;
                }
                'O' => {
                    boxes.insert(pos);
                }
                '#' => {
                    walls.insert(pos);
                }
                _ => {}
            }
        }
    }

    println!("Initial State: ");
    print_grid(robot_pos, &walls, &boxes, size);

    let moves: Vec<Direction> = split
        .1
        .chars()
        .filter(|x| *x != '\n')
        .map(|x| match x {
            '^' => Direction::Up,
            'v' => Direction::Down,
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => {
                panic!()
            }
        })
        .collect();

    for (i, direction) in moves.iter().enumerate() {
        print!("\x1B[2J\x1B[1;1H");
        println!("Move {}: {:?}", i + 1, direction);
        if check_direction(robot_pos, &direction, &walls) {
            print_grid(robot_pos, &walls, &boxes, size);
            continue;
        }

        if !check_direction(robot_pos, &direction, &boxes) {
            robot_pos = get_from_direction(robot_pos, &direction);
            print_grid(robot_pos, &walls, &boxes, size);
            continue;
        }

        // box case
        let mut checking_pos = get_from_direction(robot_pos, direction);
        let mut steps = 1;

        while !walls.contains(&checking_pos) && boxes.contains(&checking_pos) {
            checking_pos = get_from_direction(checking_pos, direction);
            steps += 1;
        }

        if walls.contains(&checking_pos) {
            continue;
        }

        boxes.remove(&get_from_direction(robot_pos, direction));
        boxes.insert(get_multiple_direction(robot_pos, direction, steps));
        robot_pos = get_from_direction(robot_pos, direction);
        print_grid(robot_pos, &walls, &boxes, size);
    }
    let sum: i32 = boxes.iter().map(|b| get_gps(*b)).sum();
    println!("Gps: {}", sum);
}

fn get_from_direction(position: [i32; 2], direction: &Direction) -> [i32; 2] {
    get_multiple_direction(position, direction, 1)
}

fn get_multiple_direction(position: [i32; 2], direction: &Direction, number: i32) -> [i32; 2] {
    match direction {
        Direction::Up => [position[0] - number, position[1]],
        Direction::Down => [position[0] + number, position[1]],
        Direction::Left => [position[0], position[1] - number],
        Direction::Right => [position[0], position[1] + number],
    }
}

fn check_direction(position: [i32; 2], direction: &Direction, set: &HashSet<[i32; 2]>) -> bool {
    set.contains(&get_from_direction(position, direction))
}

fn print_grid(
    robot_pos: [i32; 2],
    walls: &HashSet<[i32; 2]>,
    boxes: &HashSet<[i32; 2]>,
    size: [i32; 2],
) {
    for row in 0..size[0] {
        for col in 0..size[1] {
            let pos = [row, col];
            if robot_pos == pos {
                print!("{}", "\u{2588}".red());
            } else if walls.contains(&pos) {
                print!("{}", "\u{2588}".white());
            } else if boxes.contains(&pos) {
                print!("{}", "\u{2588}".yellow());
            } else {
                print!(" ");
            }
        }
        println!();
    }
    println!();
    std::thread::sleep(std::time::Duration::from_millis(20));
}

fn get_gps(point: [i32; 2]) -> i32 {
    point[0] * 100 + point[1]
}
