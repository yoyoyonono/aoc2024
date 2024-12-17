use colored::Colorize;
use std::collections::{HashMap, HashSet};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoxTile {
    LeftHalf,
    RightHalf,
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../input_test.txt");
    // let input = include_str!("../input_test1.txt");

    let split = input.split_at(input.find("\n\n").unwrap());

    let grid: Vec<Vec<char>> = split
        .0
        .replace('#', "##")
        .replace('O', "[]")
        .replace('.', "..")
        .replace('@', "@.")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let size: [i32; 2] = [
        grid.len().try_into().unwrap(),
        grid[0].len().try_into().unwrap(),
    ];

    let mut robot_pos = [-1, -1];

    let mut boxes = HashMap::new();
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
                '[' => {
                    boxes.insert(pos, BoxTile::LeftHalf);
                }
                ']' => {
                    boxes.insert(pos, BoxTile::RightHalf);
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

        if i + 1 == 189 {
            print!("");
        }

        if check_direction(robot_pos, &direction, &walls) {
            // print_grid(robot_pos, &walls, &boxes, size);
            continue;
        }

        if !check_direction_map(robot_pos, &direction, &boxes) {
            robot_pos = get_from_direction(robot_pos, &direction);
            // print_grid(robot_pos, &walls, &boxes, size);
            continue;
        }

        // box case
        if let Some(new_boxes) = check_box(
            get_from_direction(robot_pos, direction),
            direction,
            &boxes,
            &walls,
            &vec![],
        ) {
            for new_box in new_boxes {
                // println!("remove {:?}", new_box);
                if let Some(new_box_type) = boxes.remove(&new_box) {
                    boxes.insert(get_from_direction(new_box, &direction), new_box_type);
                }
            }
            robot_pos = get_from_direction(robot_pos, direction);
        }

        // print_grid(robot_pos, &walls, &boxes, size);
    }
    let sum: i32 = boxes
        .iter()
        .filter(|b| *b.1 == BoxTile::LeftHalf)
        .map(|b| get_gps(*b.0))
        .sum();
    println!("Gps: {}", sum);
}

fn check_box(
    position: [i32; 2],
    direction: &Direction,
    boxes: &HashMap<[i32; 2], BoxTile>,
    walls: &HashSet<[i32; 2]>,
    checked: &Vec<[i32; 2]>,
) -> Option<Vec<[i32; 2]>> {
    if walls.contains(&position) {
        return None;
    }

    if checked.contains(&position) || (checked.len() > 0 && !boxes.contains_key(&position)) {
        return Some(vec![]);
    }

    // println!("{:?}", position);

    let mut new_checked = checked.clone();
    new_checked.push(position);

    // actual box case

    let mut try_boxes = vec![];
    // check in the direction
    if let Some(new_boxes) = check_box(
        get_from_direction(position, direction),
        direction,
        boxes,
        walls,
        &new_checked,
    ) {
        let check_try_boxes = try_boxes.clone();
        let to_add = new_boxes.iter().filter(|x| !(check_try_boxes).contains(*x));
        try_boxes.extend(to_add);
    } else {
        return None;
    }

    // check the other half of the box
    if let Some(x) = boxes.get(&position) {
        match x {
            BoxTile::LeftHalf => {
                if *direction != Direction::Right {
                    if let Some(new_boxes) = check_box(
                        [position[0], position[1] + 1],
                        direction,
                        boxes,
                        walls,
                        &new_checked,
                    ) {
                        let check_try_boxes = try_boxes.clone();
                        let to_add = new_boxes.iter().filter(|x| !(check_try_boxes).contains(*x));
                        try_boxes.extend(to_add);
                    } else {
                        return None;
                    }
                }
            }
            BoxTile::RightHalf => {
                if *direction != Direction::Left {
                    if let Some(new_boxes) = check_box(
                        [position[0], position[1] - 1],
                        direction,
                        boxes,
                        walls,
                        &new_checked,
                    ) {
                        let check_try_boxes = try_boxes.clone();
                        let to_add = new_boxes.iter().filter(|x| !(check_try_boxes).contains(*x));
                        try_boxes.extend(to_add);
                    } else {
                        return None;
                    }
                }
            }
        }
    }

    // if the thing in the direction is okay, then go
    try_boxes.push(position);
    Some(try_boxes)
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

fn check_direction_map(
    position: [i32; 2],
    direction: &Direction,
    set: &HashMap<[i32; 2], BoxTile>,
) -> bool {
    set.contains_key(&get_from_direction(position, direction))
}

fn print_grid(
    robot_pos: [i32; 2],
    walls: &HashSet<[i32; 2]>,
    boxes: &HashMap<[i32; 2], BoxTile>,
    size: [i32; 2],
) {
    for row in 0..size[0] {
        for col in 0..size[1] {
            let pos = [row, col];
            if robot_pos == pos {
                print!("{}", "\u{2588}".red());
            } else if walls.contains(&pos) {
                print!("{}", "\u{2588}".white());
            } else if let Some(box_tile) = boxes.get(&pos) {
                match box_tile {
                    BoxTile::LeftHalf => {
                        print!("{}", "[".yellow());
                    }
                    BoxTile::RightHalf => {
                        print!("{}", "]".yellow());
                    }
                }
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
