use std::fs;

fn find_a_values() -> u128 {
    let mut all_values = vec![0];
    for &inst in [2,4,1,5,7,5,0,3,4,0,1,6,5,5,3,0].iter().rev() {
        let mut valid_as = Vec::new();
        for a in all_values.iter() {
            for j in 0u128..8 {
                let a: u128 = (a << 3) + j;
                let output = ((a % 8 ^ 5) ^ (a >> (a % 8 ^ 5)) ^ 6) % 8;

                if output == inst {
                    valid_as.push(a);
                }
            }
        }
        all_values = valid_as.drain(..).collect();
    }

    all_values.iter().min().unwrap().clone()
}

fn main() {
    let input = fs::read_to_string("inputs/input17.txt").unwrap();

    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut pointer = 0;
    let mut instructions: Vec<u128> = Vec::new();

    for line in input.lines() {
        if line.contains("Register A") {
            register_a += line.split(": ").nth(1).unwrap().parse::<u128>().unwrap();
        } else if line.contains("Register B") {
            register_b += line.split(": ").nth(1).unwrap().parse::<u128>().unwrap();
        } else if line.contains("Register C") {
            register_c += line.split(": ").nth(1).unwrap().parse::<u128>().unwrap();
        }

        if line.contains("Program") {
            let split = line.split(": ").nth(1).unwrap();
            instructions = split.split(",").map(|i| i.parse().unwrap()).collect();
        }
    }

    let mut output = Vec::new();
    loop {
        if pointer >= instructions.len() {
            break;
        }

        let instruction: u128 = instructions[pointer];
        let literal: u128 = instructions[pointer + 1];
        let combo: u128 = match literal {
            0 | 1 | 2 | 3 => literal,
            4 => register_a,
            5 => register_b,
            6 => register_c,
            _ => break
        };

        if instruction == 0 {
            register_a >>= combo;
        } else if instruction == 1 {
            register_b ^= literal;
        } else if instruction == 2 {
            register_b = combo % 8;
        } else if instruction == 3 && register_a != 0 {
            pointer = literal as usize;
        } else if instruction == 4 {
            register_b ^= register_c;
        } else if instruction == 5 {
            output.push(combo % 8)
        } else if instruction == 6 {
            register_b = register_a >> combo;
        } else if instruction == 7 {
            register_c = register_a >> combo;
        }

        if instruction != 3 {
            pointer += 2;
        } else if register_a == 0 {
            pointer += 2;
        }
    }

    println!("Output: {}", output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(","));

    let min = find_a_values();
    println!("Lowest positive initial value for register A that causes the program to output a copy of itself: {}", min);
}