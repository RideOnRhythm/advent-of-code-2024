use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input2.txt").unwrap();
    let mut safe1 = 0;
    let mut safe2 = 0;

    for i in input.lines() {
        let nums: Vec<i32> = i
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();

        let mut diffs: Vec<i32> = nums
            .windows(2)
            .map(|w| w[1] - w[0])
            .collect();
        if diffs[0] < 0 {
            diffs = diffs
                .into_iter()
                .map(|x| x * -1)
                .collect();
        }
        if diffs.into_iter().all(|x| [1, 2, 3].contains(&x) && x > 0) {
            safe1 += 1
        }

        for i in 0..nums.len() {
            let mut nums_copy = nums.clone();
            nums_copy.remove(i);
            let mut diffs: Vec<i32> = nums_copy
                .windows(2)
                .map(|w| w[1] - w[0])
                .collect();
            if diffs[0] < 0 {
                diffs = diffs
                    .into_iter()
                    .map(|x| x * -1)
                    .collect();
            }
            if diffs.into_iter().all(|x| [1, 2, 3].contains(&x) && x > 0) {
                safe2 += 1;
                break
            }
        }
    }

    println!("{}", safe1);
    println!("{}", safe2);
}