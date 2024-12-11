use core::num;

fn main() {
    // let input = include_str!("../input.txt");
    let input = include_str!("../input_test.txt");
    let mut stones: Vec<u128> = input.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    println!("{:?}", stones);
    for _ in 0..25 {
        stones = blink(stones);
        println!("{:?}", stones);
    }
    println!("{}", stones.len());
}

fn blink(stones: Vec<u128>) -> Vec<u128> {
    let mut new_stones = vec![];
    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
            continue;
        }


        let num_digits = (stone.checked_ilog10().unwrap_or(0) + 1);
        if num_digits % 2 == 0 {
            let ten_pow = (10_u128).pow(num_digits / 2);
            let left_half = stone / ten_pow;
            let right_half = stone % ten_pow;
            new_stones.push(left_half);
            new_stones.push(right_half);
            continue;
        }

        new_stones.push(stone * 2024); 
    }
    new_stones
}
