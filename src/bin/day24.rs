use std::collections::HashMap;
use std::fs;
use itertools::Itertools;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
enum Gate {
    AND,
    XOR,
    OR
}

fn find_z_wires(wires: &mut HashMap<String, bool>, gates: &HashMap<(String, Gate, String), Vec<String>>) -> u64 {
    while gates.values().flatten().any(|x| !wires.contains_key(x)) {
        for ((w1, gate, w2), vw) in gates.iter() {
            if wires.contains_key(w1) && wires.contains_key(w2) {
                let val1 = wires.get(w1).unwrap();
                let val2 = wires.get(w2).unwrap();
                let val3 = match gate {
                    Gate::AND => *val1 && *val2,
                    Gate::XOR => *val1 ^ *val2,
                    Gate::OR => *val1 || *val2,
                };
                for w3 in vw {
                    wires.insert(w3.clone(), val3);
                }
            }
        }
    }

    let mut z_wires: Vec<&String> = wires.keys().filter(|x| x.starts_with("z")).collect();
    z_wires.sort();
    let mut z_number: u64 = 0;
    for (i, wire) in z_wires.into_iter().enumerate() {
        let val = wires.get(wire).unwrap();
        if *val {
            z_number += 2_u64.pow(i as u32);
        }
    }

    z_number
}

fn main() {
    let input = fs::read_to_string("inputs/input24.txt").unwrap();

    let mut wires: HashMap<String, bool> = HashMap::new();
    let mut gates: HashMap<(String, Gate, String), Vec<String>> = HashMap::new();
    for line in input.lines() {
        if line.contains(":") {
            let split: Vec<&str> = line.split(": ").collect();
            let val = if split[1] == "1" {
                true
            } else {
                false
            };
            wires.insert(split[0].to_string(), val);
        } else if line.contains("->") {
            let split: Vec<&str> = line.split(" -> ").collect();
            let wires: Vec<&str> = split[0].split(" ").collect();
            let gate = match wires[1] {
                "AND" => Gate::AND,
                "XOR" => Gate::XOR,
                "OR" => Gate::OR,
                _ => Gate::OR
            };

            let key = gates.entry((wires[0].to_string(), gate, wires[2].to_string())).or_insert(vec![]);
            key.push(split[1].to_string());
        }
    }

    let z_number = find_z_wires(&mut wires.clone(), &gates.clone());
    println!("Z number: {}", z_number);
}