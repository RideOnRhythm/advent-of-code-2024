use std::collections::{HashMap, HashSet};
use std::fs;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn main() {
    let input = fs::read_to_string("inputs/input6.txt").unwrap();

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut guard_pos: (i32, i32) = (0, 0);
    let mut guard_direction = Direction::Up;
    for i in 0..grid.len() {
        if grid[i].contains(&'^') {
            guard_pos = (i as i32, grid[i].iter().position(|&c| c == '^').unwrap() as i32);
            break;
        }
    }
    let mut visited: Vec<(i32, i32)> = vec![guard_pos];

    loop {
        let new_pos = match guard_direction {
            Direction::Up => (guard_pos.0 - 1, guard_pos.1),
            Direction::Right => (guard_pos.0, guard_pos.1 + 1),
            Direction::Down => (guard_pos.0 + 1, guard_pos.1),
            Direction::Left => (guard_pos.0, guard_pos.1 - 1),
        };
        if new_pos.0 < 0 || new_pos.0 >= grid[0].len() as i32 || new_pos.1 < 0 || new_pos.1 >= grid.len() as i32 {
            break;
        }
        if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            guard_direction = match guard_direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            if !visited.contains(&new_pos) {
                visited.push(new_pos);
            }
            guard_pos = new_pos;
        }
    }
    let v_clone = visited.clone();

    let total: usize = (0..grid.len()).into_par_iter()
        .map(|i| {
            let total = (0..grid[i].len()).into_par_iter()
                .filter(|&j| {
                    if !v_clone.contains(&(i as i32, j as i32)) {
                        return false;
                    }

                    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
                    let mut guard_direction = Direction::Up;
                    let mut guard_pos: (i32, i32) = (0, 0);
                    for i in 0..grid.len() {
                        if grid[i].contains(&'^') {
                            guard_pos = (i as i32, grid[i].iter().position(|&c| c == '^').unwrap() as i32);
                            break;
                        }
                    }
                    let mut direction_map: HashMap<Direction, HashSet<(i32, i32)>> = HashMap::new();
                    direction_map.insert(Direction::Up, HashSet::new());
                    direction_map.insert(Direction::Right, HashSet::new());
                    direction_map.insert(Direction::Down, HashSet::new());
                    direction_map.insert(Direction::Left, HashSet::new());

                    if grid[i][j] != '.' {
                        return false;
                    }
                    grid[i][j] = '#';

                    loop {
                        let new_pos = match guard_direction {
                            Direction::Up => (guard_pos.0 - 1, guard_pos.1),
                            Direction::Right => (guard_pos.0, guard_pos.1 + 1),
                            Direction::Down => (guard_pos.0 + 1, guard_pos.1),
                            Direction::Left => (guard_pos.0, guard_pos.1 - 1),
                        };
                        if new_pos.0 < 0 || new_pos.0 >= grid[0].len() as i32 || new_pos.1 < 0 || new_pos.1 >= grid.len() as i32 {
                            return false;
                        }
                        if direction_map.get(&guard_direction).unwrap().contains(&new_pos) {
                            return true;
                        }
                        if grid[new_pos.0 as usize][new_pos.1 as usize] == '#' {
                            guard_direction = match guard_direction {
                                Direction::Up => Direction::Right,
                                Direction::Right => Direction::Down,
                                Direction::Down => Direction::Left,
                                Direction::Left => Direction::Up,
                            };
                        } else {
                            guard_pos = new_pos;
                            direction_map.get_mut(&guard_direction).unwrap().insert(new_pos);
                        }
                    }
                })
                .count();
            total
        })
        .sum();

    println!("{}", visited.len());
    println!("{}", total);
}