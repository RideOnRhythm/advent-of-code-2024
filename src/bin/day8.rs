use std::fs;
use itertools::Itertools;

struct Antenna {
    frequency: char,
    x: i32,
    y: i32
}

fn find_antinodes(grid: &Vec<Vec<char>>, a: &Antenna, b: &Antenna) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();
    if a.frequency != b.frequency {
        return antinodes;
    }
    let b_offset = (b.x - a.x, b.y - a.y);
    let antinode_a = (a.x - b_offset.0, a.y - b_offset.1);
    let antinode_b = (b.x + b_offset.0, b.y + b_offset.1);
    for antinode in [antinode_a, antinode_b].into_iter() {
        if 0 <= antinode.0 && antinode.0 < grid.len() as i32 && 0 <= antinode.1 && antinode.1 < grid[0].len() as i32 {
            antinodes.push(antinode);
        }
    }

    antinodes
}

fn find_new_antinodes(grid: &Vec<Vec<char>>, a: &Antenna, b: &Antenna) -> Vec<(i32, i32)> {
    let mut antinodes = Vec::new();
    if a.frequency != b.frequency {
        return antinodes;
    }
    antinodes.push((a.x, a.y));
    antinodes.push((b.x, b.y));
    let b_offset = (b.x - a.x, b.y - a.y);
    let mut antinode = (a.x, a.y);
    loop {
        antinode = (antinode.0 - b_offset.0, antinode.1 - b_offset.1);
        if 0 <= antinode.0 && antinode.0 < grid.len() as i32 && 0 <= antinode.1 && antinode.1 < grid[0].len() as i32 {
            antinodes.push(antinode);
        } else {
            break;
        }
    }
    antinode = (b.x, b.y);
    loop {
        antinode = (antinode.0 + b_offset.0, antinode.1 + b_offset.1);
        if 0 <= antinode.0 && antinode.0 < grid.len() as i32 && 0 <= antinode.1 && antinode.1 < grid[0].len() as i32 {
            antinodes.push(antinode);
        } else {
            break;
        }
    }

    antinodes
}

fn main() {
    let input = fs::read_to_string("inputs/input8.txt").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antennas = Vec::new();
    let mut antinode_locations = Vec::new();
    let mut new_antinode_locations = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let c = grid[i][j];
            if c != '.' {
                antennas.push(Antenna { frequency: c, x: i as i32, y: j as i32 });
            }
        }
    }
    for pair in antennas.iter().combinations(2) {
        let (a, b) = (pair[0], pair[1]);
        let antinodes= find_antinodes(&grid, a, b);
        for n in antinodes.iter() {
            if !antinode_locations.contains(n) {
                antinode_locations.push(*n);
            }
        }
        let new_antinodes = find_new_antinodes(&grid, a, b);
        for n in new_antinodes.iter() {
            if !new_antinode_locations.contains(n) {
                new_antinode_locations.push(*n);
            }
        }
    }

    println!("Antinode locations: {}", antinode_locations.len());
    println!("New antinode locations: {}", new_antinode_locations.len());
}