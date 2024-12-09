fn main() {
    let input = include_str!("../input.txt");
    //let input = include_str!("../input_test.txt");
    let numbers: Vec<usize> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as usize)
        .collect();

    let mut system: Vec<Option<usize>> = numbers
        .iter()
        .enumerate()
        .map(|(index, number)| match index % 2 {
            0 => {
                vec![Some(index / 2); *number]
            }
            1 => {
                vec![None; *number]
            }
            _ => panic!(),
        })
        .flatten()
        .collect();

    let num_some = system.iter().filter(|x| x.is_some()).count();

    println!("{num_some}");

    while system.len() > num_some {
        if let Some(number) = system.pop().unwrap() {
            let first_none = system
                .iter()
                .enumerate()
                .filter(|(i, x)| x.is_none())
                .nth(0)
                .unwrap()
                .0;
            system[first_none] = Some(number);
        }

        // print_system(&system);
    }

    println!("{}", checksum(&system));
}

fn print_system(system: &Vec<Option<usize>>) {
    for x in system {
        match x {
            Some(number) => {
                print!("{number}");
            }
            None => {
                print!(".");
            }
        }
    }
    println!();
}

fn checksum(system: &Vec<Option<usize>>) -> usize {
    system
        .iter()
        .enumerate()
        .map(|(i, number)| i * number.unwrap())
        .sum()
}
