use std::collections::{HashMap, VecDeque};
use std::fs;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Node {
    x: usize,
    y: usize
}

fn bfs(grid: &Vec<Vec<char>>, start: &Node, end: &Node) -> (i32, HashMap<Node, i32>) {
    let mut queue: VecDeque<(Node, i32)> = VecDeque::new();
    let mut dist: HashMap<Node, i32> = HashMap::new();
    queue.push_back((start.clone(), 0));

    loop {
        let Some((node, d)) = queue.pop_front() else {
            break;
        };
        if grid[node.y][node.x] == '#' {
            continue;
        }
        if dist.contains_key(&node) {
            let current_dist = dist.get(&node).unwrap().clone();
            if d > current_dist {
                continue;
            } else {
                dist.insert(node.clone(), d);
            }
        }

        queue.push_back((Node { x: node.x, y: node.y - 1 }, d + 1));
        queue.push_back((Node { x: node.x, y: node.y + 1 }, d + 1));
        queue.push_back((Node { x: node.x - 1, y: node.y }, d + 1));
        queue.push_back((Node { x: node.x + 1, y: node.y }, d + 1));

        dist.insert(node, d);
    }

    (dist.get(&end).unwrap().clone(), dist)
}

fn main() {
    let input = fs::read_to_string("inputs/input20.txt").unwrap();

    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut start_node = Node{ x: 0, y: 0 };
    let mut end_node = Node { x: 0, y: 0 };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start_node = Node { x, y };
            } else if c == 'E' {
                end_node = Node { x, y };
            }
        }
        grid.push(line.chars().collect());
    }

    let (no_cheats, dist) = bfs(&grid, &start_node, &end_node);
    let mut cheats = Vec::new();
    let mut end_cheats: HashMap<Node, i32> = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cheat = Node{ x, y };
            if !dist.contains_key(&cheat) {
                continue;
            }
            for y2 in 0..grid.len() {
                for x2 in 0..grid[y].len() {
                    let end_cheat = Node{ x: x2, y: y2 };
                    if !dist.contains_key(&end_cheat) {
                        continue;
                    }
                    let cheat_pico = end_cheat.x.abs_diff(cheat.x) + end_cheat.y.abs_diff(cheat.y);
                    if cheat_pico > 2 {
                        continue;
                    }
                    let start_dist = dist.get(&cheat).unwrap() + cheat_pico as i32;
                    let shortest = if end_cheats.contains_key(&end_cheat) {
                        *end_cheats.get(&end_cheat).unwrap()
                    } else {
                        bfs(&grid, &end_cheat, &end_node).0
                    };

                    end_cheats.insert(end_cheat, shortest);
                    cheats.push(shortest + start_dist);
                }
            }
        }
    }

    let cheats_count = cheats.iter()
        .filter(|&&c| no_cheats - c >= 100)
        .count();
    println!("{:?}", cheats_count);

    let (no_cheats, dist) = bfs(&grid, &start_node, &end_node);
    let mut cheats = Vec::new();
    let mut end_cheats: HashMap<Node, i32> = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let cheat = Node{ x, y };
            if !dist.contains_key(&cheat) {
                continue;
            }
            for y2 in 0..grid.len() {
                for x2 in 0..grid[y].len() {
                    let end_cheat = Node{ x: x2, y: y2 };
                    if !dist.contains_key(&end_cheat) {
                        continue;
                    }
                    let cheat_pico = end_cheat.x.abs_diff(cheat.x) + end_cheat.y.abs_diff(cheat.y);
                    if cheat_pico > 20 {
                        continue;
                    }
                    let start_dist = dist.get(&cheat).unwrap() + cheat_pico as i32;
                    let shortest = if end_cheats.contains_key(&end_cheat) {
                        *end_cheats.get(&end_cheat).unwrap()
                    } else {
                        bfs(&grid, &end_cheat, &end_node).0
                    };

                    end_cheats.insert(end_cheat, shortest);
                    cheats.push(shortest + start_dist);
                }
            }
        }
    }

    let cheats_count = cheats.iter()
        .filter(|&&c| no_cheats - c >= 100)
        .count();
    println!("{:?}", cheats_count);
}