use std::collections::HashMap;
use std::fs;
use itertools::Itertools;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

fn secret_number(n: u64) -> u64 {
    let mask = 16777215;
    let n = ((n << 6) ^ n) & mask;
    let n = ((n >> 5) ^ n) & mask;
    let n = ((n << 11) ^ n) & mask;

    n
}

fn main() {
    let input = fs::read_to_string("inputs/input22.txt").unwrap();

    let mut total = 0;
    for line in input.lines() {
        let number = line.parse::<u64>().unwrap();
        let secret_number = (0..2000).fold(number, |s, _| secret_number(s));

        total += secret_number;
    }

    println!("Total: {}", total);

    let mut map: HashMap<&str, (Vec<i32>, Vec<i32>)> = HashMap::new();
    for line in input.lines() {
        let mut n = line.parse::<u64>().unwrap();
        let mut last_digits = vec![n as i32];
        let mut changes = Vec::new();
        for _ in 0..2000 {
            let secret_number = secret_number(n);
            let last_digit = secret_number % 10;

            changes.push(last_digit as i32 - last_digits.last().unwrap());
            last_digits.push(last_digit as i32);
            n = secret_number;
        }

        map.insert(line, (last_digits, changes));
    }

    let mut possible_changes = (0..4)
        .map(|_| [-9, -8, -7, -6, -5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9])
        .multi_cartesian_product();
    let max_total = possible_changes.par_bridge().map(|price_changes| {
        let no_zeros: Vec<i32> = price_changes.clone().into_iter()
            .filter(|&x| x != 0)
            .collect();
        for win in no_zeros.windows(3) {
            if -9 + win[0] + win[1] + win[2] > 9 || 9 - win[0] - win[1] - win[2] < -9 {
                return 0;
            }
        }

        let mut total = 0;
        for line in input.lines() {
            let (last_digits, changes) = map.get(line).unwrap();
            for (index, change) in changes.windows(4).enumerate() {
                if change == price_changes {
                    total += last_digits[index + 4];
                    break;
                }
            }
        }

        total
    }).max().unwrap();

    println!("Maximum number of bananas: {}", max_total);
}