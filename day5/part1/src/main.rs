fn main() {
    let input = include_str!("../input.txt");

    println!("{input}");

    let input_split: Vec<&str> = input.split("\n\n").collect();

    let ordering_rules: Vec<[i32; 2]> = input_split[0]
        .lines()
        .map(|x| {
            let numbers: Vec<i32> = x.split('|').map(|n| n.parse().unwrap()).collect();
            [numbers[0], numbers[1]]
        })
        .collect();

    let updates: Vec<Vec<i32>> = input_split[1]
        .lines()
        .map(|line| line.split(',').map(|n| n.parse::<i32>().unwrap()).collect())
        .collect();

    let sum: i32 = updates
        .iter()
        .map(|update| {
            if is_correct(update, &ordering_rules) {
                update[update.len() / 2]
            } else {
                0
            }
        })
        .sum();

    println!("{sum}")
}

fn is_correct(update: &Vec<i32>, ordering_rules: &Vec<[i32; 2]>) -> bool {
    for rule in ordering_rules {
        if !(update.contains(&rule[0]) && update.contains(&rule[1])) {
            continue;
        }
        if update.iter().position(|x| *x == rule[0]).unwrap()
            > update.iter().position(|x| *x == rule[1]).unwrap()
        {
            return false;
        }
    }
    true
}
