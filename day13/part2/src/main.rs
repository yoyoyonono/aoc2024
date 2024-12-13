use core::num;
use std::arch::x86_64::_SIDD_LEAST_SIGNIFICANT;

#[derive(Debug, Clone, Copy)]
struct Machine {
    prize: [i128; 2],
    button_a: [i128; 2],
    button_b: [i128; 2],
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
            let prize_raw = parse(desc.next().unwrap());
            let prize = [prize_raw[0] + 10000000000000, prize_raw[1] + 10000000000000];
            // let prize = [prize_raw[0], prize_raw[1]];
            Machine {
                prize: prize,
                button_a: button_a,
                button_b: button_b,
            }
        })
        .collect();
    println!("{:?}", machines);
    let sum: i128 = machines
        .iter()
        .map(|x| find_cheap_way(*x).unwrap_or(0))
        .sum();
    println!("{sum}");
}

fn find_cheap_way(machine: Machine) -> Option<i128> {
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

    let mut minimum_price = i128::MAX;

    let mut num_b = max_b;
    let mut num_a = 0;

    let mut last_num_a = i128::MIN;
    let mut last_num_b = i128::MIN;

    while num_b > 0 {
        let mul_b = multiply(num_b, machine.button_b);
        num_a = std::cmp::max(
            ((machine.prize[0] - mul_b[0]) as f64 / machine.button_a[0] as f64).round() as i128,
            ((machine.prize[1] - mul_b[1]) as f64 / machine.button_a[1] as f64).round() as i128,
        );
        let mul_a = multiply(num_a, machine.button_a);
        let new_try = add(mul_b, mul_a);

        println!(
            "{num_a} {num_b} \t {:?} \t {:?} \t {minimum_price}",
            new_try, machine.prize
        );

        if new_try[0] == machine.prize[0] && new_try[1] == machine.prize[1] {
            minimum_price = std::cmp::min(
                minimum_price,
                num_b * button_b_price + num_a * button_a_price,
            );
            break;
        }
        num_b = std::cmp::min(
            ((machine.prize[0] - mul_a[0]) as f64 / machine.button_b[0] as f64).round() as i128,
            ((machine.prize[1] - mul_a[1]) as f64 / machine.button_b[1] as f64).round() as i128,
        );

        if last_num_a == num_a && last_num_b == num_b {
            num_b -= 1;
        }

        last_num_a = num_a;
        last_num_b = num_b;
    }

    if minimum_price < i128::MAX {
        println!("Found, {minimum_price}");
        return Some(minimum_price);
    }
    println!("Not found");
    None
}

fn find_minimum_greater(prize: [i128; 2], button: [i128; 2]) -> i128 {
    std::cmp::min(prize[0] / button[0], prize[1] / button[1])
}

fn parse(line: &str) -> [i128; 2] {
    let mut line_cut = line.split_once(":").unwrap().1.split_ascii_whitespace();
    let line_1 = &line_cut.next().unwrap()[2..];
    let line_1_num = line_1[..line_1.len() - 1].parse::<i128>().unwrap();

    let line_2 = &line_cut.next().unwrap()[2..];
    let line_2_num = line_2.parse::<i128>().unwrap();

    [line_1_num, line_2_num]
}

fn multiply(count: i128, button: [i128; 2]) -> [i128; 2] {
    [button[0] * count, button[1] * count]
}

fn add(button_1: [i128; 2], button_2: [i128; 2]) -> [i128; 2] {
    [button_1[0] + button_2[0], button_1[1] + button_2[1]]
}
