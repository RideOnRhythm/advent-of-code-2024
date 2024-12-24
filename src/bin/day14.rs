use std::fs;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Robot {
    x: usize,
    y: usize,
    vel_x: i32,
    vel_y: i32
}

fn main() {
    let input = fs::read_to_string("inputs/input14.txt").unwrap();
    let mut robots: Vec<Robot> = Vec::new();
    for line in input.lines() {
        let mut split = line.split(" ");
        let (mut p_def, mut v_def) = (
            split.next().unwrap().split(","),
            split.next().unwrap().split(",")
        );
        let px: usize = p_def.next().unwrap()[2..].parse().unwrap();
        let py: usize = p_def.next().unwrap().parse().unwrap();
        let vx: i32 = v_def.next().unwrap()[2..].parse().unwrap();
        let vy: i32 = v_def.next().unwrap().parse().unwrap();

        robots.push(Robot {
            x: px,
            y: py,
            vel_x: vx,
            vel_y: vy
        })
    }

    let width = 101;
    let height = 103;
    let mid_x = ((width - 1) / 2) as usize;
    let mid_y = ((height - 1) / 2) as usize;
    let mut positions = Vec::new();

    for robot in &robots {
        let x = robot.x as i32 + robot.vel_x * 100;
        let x = x.rem_euclid(width) as usize;
        let y = robot.y as i32 + robot.vel_y * 100;
        let y = y.rem_euclid(height) as usize;

        positions.push((x, y));
    }

    let mut quad_1 = 0;
    let mut quad_2 = 0;
    let mut quad_3 = 0;
    let mut quad_4 = 0;

    for (x, y) in positions {
        if x < mid_x && y > mid_y {
            quad_1 += 1;
        } else if x > mid_x && y > mid_y {
            quad_2 += 1;
        } else if x < mid_x && y < mid_y {
            quad_3 += 1;
        } else if x > mid_x && y < mid_y {
            quad_4 += 1;
        }
    }

    println!("Safety Factor: {}", quad_1 * quad_2 * quad_3 * quad_4);

    let mut positions = Vec::new();
    for robot in &robots {
        let x = robot.x;
        let y = robot.y;
        positions.push((x, y));
    }
    for seconds in 1.. {
        for (i, robot) in robots.clone().iter().enumerate() {
            let x = robot.x as i32 + robot.vel_x;
            let x = x.rem_euclid(width) as usize;
            let y = robot.y as i32 + robot.vel_y;
            let y = y.rem_euclid(height) as usize;

            robots[i] = Robot {
                x,
                y,
                vel_x: robot.vel_x,
                vel_y: robot.vel_y
            };
        }

        let positions = robots
            .iter()
            .map(|r| (r.x, r.y));
        let unique = robots
            .iter()
            .map(|r| (r.x, r.y))
            .unique()
            .count();

        if unique == 500 {
            let mut s: Vec<Vec<char>> = (0..height)
                .map(|_| (0..width).map(|_| '.').collect())
                .collect();
            for (x, y) in positions {
                s[y][x] = '0';
            }
            println!("Easter Egg: {} seconds", seconds);
            break;
        }
    }
}