use core::panic;

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../input_test.txt");
    // let input = include_str!("../input_test1.txt");

    let halves = input.split_once("\n\n").unwrap();

    let mut registers = halves.0.lines();

    let program: Vec<i128> = halves
        .1
        .split_at(9)
        .1
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let mut reg_a: i128 = registers.next().unwrap().split_at(12).1.parse().unwrap();
    let mut reg_b: i128 = registers.next().unwrap().split_at(12).1.parse().unwrap();
    let mut reg_c: i128 = registers.next().unwrap().split_at(12).1.parse().unwrap();

    let mut solution: i128 = 0;

    let correct_digits = 0;

    loop {
        let output = run_program(solution, reg_b, reg_c, &program);
        println!("{}, {:?}", solution, output);

        if output == program {
            break;
        }

        if output == program[program.len() - output.len()..] {
            solution <<= 3;
            continue;
        }

        solution += 1;
    }
}

fn run_program(a: i128, b: i128, c: i128, program: &Vec<i128>) -> Vec<i128> {
    let mut reg_a: i128 = a;
    let mut reg_b: i128 = b;
    let mut reg_c: i128 = c;

    // println!("A: {reg_a}, B: {reg_b}, C:{reg_c}");

    let mut ip = 0;

    let mut output = vec![];

    while ip < program.len() {
        let instruction = program[ip];
        let operand = program[ip + 1];

        let value = match instruction {
            0 | 2 | 5 | 6 | 7 => match operand {
                4 => reg_a,
                5 => reg_b,
                6 => reg_c,
                _ => operand,
            },
            _ => operand,
        };

        // println!("{}, {} -> {}", instruction, operand, value);

        match instruction {
            0 => {
                // adv
                reg_a = reg_a >> value;
            }
            1 => {
                // bxl
                reg_b = reg_b ^ value;
            }
            2 => {
                //bst
                reg_b = value % 8;
            }
            3 => {
                //jnz
                if reg_a != 0 {
                    ip = value.try_into().unwrap();
                    continue;
                }
            }
            4 => {
                //bxc
                reg_b = reg_b ^ reg_c;
            }
            5 => {
                //out
                output.push(value % 8);
            }
            6 => {
                //bdv
                reg_b = reg_a / (1 << value);
            }
            7 => {
                //cdv
                reg_c = reg_a / (1 << value);
            }
            _ => {
                panic!();
            }
        }
        // println!("A: {reg_a}, B: {reg_b}, C:{reg_c}");
        ip += 2;
    }
    // println!("{}", output.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(","));
    output
}

fn solution_to_number(solution: &Vec<i128>) -> i128 {
    let mut total = 0;
    for number in solution {
        total <<= 3;
        total += number % 8;
    }
    total
}
