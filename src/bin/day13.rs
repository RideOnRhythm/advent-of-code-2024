use std::fs;
use std::str::Split;

fn calculate_tokens(machines: Split<&str>, extra: i64) -> i64 {
    let mut tokens = 0;

    for machine in machines {
        let mut split = machine.split("\n");
        let mut button_a = split
            .next().unwrap()
            .split(": ")
            .nth(1).unwrap()
            .split(", ");
        let ax = button_a.next().unwrap()[2..].parse::<i64>().unwrap();
        let ay = button_a.next().unwrap()[2..].parse::<i64>().unwrap();
        let mut button_b = split
            .next().unwrap()
            .split(": ")
            .nth(1).unwrap()
            .split(", ");
        let bx = button_b.next().unwrap()[2..].parse::<i64>().unwrap();
        let by = button_b.next().unwrap()[2..].parse::<i64>().unwrap();
        let mut prize = split
            .next().unwrap()
            .split(": ")
            .nth(1).unwrap()
            .split(", ");
        let px = prize.next().unwrap()[2..].parse::<i64>().unwrap() + extra;
        let py = prize.next().unwrap()[2..].parse::<i64>().unwrap() + extra;

        let push_a = (by * px - bx * py) as f64 / (by * ax - bx * ay) as f64;
        if push_a.fract() != 0.0 {
            continue;
        }
        let push_a = push_a as i64;
        let push_b = (-ax * push_a + px) as f64 / bx as f64;
        if push_b.fract() != 0.0 {
            continue;
        }
        let push_b = push_b as i64;

        tokens += 3 * push_a + push_b;
    }

    tokens
}

fn main() {
    let input = fs::read_to_string("inputs/input13.txt").unwrap();
    let machines = input.split("\n\n");

    println!("Tokens: {}", calculate_tokens(machines.clone(), 0));
    println!("Tokens with increased prizes: {}", calculate_tokens(machines.clone(), 10000000000000));
}