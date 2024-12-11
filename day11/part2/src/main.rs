#![recursion_limit = "512"]
use memoize::memoize;
use rayon::iter::*;

fn main() {
    let input = include_str!("../input.txt");
    // let input = include_str!("../input_test.txt");
    let mut stones: Vec<u128> = input
        .split_ascii_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    println!("{:?}", stones);

    let count: usize = stones.par_iter().map(|x| blink_count(*x, 75)).sum();

    println!("{count}");
}

#[memoize(SharedCache)]
fn blink_stone(stone: u128) -> Vec<u128> {
    if stone == 0 {
        return vec![1];
    }

    let num_digits = (stone.checked_ilog10().unwrap_or(0) + 1);
    if num_digits % 2 == 0 {
        let ten_pow = (10_u128).pow(num_digits / 2);
        let left_half = stone / ten_pow;
        let right_half = stone % ten_pow;
        return vec![left_half, right_half];
    }

    vec![stone * 2024]
}

#[memoize(Capacity: 65535)]
fn blink_count(stone: u128, level: u32) -> usize {
    if level == 0 {
        return 1;
    }
    return blink_stone(stone)
        .iter()
        .map(|x| blink_count(*x, level - 1))
        .sum();
}
