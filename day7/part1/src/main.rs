#[derive(Debug)]
struct Equation {
    target: i128,
    numbers: Vec<i128>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Times,
}

fn main() {
    let input = include_str!("../input.txt");
    let equations: Vec<Equation> = input
        .lines()
        .map(|line| {
            let (target, numbers) = line.split_once(':').unwrap();
            Equation {
                target: target.parse().unwrap(),
                numbers: numbers[1..]
                    .split(' ')
                    .map(|x| x.parse().unwrap())
                    .collect(),
            }
        })
        .collect();
    println!("{:?}", equations);

    let sum: i128 = equations
        .iter()
        .map(|equation| {
            if (generate_combinations(equation.numbers.len())
                .iter()
                .map(|operators| {
                    if evaluate_equation(equation.numbers.clone(), (*operators).clone())
                        == equation.target
                    {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i128>())
                != 0
            {
                equation.target
            } else {
                0
            }
        })
        .sum();

    println!("{sum}");
}

fn generate_combinations(length: usize) -> Vec<Vec<Operator>> {
    let combinations = (0..(1 << length))
        .map(|index| {
            let mut combination = Vec::new();
            for bit in 0..length {
                if (index & (1 << bit)) == (1 << bit) {
                    combination.push(Operator::Plus);
                } else {
                    combination.push(Operator::Times);
                }
            }
            combination
        })
        .collect();
    combinations
}

fn evaluate_equation(numbers: Vec<i128>, operators: Vec<Operator>) -> i128 {
    let mut result = numbers[0];
    for (number, operator) in std::iter::zip(numbers[1..].iter(), operators.iter()) {
        match operator {
            Operator::Plus => result += number,
            Operator::Times => result *= number,
        };
    }
    result
}
