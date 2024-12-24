use std::collections::HashMap;
use std::fs;
use std::iter;

fn num_digits(n: u64) -> u32 {
    iter::successors(Some(n), |&n| (n >= 10).then(|| n / 10)).count() as u32
}

fn increment(stones: &mut HashMap<u64, u64>, key: u64, value: u64) {
    let key1 = stones.entry(key).or_insert(0);
    *key1 += value;
}

fn decrement(stones: &mut HashMap<u64, u64>, key: u64, value: u64) {
    let key1 = stones.entry(key).or_insert(0);
    *key1 -= value;
}

fn blink(stones: &mut HashMap<u64, u64>) {
    for (&stone, &count) in stones.clone().iter() {
        let digits = num_digits(stone);
        if stone == 0 {
            decrement(stones, stone, count);
            increment(stones, 1, count);
        } else if digits % 2 == 0 {
            let pow = 10_u64.pow(digits / 2);
            let first_half = stone / pow;
            let second_half = stone - (first_half * pow);
            decrement(stones, stone, count);
            increment(stones, first_half, count);
            increment(stones, second_half, count);
        } else {
            decrement(stones, stone, count);
            increment(stones, stone * 2024, count);
        }
    }
}

fn num_stones(stones: &HashMap<u64, u64>) -> u64 {
    stones.values().sum()
}

fn main() {
    let input = fs::read_to_string("inputs/input11.txt").unwrap();
    let mut stones: HashMap<u64, u64> = HashMap::new();
    for s in input.split_whitespace() {
        stones.insert(s.parse().unwrap(), 1);
    }
    for _ in 0..25 {
        blink(&mut stones);
    }

    println!("Number of stones after 25 blinks: {}", num_stones(&stones));

    for _ in 0..(75 - 25) {
        blink(&mut stones);
    }

    println!("Number of stones after 75 blinks: {}", num_stones(&stones));
}