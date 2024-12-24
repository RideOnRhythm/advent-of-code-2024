use std::collections::VecDeque;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input15.txt").unwrap();
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut moves = String::new();

    for line in input.lines() {
        if line.trim().is_empty() || line.contains('^') {
            moves.push_str(line.trim());
            continue;
        }

        grid.push(line.trim().chars().collect());
    }

    let mut guard_x = 0;
    let mut guard_y = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                guard_x = x;
                guard_y = y;
            }
        }
    }

    for m in moves.chars() {
        let (pos_x, pos_y): (usize, usize) = match m {
            '^' => (guard_x, guard_y - 1),
            'v' => (guard_x, guard_y + 1),
            '<' => (guard_x - 1, guard_y),
            '>' => (guard_x + 1, guard_y),
            _ => continue
        };

        let adj = grid[pos_y][pos_x];
        if adj == '#' {
            continue;
        }
        if adj == 'O' {
            let mut facing: Vec<char> = match m {
                '^' => {
                    (0..guard_y).map(|i| {
                        grid[i][guard_x]
                    }).rev().collect()
                },
                'v' => {
                    (pos_y..grid.len()).map(|i| {
                        grid[i][guard_x]
                    }).collect()
                },
                '<' => {
                    grid[guard_y][0..guard_x].iter().map(|x| *x).rev().collect()
                },
                '>' => {
                    grid[guard_y][pos_x..].iter().map(|x| *x).collect()
                },
                _ => continue
            };

            if !facing.contains(&'.') {
                continue;
            }
            let dot_index = facing.iter().position(|&c| c == '.').unwrap();
            let wall_index = facing.iter().position(|&c| c == '#').unwrap();
            if wall_index < dot_index {
                continue;
            }
            facing = facing[..dot_index].to_vec();
            match m {
                '^' => {
                    for i in 0..facing.len() {
                        grid[pos_y - i - 1][guard_x] = facing[i];
                    }
                },
                'v' => {
                    for i in 0..facing.len() {
                        grid[pos_y + i + 1][guard_x] = facing[i];
                    }
                },
                '<' => {
                    for i in 0..facing.len() {
                        grid[guard_y][pos_x - i - 1] = facing[i];
                    }
                },
                '>' => {
                    for i in 0..facing.len() {
                        grid[guard_y][pos_x + i + 1] = facing[i];
                    }
                },
                _ => continue
            };
        }
        grid[guard_y][guard_x] = '.';
        grid[pos_y][pos_x] = '@';
        guard_x = pos_x;
        guard_y = pos_y;
    }

    let mut coordinates = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == 'O' {
                coordinates += 100 * y + x;
            }
        }
    }

    println!("Sum of all boxes' GPS coordinates: {}", coordinates);

    grid = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() || line.contains('^') {
            continue;
        }
        grid.push(line.trim().chars().map(|c| {
            if c == 'O' {
                ['[', ']']
            } else if c == '@' {
                ['@', '.']
            } else {
                [c, c]
            }
        }).flatten().collect());
    }

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '@' {
                guard_x = x;
                guard_y = y;
                break;
            }
        }
    }

    'outer: for m in moves.chars() {
        let (pos_x, pos_y): (usize, usize) = match m {
            '^' => (guard_x, guard_y - 1),
            'v' => (guard_x, guard_y + 1),
            '<' => (guard_x - 1, guard_y),
            '>' => (guard_x + 1, guard_y),
            _ => continue
        };

        let adj = grid[pos_y][pos_x];
        if adj == '#' {
            continue;
        }
        if adj == ']' && m == '<' {
            let mut facing: Vec<char> = grid[guard_y][0..guard_x].iter().map(|x| *x).rev().collect();

            if !facing.contains(&'.') {
                continue;
            }
            let dot_index = facing.iter().position(|&c| c == '.').unwrap();
            let wall_index = facing.iter().position(|&c| c == '#').unwrap();
            if wall_index < dot_index {
                continue;
            }
            facing = facing[..dot_index].to_vec();

            for i in 0..facing.len() {
                grid[guard_y][pos_x - i - 1] = facing[i];
            }
        } else if adj == '[' && m == '>' {
            let mut facing: Vec<char> = grid[guard_y][pos_x..].iter().map(|x| *x).collect();

            if !facing.contains(&'.') {
                continue;
            }
            let dot_index = facing.iter().position(|&c| c == '.').unwrap();
            let wall_index = facing.iter().position(|&c| c == '#').unwrap();
            if wall_index < dot_index {
                continue;
            }
            facing = facing[..dot_index].to_vec();

            for i in 0..facing.len() {
                grid[guard_y][pos_x + i + 1] = facing[i];
            }
        } else if ['[', ']'].contains(&adj) && m == '^' {
            let facing: Vec<char> = (0..guard_y).map(|i| {
                grid[i][guard_x]
            }).rev().collect();

            if !facing.contains(&'.') {
                continue;
            }
            let dot_index = facing.iter().position(|&c| c == '.').unwrap();
            let wall_index = facing.iter().position(|&c| c == '#').unwrap();
            if wall_index < dot_index {
                continue;
            }

            let mut boxes = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back((pos_x, pos_y));
            loop {
                let Some((x, y)) = queue.pop_front() else {
                    break;
                };
                if boxes.contains(&(x, y)) {
                    continue;
                }

                let c = grid[y][x];
                if c == '[' {
                    queue.push_back((x + 1, y));
                } else {
                    queue.push_back((x - 1, y));
                }
                if ['[', ']'].contains(&grid[y - 1][x]) {
                    queue.push_back((x, y - 1));
                }
                if grid[y - 1][x] == '#' {
                    continue 'outer;
                }

                boxes.push((x, y));
            }

            let mut lines = Vec::new();
            let min = boxes.iter()
                .map(|(_, y)| y)
                .min().unwrap();
            let max = boxes.iter()
                .map(|(_, y)| y)
                .max().unwrap();
            for _ in 0..max - min + 1 {
                lines.push(Vec::new());
            }
            for (x, y) in boxes.clone() {
                lines[y - min].push((x, y));
            }

            for line in lines.iter_mut() {
                line.sort();
            }

            for line in lines {
                for (x, y) in line {
                    grid[y - 1][x] = grid[y][x];
                    grid[y][x] = '.';
                }
            }
        } else if ['[', ']'].contains(&adj) && m == 'v' {
            let facing: Vec<char> = (pos_y..grid.len()).map(|i| {
                grid[i][guard_x]
            }).collect();

            if !facing.contains(&'.') {
                continue;
            }
            let dot_index = facing.iter().position(|&c| c == '.').unwrap();
            let wall_index = facing.iter().position(|&c| c == '#').unwrap();
            if wall_index < dot_index {
                continue;
            }

            let mut boxes = Vec::new();
            let mut queue = VecDeque::new();
            queue.push_back((pos_x, pos_y));
            loop {
                let Some((x, y)) = queue.pop_front() else {
                    break;
                };
                if boxes.contains(&(x, y)) {
                    continue;
                }

                let c = grid[y][x];
                if c == '[' {
                    queue.push_back((x + 1, y));
                } else {
                    queue.push_back((x - 1, y));
                }
                if ['[', ']'].contains(&grid[y + 1][x]) {
                    queue.push_back((x, y + 1));
                }
                if grid[y + 1][x] == '#' {
                    continue 'outer;
                }

                boxes.push((x, y));
            }

            let mut lines = Vec::new();
            let min = boxes.iter()
                .map(|(_, y)| y)
                .min().unwrap();
            let max = boxes.iter()
                .map(|(_, y)| y)
                .max().unwrap();
            for _ in 0..max - min + 1 {
                lines.push(Vec::new());
            }
            for (x, y) in boxes.clone() {
                lines[max - y].push((x, y));
            }

            for line in lines.iter_mut() {
                line.sort();
            }

            for line in lines {
                for (x, y) in line {
                    grid[y + 1][x] = grid[y][x];
                    grid[y][x] = '.';
                }
            }
            if adj == '[' {
                grid[pos_y][pos_x + 1] = '.';
            } else {
                grid[pos_y][pos_x - 1] = '.';
            }
        }

        grid[guard_y][guard_x] = '.';
        grid[pos_y][pos_x] = '@';
        guard_x = pos_x;
        guard_y = pos_y;
    }


    let mut coordinates = 0;
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] == '[' {
                coordinates += 100 * y + x;
            }
        }
    }

    println!("Sum of all boxes' GPS coordinates: {}", coordinates);
}