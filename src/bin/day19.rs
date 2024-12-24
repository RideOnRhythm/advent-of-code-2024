use std::collections::HashMap;
use std::fs;

fn dp(map: &mut HashMap<String, bool>, patterns: &Vec<&str>, design: String) -> bool {
    if map.contains_key(&design) {
        return map.get(&design).unwrap().clone();
    }

    for pattern in patterns {
        if design.starts_with(pattern) {
            if design[pattern.len()..].is_empty() {
                return true;
            }
            let result = dp(map, patterns, design[pattern.len()..].to_string());
            map.insert(design.clone(), result);
            if result {
                return true;
            }
        }
    }

    false
}

fn dp_2(map: &mut HashMap<String, u64>, patterns: &Vec<&str>, design: String) -> u64 {
    let mut total = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            if design[pattern.len()..].is_empty() {
                total += 1;
            }
            let result= if map.contains_key(&design) {
                *map.get(&design[pattern.len()..].to_string()).unwrap()
            } else {
                dp_2(map, patterns, design[pattern.len()..].to_string())
            };
            map.insert(design[pattern.len()..].to_string(), result);
            total += result;
        }
    }

    total
}

fn main() {
    let input = fs::read_to_string("inputs/input19.txt").unwrap();
    let mut patterns = Vec::new();
    let mut total = 0;
    let mut total_2: u64 = 0;
    let mut map: HashMap<String, bool> = HashMap::new();
    let mut map_2: HashMap<String, u64> = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }
        if line.contains(',') {
            patterns = line.split(", ").collect();
        } else {
            if dp(&mut map, &patterns, line.to_string()) {
                total += 1;
            }
            total_2 += dp_2(&mut map_2, &patterns, line.to_string())
        }
    }

    println!("Total number of possible designs: {}", total);
    println!("Total number of ways: {}", total_2);
}