use std::fs;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Side {
    i: usize,
    j: usize,
    direction: Direction
}

impl Side {
    fn new(i: usize, j: usize, direction: Direction) -> Self {
        Side { i, j, direction }
    }
}

fn in_bounds(grid: &Vec<Vec<char>>, i: i32, j: i32) -> bool {
    if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
        return false;
    }

    true
}

fn num_sides(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Vec<Side> {
    let mut sides = Vec::new();
    let plant = grid[i][j];

    if i == 0 {
        sides.push(Side::new(i, j, Direction::Up));
    } else if grid[i - 1][j] != plant {
        sides.push(Side::new(i, j, Direction::Up));
    }
    if i == grid.len() - 1 {
        sides.push(Side::new(i, j, Direction::Down));
    } else if grid[i + 1][j] != plant {
        sides.push(Side::new(i, j, Direction::Down));
    }
    if j == 0 {
        sides.push(Side::new(i, j, Direction::Left));
    } else if grid[i][j - 1] != plant {
        sides.push(Side::new(i, j, Direction::Left));
    }
    if j == grid[0].len() - 1 {
        sides.push(Side::new(i, j, Direction::Right));
    } else if grid[i][j + 1] != plant {
        sides.push(Side::new(i, j, Direction::Right));
    }

    sides
}

fn grouped_sides(sides: &Vec<Side>) -> usize {
    let mut num_sides = 4;
    let mut ups: Vec<(usize, usize)> = sides
        .iter()
        .filter(|s| matches!(s.direction, Direction::Up))
        .map(|s| (s.i, s.j))
        .collect();
    ups.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    let mut downs: Vec<(usize, usize)> = sides
        .iter()
        .filter(|s| matches!(s.direction, Direction::Down))
        .map(|s| (s.i, s.j))
        .collect();
    downs.sort_by(|a, b| a.partial_cmp(&b).unwrap());
    let mut lefts: Vec<(usize, usize)> = sides
        .iter()
        .filter(|s| matches!(s.direction, Direction::Left))
        .map(|s| (s.i, s.j))
        .collect();
    lefts.sort_by(|a, b| (a.1, a.0).partial_cmp(&(b.1, b.0)).unwrap());
    let mut rights: Vec<(usize, usize)> = sides
        .iter()
        .filter(|s| matches!(s.direction, Direction::Right))
        .map(|s| (s.i, s.j))
        .collect();
    rights.sort_by(|a, b| (a.1, a.0).partial_cmp(&(b.1, b.0)).unwrap());
    for direction in [ups, downs] {
        for win in direction.windows(2) {
            let ((i1, j1), (i2, j2)) = (win[0], win[1]);
            if i1 != i2 || j2.abs_diff(j1) > 1 {
                num_sides += 1;
            }
        }
    }
    for direction in [lefts, rights] {
        for win in direction.windows(2) {
            let ((i1, j1), (i2, j2)) = (win[0], win[1]);
            if j1 != j2 || i2.abs_diff(i1) > 1 {
                num_sides += 1;
            }
        }
    }
    num_sides
}

fn dfs(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let mut visited = Vec::new();
    let mut stack = Vec::new();
    let mut price = 0;
    let mut discount_price = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited.contains(&(i, j)) {
                continue;
            }
            stack.push((i, j));

            let mut area = 0;
            let mut perimeter = 0;
            let mut sides = Vec::new();

            loop {
                if stack.is_empty() {
                    break;
                }
                let (x, y) = stack.pop().unwrap();
                if visited.contains(&(x, y)) {
                    continue;
                }
                let plant = grid[x][y];

                if in_bounds(&grid, x as i32 - 1, y as i32) {
                    if grid[x - 1][y] == plant {
                        stack.push((x - 1, y));
                    }
                }
                if in_bounds(&grid, x as i32 + 1, y as i32) {
                    if grid[x + 1][y] == plant {
                        stack.push((x + 1, y));
                    }
                }
                if in_bounds(&grid, x as i32, y as i32 - 1) {
                    if grid[x][y - 1] == plant {
                        stack.push((x, y - 1));
                    }
                }
                if in_bounds(&grid, x as i32, y as i32 + 1) {
                    if grid[x][y + 1] == plant {
                        stack.push((x, y + 1));
                    }
                }

                let plant_sides = num_sides(grid, x, y);
                area += 1;
                perimeter += plant_sides.len();

                sides.extend(plant_sides);
                visited.push((x, y));
            }

            price += area * perimeter;
            discount_price += area * grouped_sides(&sides);
        }
    }

    (price, discount_price)
}

fn main() {
    let input = fs::read_to_string("inputs/input12.txt").unwrap();
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let (price, discount_price) = dfs(&grid);
    println!("Total price: {}", price);
    println!("Total discounted price: {}", discount_price);
}