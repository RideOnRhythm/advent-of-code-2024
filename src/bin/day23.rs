use std::collections::{HashMap, HashSet};
use std::fs;

fn dfs(connections: &Vec<(String, String)>, path: &Vec<String>) -> Vec<Vec<String>> {
    let last = path.last().unwrap();
    let mut paths = Vec::new();
    for (c1, c2) in connections {
        if c1 != last && c2 != last || (*c1 == path[0] && *c2 == path[1]) {
            continue;
        }
        let other = if c1 == last { c2 } else { c1 };

        if *other == path[0] && path.len() == 3 {
            return vec![path.clone()];
        } else if path.len() == 3 {
            continue;
        }

        let mut new_path = path.clone();
        new_path.push(other.clone());
        paths.extend(dfs(connections, &new_path));
    }

    paths
}

fn all_connections(connections: &Vec<(String, String)>, computer: String) -> HashSet<String> {
    let mut connected = HashSet::new();
    for (c1, c2) in connections {
        if *c1 != computer && *c2 != computer {
            continue;
        }
        let other = if *c1 == computer { c2 } else { c1 };

        connected.insert(other.clone());
    }

    connected
}

fn set_size(connections: &Vec<(String, String)>, computer: String) -> HashSet<String> {
    let mut network = HashSet::new();
    network.insert(computer.clone());

    let mut stack: Vec<String> = vec![computer];
    let mut visited = HashSet::new();
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    loop {
        let Some(c) = stack.pop() else {
            break;
        };
        if visited.contains(&c) {
            continue;
        }

        let mut neighbors: HashSet<String> = if map.contains_key(&c) {
            map.get(&c).unwrap().clone()
        } else {
            all_connections(&connections, c.clone())
        };
        map.insert(c.clone(), neighbors.clone());
        for n in neighbors {
            let mut sub_connections: HashSet<String> = if map.contains_key(&n) {
                map.get(&n).unwrap().clone()
            } else {
                all_connections(&connections, n.clone())
            };
            map.insert(n.clone(), sub_connections.clone());
            sub_connections = sub_connections.intersection(&network).cloned().collect();
            if sub_connections == network {
                stack.push(n.clone());
                network.insert(n.clone());
            }
        }
        visited.insert(c);
    }

    network
}

fn main() {
    let input = fs::read_to_string("inputs/input23.txt").unwrap();

    let mut connections = Vec::new();
    for line in input.lines() {
        let split: Vec<&str> = line.split("-").collect();
        connections.push((split[0].to_owned(), split[1].to_owned()));
    }

    let mut sets = HashSet::new();

    for (c1, c2) in connections.iter() {
        let paths = dfs(&connections, &vec![c1.clone(), c2.clone()]);
        for mut path in paths {
            path.sort();
            sets.insert(path);
        }
    }

    let count = sets.iter()
        .filter(|s| s.iter().any(|c| c.starts_with("t")))
        .count();
    println!("Number of sets starting with T: {}", count);

    let mut largest_set = HashSet::new();
    for (c1, _) in connections.iter() {
        let set = set_size(&connections, c1.clone());
        if set.len() > largest_set.len() {
            largest_set = set;
        }
    }
    let mut password: Vec<String> = largest_set.into_iter().collect();
    password.sort();
    println!("Password: {}", password.join(","));
}