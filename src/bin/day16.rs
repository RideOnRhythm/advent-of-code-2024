use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;
use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Node {
    x: usize,
    y: usize,
    direction: Direction
}

fn bfs(grid: &Vec<Vec<char>>, start: Node) -> (i32, usize) {
    let mut queue = VecDeque::new();
    queue.push_back((start.clone(), 0, start.clone()));
    let mut distances: HashMap<Node, (i32, Vec<Node>)> = HashMap::new();
    distances.insert(start.clone(), (0, vec![]));
    let mut end_node = Node {
        x: 0,
        y: 0,
        direction: Direction::Up,
    };

    loop {
        if queue.is_empty() {
            break;
        }
        let (node, dist, parent) = queue.pop_front().unwrap();
        if grid[node.y][node.x] == '#' {
            continue;
        }
        if distances.contains_key(&node) {
            if dist > distances.get(&node).unwrap().0 {
                continue;
            } else if dist == distances.get(&node).unwrap().0 {
                distances.get_mut(&node).unwrap().1.push(parent.clone());
            } else {
                distances.insert(node.clone(), (dist, vec![parent.clone()]));
            }
        }
        if grid[node.y][node.x] == 'E' {
            end_node = node.clone();
            distances.insert(node, (dist, vec![parent]));
            continue;
        }

        if node.direction != Direction::Down {
            let d = {
                if node.direction == Direction::Up {
                    dist + 1
                } else {
                    dist + 1001
                }
            };
            queue.push_back((Node { x: node.x, y: node.y - 1, direction: Direction::Up }, d, node.clone()));
        }
        if node.direction != Direction::Up {
            let d = {
                if node.direction == Direction::Down {
                    dist + 1
                } else {
                    dist + 1001
                }
            };
            queue.push_back((Node { x: node.x, y: node.y + 1, direction: Direction::Down }, d, node.clone()));
        }
        if node.direction != Direction::Right {
            let d = {
                if node.direction == Direction::Left {
                    dist + 1
                } else {
                    dist + 1001
                }
            };
            queue.push_back((Node { x: node.x - 1, y: node.y, direction: Direction::Left }, d, node.clone()));
        }
        if node.direction != Direction::Left {
            let d = {
                if node.direction == Direction::Right {
                    dist + 1
                } else {
                    dist + 1001
                }
            };
            queue.push_back((Node { x: node.x + 1, y: node.y, direction: Direction::Right }, d, node.clone()));
        }

        distances.entry(node).or_insert((dist, vec![parent])).0 = dist;
    }

    let mut in_path: HashSet<Node> = HashSet::new();
    let mut stack = vec![end_node.clone()];
    let mut new_grid = grid.clone();
    loop {
        if stack.is_empty() {
            break;
        }
        let node = stack.pop().unwrap();
        if in_path.contains(&node) {
            continue;
        }

        let parents = distances.get(&node).unwrap().clone().1;
        stack.extend(parents);

        in_path.insert(node.clone());
        new_grid[node.y][node.x] = 'O';
    }
    let count = in_path.iter()
        .map(|node| (node.x, node.y))
        .unique()
        .count();

    (distances.get(&end_node).unwrap().0, count)
}

fn main() {
    let input = fs::read_to_string("inputs/input16.txt").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in input.lines() {
        grid.push(line.chars().collect())
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'S' {
                let start = Node {
                    x,
                    y,
                    direction: Direction::Right
                };
                let (lowest_score, in_path) = bfs(&grid, start);

                println!("Lowest score: {}", lowest_score);
                println!("Number of tiles in shortest paths: {}", in_path);
                break;
            }
        }
    }
}