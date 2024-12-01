fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();
    for line in input.lines() {
        let mut numbers = line.split_ascii_whitespace();
        first_list.push(numbers.next().unwrap().parse::<i32>().unwrap());
        second_list.push(numbers.next().unwrap().parse::<i32>().unwrap());
    }
    println!("{:?}", first_list);
    println!("{:?}", second_list);
    let sum = first_list.iter().map(|x| second_list.iter().filter(|a| *a == x).count() as i32 * x).sum::<i32>();
    println!("{}", sum);
}
