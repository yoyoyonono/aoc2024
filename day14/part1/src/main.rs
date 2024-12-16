#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Pair {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
struct Robot {
    position: Pair,
    velocity: Pair,
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../input_test.txt");

    let size = Pair { x: 101, y: 103 };
    // let size = Pair { x: 11, y: 7 };

    let robots: Vec<Robot> = input
        .lines()
        .map(|line| {
            let line_split: Vec<&str> = line.split_ascii_whitespace().collect();
            let p_half: Vec<i32> = line_split[0][2..]
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            let pos = Pair {
                x: p_half[0],
                y: p_half[1],
            };
            let v_half: Vec<i32> = line_split[1][2..]
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();
            let vel = Pair {
                x: v_half[0],
                y: v_half[1],
            };
            Robot {
                position: pos,
                velocity: vel,
            }
        })
        .collect();

    let mut new_robots = robots;
    for _ in 0..100 {
        new_robots = new_robots
            .iter()
            .map(|robot| robot_step(robot, size))
            .collect();
        print_robots(&new_robots, size);
    }

    println!("{}", safety_score(&new_robots, size));
}

fn robot_step(robot: &Robot, size: Pair) -> Robot {
    let new_pos = Pair {
        x: (robot.position.x + robot.velocity.x).rem_euclid(size.x),
        y: (robot.position.y + robot.velocity.y).rem_euclid(size.y),
    };
    Robot {
        position: new_pos,
        velocity: robot.velocity,
    }
}

fn print_robots(robots: &Vec<Robot>, size: Pair) {
    let positions: Vec<Pair> = robots.iter().map(|robot| robot.position).collect();
    for y in 0..size.y {
        for x in 0..size.x {
            let num = positions
                .iter()
                .filter(|position| **position == Pair { x: x, y: y })
                .count();
            if num == 0 {
                print!(".");
            } else {
                print!("{}", num);
            }
        }
        println!();
    }
    println!();
}

fn safety_score(robots: &Vec<Robot>, size: Pair) -> usize {
    let top_left_quadrant = robots
        .iter()
        .filter(|robot| robot.position.x < size.x / 2 && robot.position.y < size.y / 2)
        .count();
    let top_right_quadrant = robots
        .iter()
        .filter(|robot| robot.position.x > size.x / 2 && robot.position.y < size.y / 2)
        .count();
    let bottom_left_quadrant = robots
        .iter()
        .filter(|robot| robot.position.x < size.x / 2 && robot.position.y > size.y / 2)
        .count();
    let bottom_right_quadrant = robots
        .iter()
        .filter(|robot| robot.position.x > size.x / 2 && robot.position.y > size.y / 2)
        .count();

    top_left_quadrant * top_right_quadrant * bottom_left_quadrant * bottom_right_quadrant
}
