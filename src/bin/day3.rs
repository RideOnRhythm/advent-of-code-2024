use std::fs;
use regex::{Captures, Regex};

fn main() {
    let input = fs::read_to_string("inputs/input3.txt").unwrap();
    let mut total = 0;

    let mul = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let do1 = Regex::new(r"do\(\)").unwrap();
    let dont = Regex::new(r"don't\(\)").unwrap();
    let mut results: Vec<Captures> = mul.captures_iter(&input)
        .chain(do1.captures_iter(&input))
        .chain(dont.captures_iter(&input))
        .collect();
    results.sort_by(|a, b| {
       a.get(0).unwrap().start().cmp(&b.get(0).unwrap().start())
    });

    let mut inst = true;
    for m in results {
        let str = m.get(0).unwrap().as_str();
        match str {
            "do()" => inst = true,
            "don't()" => inst = false,
            _ => ()
        }
        if str.starts_with("mul(") && inst {
            let num1 = m.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let num2 = m.get(2).unwrap().as_str().parse::<i32>().unwrap();
            total += num1 * num2;
        }
    }

    println!("{}", total);
}