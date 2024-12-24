use std::fs;

fn dfs(grid: &Vec<Vec<u32>>, i: usize, j: usize) -> (i32, i32) {
    let mut stack = vec![(i, j)];
    let mut visited = Vec::new();
    let mut score = 0;
    let mut rating: i32 = 0;
    loop {
        if stack.is_empty() {
            break;
        }
        let (x, y) = stack.pop().unwrap();
        let current = grid[x][y];
        if current == 9 {
            if !visited.contains(&(x, y)) {
                score += 1;
            }
            rating += 1;
        }

        if x > 0 {
            if grid[x - 1][y] - current == 1 {
                stack.push((x - 1, y));
            }
        }
        if x < grid.len() - 1 {
            if grid[x + 1][y] - current == 1 {
                stack.push((x + 1, y));
            }
        }
        if y > 0 {
            if grid[x][y - 1] - current == 1 {
                stack.push((x, y - 1));
            }
        }
        if y < grid[0].len() - 1 {
            if grid[x][y + 1] - current == 1 {
                stack.push((x, y + 1));
            }
        }

        visited.push((x, y));
    }
    (score, rating)
}

fn main() {
    let input = fs::read_to_string("inputs/input10.txt").unwrap();
    let mut total_score = 0;
    let mut total_rating = 0;
    let grid: Vec<Vec<u32>> = input.lines().map(|l| {
        l
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect()
    })
        .collect();

    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            if grid[x][y] != 0 {
                continue;
            }
            let result = dfs(&grid, x, y);
            total_score += result.0;
            total_rating += result.1;
        }
    }
    println!("Total Score: {}", total_score);
    println!("Total Rating: {}", total_rating);
}