#[derive(Debug, Clone, Copy)]
struct Machine {
    prize: [i32; 2],
    button_a: [i32; 2],
    button_b: [i32; 2],
}

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../input_test.txt");
    let machines: Vec<Machine> = input
        .split("\n\n")
        .map(|x| {
            let mut desc = x.lines();
            let button_a = parse(desc.next().unwrap());
            let button_b = parse(desc.next().unwrap());
            let prize = parse(desc.next().unwrap());
            Machine {
                prize: prize,
                button_a: button_a,
                button_b: button_b,
            }
        })
        .collect();
    println!("{:?}", machines);
    let sum: i32 = machines
        .iter()
        .map(|x| find_cheap_way(*x).unwrap_or(0))
        .sum();
    println!("{sum}");
}

fn find_cheap_way(machine: Machine) -> Option<i32> {
    let button_a_price = 3;
    let button_b_price = 1;

    // is it multiple of b button
    if (machine.prize[0] % machine.button_b[0] == 0 && machine.prize[1] % machine.button_b[1] == 0)
        && (machine.prize[0] / machine.button_b[0] == machine.prize[1] / machine.button_b[1])
    {
        return Some(machine.prize[0] / machine.button_b[0]);
    }

    // find number of b buttons to go over

    let max_b = find_minimum_greater(machine.prize, machine.button_b);

    let mut minimum_price = i32::MAX;

    let mut num_b = max_b;
    let mut num_a = 0;

    while num_b > 0 {
        let new_try = add(
            multiply(num_b, machine.button_b),
            multiply(num_a, machine.button_a),
        );

        // println!("{num_a} {num_b} {:?}", new_try);

        if new_try[0] < machine.prize[0] && new_try[1] < machine.prize[1] {
            num_a += 1;
            continue;
        } else if new_try[0] == machine.prize[0] && new_try[1] == machine.prize[1] {
            minimum_price = std::cmp::min(
                minimum_price,
                num_b * button_b_price + num_a * button_a_price,
            );
        }
        num_b -= 1;
        num_a = 0;
    }

    if minimum_price < i32::MAX {
        return Some(minimum_price);
    }
    None
}

fn find_minimum_greater(prize: [i32; 2], button: [i32; 2]) -> i32 {
    let mut pos = [0, 0];
    let mut count = 0;
    while pos[0] < prize[0] || pos[1] < prize[1] {
        pos[0] += button[0];
        pos[1] += button[1];
        count += 1
    }
    count
}

fn parse(line: &str) -> [i32; 2] {
    let mut line_cut = line.split_once(":").unwrap().1.split_ascii_whitespace();
    let line_1 = &line_cut.next().unwrap()[2..];
    let line_1_num = line_1[..line_1.len() - 1].parse().unwrap();

    let line_2 = &line_cut.next().unwrap()[2..];
    let line_2_num = line_2.parse().unwrap();

    [line_1_num, line_2_num]
}

fn multiply(count: i32, button: [i32; 2]) -> [i32; 2] {
    [button[0] * count, button[1] * count]
}

fn add(button_1: [i32; 2], button_2: [i32; 2]) -> [i32; 2] {
    [button_1[0] + button_2[0], button_1[1] + button_2[1]]
}
