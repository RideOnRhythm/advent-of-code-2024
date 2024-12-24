use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/input4.txt").unwrap();
    let mut total = 0;
    let mut total2 = 0;

    for line in input.lines() {
        for win in line.chars().collect::<Vec<_>>().windows(4) {
            if ["XMAS".to_string(), "SAMX".to_string()].contains(&win.iter().collect::<String>()) {
                total += 1;
            }
        }
    }
    for win in input.lines().collect::<Vec<_>>().windows(4) {
        for i in 0..win.first().unwrap().len() {
            let col: String = win.into_iter().map(|w| w.chars().nth(i).unwrap()).collect();
            if ["XMAS".to_string(), "SAMX".to_string()].contains(&col) {
                total += 1;
            }
        }
    }
    for win in input.lines().collect::<Vec<_>>().windows(4) {
        for i in 0..win.first().unwrap().len() - 3 {
            let col: String = (i..i+4)
                .map(|j| win[j - i].chars().nth(j).unwrap())
                .collect::<String>();
            if ["XMAS".to_string(), "SAMX".to_string()].contains(&col) {
                total += 1;
            }
        }
    }
    for win in input.lines().collect::<Vec<_>>().windows(4) {
        for i in 0..win.first().unwrap().len() - 3 {
            let col: String = (i..i+4)
                .map(|j| win[j - i].chars().nth(2*i+3-j).unwrap())
                .collect::<String>();
            if ["XMAS".to_string(), "SAMX".to_string()].contains(&col) {
                total += 1;
            }
        }
    }

    for win in input.lines().collect::<Vec<_>>().windows(3) {
        for i in 0..win.first().unwrap().len() - 2 {
            let col: String = (i..i+3)
                .map(|j| win[j - i].chars().nth(j).unwrap())
                .collect::<String>();
            let col2: String = (i..i+3)
                .map(|j| win[j - i].chars().nth(2*i+2-j).unwrap())
                .collect::<String>();
            if ["MAS".to_string(), "SAM".to_string()].contains(&col) && ["MAS".to_string(), "SAM".to_string()].contains(&col2) {
                total2 += 1;
            }
        }
    }

    println!("{}", total);
    println!("{}", total2);
}