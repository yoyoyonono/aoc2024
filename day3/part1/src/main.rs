fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("{}", input);
    let mul_indices: Vec<usize> = input.match_indices("mul(").map(|x| x.0).collect();
    println!("{:?}", mul_indices);
    let mut sum = 0;
    for mul_index in mul_indices {
        let start_index = mul_index + 4;
        // find comma
        let Some(comma_index) = input[start_index..].find(",") else {
            continue;
        };
        if comma_index > 3 || !input[start_index .. start_index + comma_index].chars().all(|x| x.is_digit(10)) {
            continue;
        }
        // find close paren
        let Some(close_paren_index) = input[start_index + comma_index + 1..].find(")") else {
            continue;
        };
        if close_paren_index > 3 || !input[start_index + comma_index + 1.. start_index + comma_index + 1 + close_paren_index].chars().all(|x| x.is_digit(10)) {
            continue;
        }
        let first_num: i32 = input[start_index..start_index + comma_index].parse().unwrap();
        let second_num: i32 = input[start_index + comma_index + 1..start_index + comma_index + 1 + close_paren_index].parse().unwrap();

        sum += first_num * second_num;

        println!("{} {} {} | {} {}", start_index, comma_index, close_paren_index, first_num, second_num);        
    }
    println!("{}", sum);
}
