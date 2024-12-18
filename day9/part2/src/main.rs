use std::collections::HashSet;

fn main() {
    // let input = include_str!("../input.txt");
    let input = include_str!("../input_test.txt");
    // let input = include_str!("../input_bonus.txt");
    // let input = include_str!("../input_bonus2.txt");
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

    println!("{num_some} | {}", system.len());

    let mut to_check = system.len() - 1;

    // print_system(&system);

    let mut settled = HashSet::new();

    'outer: while system.len() > num_some {
        println!("{to_check} \t {}", system[to_check].unwrap_or_default());
        let mut changes = Vec::new();
        if let Some(number) = system[to_check] {
            changes.push(to_check);
            to_check -= 1;

            let mut len = 1;
            // find length
            while let Some(x) = system[to_check] {
                changes.push(to_check);
                if to_check == 0 {
                    break 'outer;
                }
                to_check -= 1;
                if x != number {
                    to_check += 1;
                    changes.pop();
                    break;
                }
                len += 1;
            }

            if settled.contains(&number) {
                continue;
            }
            // find first bank of nones long enough
            if let Some(gap_index) = find_gap(&system, len) {
                if gap_index > to_check {
                    continue;
                }
                for i in gap_index..gap_index + len {
                    system[i] = Some(number);
                }
                settled.insert(number);
            } else {
                continue;
            }
            make_changes(&mut system, changes);
        } else {
            to_check -= 1;
        }

        if to_check == 0 {
            break;
        }

        // print_system(&system);
    }

    println!("{}", checksum(&system));
}

fn make_changes(system: &mut Vec<Option<usize>>, changes: Vec<usize>) {
    for change in changes {
        system[change] = None;
    }
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
        .map(|(i, number)| match number {
            Some(x) => i * x,
            None => 0,
        })
        .sum()
}

fn find_gap(system: &Vec<Option<usize>>, target_gap: usize) -> Option<usize> {
    let mut gap_size = 0;

    for (i, x) in system.iter().enumerate() {
        match x {
            Some(_) => gap_size = 0,
            None => gap_size += 1,
        }
        if gap_size == target_gap {
            return Some(i - gap_size + 1);
        }
    }
    None
}
