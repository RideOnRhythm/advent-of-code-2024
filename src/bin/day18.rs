use std::collections::VecDeque;
use std::fs;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

fn bfs(grid: &[[bool; 71]; 71]) -> Option<i32> {
    let mut queue = VecDeque::new();
    queue.push_back((70, 70, 0));
    let mut visited = Vec::new();

    loop {
        let Some((x, y, depth)) = queue.pop_front() else {
            return None;
        };
        if x == 0 && y == 0 {
            return Some(depth);
        }
        if visited.contains(&(x, y)) || grid[y][x] {
            continue;
        }
        if y > 0 {
            queue.push_back((x, y - 1, depth + 1));
        }
        if y < 70 {
            queue.push_back((x, y + 1, depth + 1));
        }
        if x > 0 {
            queue.push_back((x - 1, y, depth + 1));
        }
        if x < 70 {
            queue.push_back((x + 1, y, depth + 1));
        }

        visited.push((x, y));
    }
}

fn main() {
    let input = fs::read_to_string("inputs/input18.txt").unwrap();
    let mut grid = [[false; 71]; 71];

    for (i, line) in input.lines().enumerate() {
        if i == 1024 {
            break;
        }
        let split = line.split_once(',').unwrap();
        let x: usize = split.0.parse().unwrap();
        let y: usize = split.1.parse().unwrap();
        grid[x][y] = true;
    }

    let steps = bfs(&grid).unwrap();
    println!("Minimum number of steps: {}", steps);

    let mut grids = Vec::new();
    grid = [[false; 71]; 71];
    for line in input.lines() {
        let split = line.split_once(',').unwrap();
        let x: usize = split.0.parse().unwrap();
        let y: usize = split.1.parse().unwrap();
        grid[x][y] = true;

        grids.push((x, y, grid));
    }

    let coordinates = grids.par_iter()
        .map(|(x, y, g)| (x, y, bfs(g)))
        .find_first(|(_, _, g)| g.is_none())
        .unwrap();
    println!("Coordinates: {},{}", coordinates.0, coordinates.1);
}