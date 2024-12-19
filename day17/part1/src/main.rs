use core::panic;

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../input_test.txt");
    
    let halves = input.split_once("\n\n").unwrap();

    let mut registers = halves.0.lines();

    let program: Vec<i32> = halves.1.split_at(9).1.split(',').map(|x| x.parse().unwrap()).collect();

    let mut reg_a: i32 = registers.next().unwrap().split_at(12).1.parse().unwrap();
    let mut reg_b: i32 = registers.next().unwrap().split_at(12).1.parse().unwrap();
    let mut reg_c: i32 = registers.next().unwrap().split_at(12).1.parse().unwrap();

    println!("A: {reg_a}, B: {reg_b}, C:{reg_c}");

    let mut ip = 0;

    let mut output = vec![];

    while ip < program.len() {
        let instruction = program[ip];
        let operand = program[ip + 1];

        let value = match instruction {
            0 | 2 | 5 | 6 | 7 => {
                match operand {
                    4 => reg_a,
                    5 => reg_b,
                    6 => reg_c,
                    _ => operand
                }
            }
            _ => {
                operand
            }
        };

        println!("{}, {} -> {}", instruction, operand, value);

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
        println!("A: {reg_a}, B: {reg_b}, C:{reg_c}");
        ip += 2;
    }
    println!("{}", output.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(","));
}
