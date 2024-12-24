use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input1.txt").unwrap();
    let mut vec1: Vec<i32> = Vec::new();
    let mut vec2: Vec<i32> = Vec::new();
    for line in input.lines() {
        let mut split = line.split_whitespace();
        let num1: i32 = split.next().unwrap().parse().unwrap();
        let num2: i32 = split.next().unwrap().parse().unwrap();
        vec1.push(num1);
        vec2.push(num2);
    }
    vec1.sort();
    vec2.sort();

    let mut total1 = 0;
    for it in vec1.iter().zip(vec2.iter()) {
        let (num1, num2) = it;
        total1 += (num1 - num2).abs();
    }

    println!("{}", total1);

    let mut total2 = 0;
    for i in vec1.iter() {
        total2 += i * vec2.iter().filter(|x| **x == *i).count() as i32;
    }

    println!("{}", total2);
}