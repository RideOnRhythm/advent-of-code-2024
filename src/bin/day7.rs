use std::fs;
use std::iter::successors;
use itertools::Itertools;

#[derive(Clone, Debug)]
enum Operations {
    Add,
    Multiply,
    Concatenate
}

fn concat(a: i64, b: i64) -> i64 {
    a * 10_i64.pow(successors(Some(b), |&b| (b >= 10).then(|| b / 10)).count() as u32) + b
}

fn main() {
    let input = fs::read_to_string("inputs/input7.txt").unwrap();
    let mut total = 0;
    let mut total2 = 0;

    for line in input.lines() {
        let mut split = line.split(": ");
        let result: i64 = split.next().unwrap().parse().unwrap();
        let vals: Vec<i64> = split
            .next()
            .unwrap()
            .split(" ")
            .map(|x| x.parse::<i64>().unwrap())
            .collect();
        let ops = vec![Operations::Add, Operations::Multiply];
        let n = vals.len() - 1;
        let combinations: Vec<_> = (0..n).map(|_| ops.clone())
            .multi_cartesian_product()
            .collect();
        for c in combinations {
            let mut running = vals[0];
            for i in 0..c.len() {
                let op = &c[i];
                match op {
                    Operations::Add => running += vals[i + 1],
                    Operations::Multiply => running *= vals[i + 1],
                    _ => ()
                }
            }
            if running == result {
                total += result;
                break;
            }
        }

        let ops = vec![Operations::Add, Operations::Multiply, Operations::Concatenate];
        let combinations: Vec<_> = (0..n).map(|_| ops.clone())
            .multi_cartesian_product()
            .collect();
        for c in combinations {
            let mut running = vals[0];
            for i in 0..c.len() {
                let op = &c[i];
                match op {
                    Operations::Add => running += vals[i + 1],
                    Operations::Multiply => running *= vals[i + 1],
                    Operations::Concatenate => running = concat(running, vals[i + 1]),
                }
            }
            if running == result {
                total2 += result;
                break;
            }
        }
    }

    println!("{}", total);
    println!("{}", total2);
}