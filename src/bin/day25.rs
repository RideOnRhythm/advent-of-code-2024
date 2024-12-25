use std::fs;
use itertools::Itertools;

#[derive(Debug, Clone)]
enum Schematic {
    Lock(Vec<Vec<char>>),
    Key(Vec<Vec<char>>)
}

fn grid_to_heights(schematic: Schematic) -> Vec<u32> {
    let grid = match schematic {
        Schematic::Lock(v) => v,
        Schematic::Key(v) => v.into_iter().rev().collect()
    };
    let mut heights: Vec<u32> = Vec::new();
    for i in 0..grid[0].len() {
        let column: Vec<char> = (0..grid.len())
            .map(|j| grid[j][i])
            .collect();
        let position = column.iter().position(|&c| c == '.').unwrap() - 1;
        heights.push(position as u32);
    }

    heights
}

fn main() {
    let input = fs::read_to_string("inputs/input25.txt").unwrap();

    let mut locks: Vec<Vec<u32>> = Vec::new();
    let mut keys: Vec<Vec<u32>> = Vec::new();
    let mut current_schematic: Option<Schematic> = None;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if current_schematic.is_none() && line == "#####" {
            current_schematic = Some(Schematic::Lock(Vec::new()));
        } else if current_schematic.is_none() {
            current_schematic = Some(Schematic::Key(Vec::new()));
        }

        let grid = match current_schematic {
            Some(Schematic::Lock(ref mut v)) | Some(Schematic::Key(ref mut v)) => v,
            _ => unreachable!()
        };
        grid.push(line.chars().collect());
        if grid.len() == 7 {
            let heights = grid_to_heights(current_schematic.clone().unwrap());
            match current_schematic {
                Some(Schematic::Lock(_)) => locks.push(heights),
                Some(Schematic::Key(_)) => keys.push(heights),
                None => ()
            }
            current_schematic = None;
            continue;
        }
    }

    let pairs = locks.iter().cartesian_product(keys.iter());
    let mut total = 0;
    for pair in pairs {
        let mut sums = pair.0.iter()
            .zip(pair.1)
            .map(|(a, b)| a + b);
        if sums.any(|x| x >= 6) {
            continue;
        } else {
            total += 1;
        }
    }

    println!("Total: {}", total);
}