use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input5.txt").unwrap();
    let mut total = 0;
    let mut total2 = 0;

    let mut rules: Vec<(i32, i32)> = Vec::new();
    for line in input.lines() {
        if !line.contains('|') {
            continue;
        }
        let mut split = line.split('|');
        rules.push((
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap()
        ));
    }
    for line in input.lines() {
        if !line.contains(',') {
            continue;
        }
        let mut nums: Vec<i32> = line.split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        if rules.iter()
            .any(|(n1, n2)| {
                if !(nums.contains(n1) && nums.contains(n2)) {
                    return false;
                }
                nums.iter().position(|&x| x == *n1).unwrap() > nums.iter().position(|&x| x == *n2).unwrap()
            }) {
            continue;
        }
        total += nums[(nums.len() - 1) / 2];
    }

    for line in input.lines() {
        let mut swapped = false;
        if !line.contains(',') {
            continue;
        }
        let mut nums: Vec<i32> = line.split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        while rules.iter()
            .any(|(n1, n2)| {
                if !(nums.contains(n1) && nums.contains(n2)) {
                    return false;
                }
                nums.iter().position(|&x| x == *n1).unwrap() > nums.iter().position(|&x| x == *n2).unwrap()
            }) {
            for (n1, n2) in rules.iter() {
                if !(nums.contains(n1) && nums.contains(n2)) {
                    continue;
                }
                let i1 = nums.iter().position(|&x| x == *n1).unwrap();
                let i2 = nums.iter().position(|&x| x == *n2).unwrap();
                if i1 > i2 {
                    swapped = true;
                    nums.swap(i1, i2);
                }
            }
        }
        if swapped {
            total2 += nums[(nums.len() - 1) / 2];
        }
    }

    println!("{}", total);
    println!("{}", total2);
}