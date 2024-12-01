fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();
    for line in input.lines() {
        let mut numbers = line.split_ascii_whitespace();
        first_list.push(numbers.next().unwrap().parse::<i32>().unwrap());
        second_list.push(numbers.next().unwrap().parse::<i32>().unwrap());
    }
    first_list.sort();
    second_list.sort();
    println!("{:?}", first_list);
    println!("{:?}", second_list);
    let sum = std::iter::zip(first_list, second_list)
        .map(|(a, b)| (a - b).abs())
        .sum::<i32>();
    println!("{}", sum);
}
