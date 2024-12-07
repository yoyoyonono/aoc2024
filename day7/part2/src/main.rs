use rayon::iter::*;
use std::fmt::format;

#[derive(Debug)]
struct Equation {
    target: i128,
    numbers: Vec<i128>,
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Plus,
    Times,
    Concatenate,
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

    let sum: i128 = equations
        .par_iter()
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
    let mut combinations = Vec::new();
    let base: i128 = 3;
    for index in 0..base.pow(length.try_into().unwrap()) {
        let mut combination = Vec::new();
        let mut number = index;
        while {
            combination.push(match number % base {
                0 => Operator::Plus,
                1 => Operator::Times,
                2 => Operator::Concatenate,
                _ => panic!(),
            });
            number /= base;
            number > 1
        } {}
        if combination.len() < length {
            for _ in 0..length - combination.len() {
                combination.insert(0, Operator::Plus);
            }
        }
        combinations.push(combination);
    }
    combinations
}

fn evaluate_equation(numbers: Vec<i128>, operators: Vec<Operator>) -> i128 {
    let mut result = numbers[0];
    for (number, operator) in std::iter::zip(numbers[1..].iter(), operators.iter()) {
        match operator {
            Operator::Plus => result += number,
            Operator::Times => result *= number,
            Operator::Concatenate => result = format!("{result}{number}").parse().unwrap(),
        };
    }
    result
}
