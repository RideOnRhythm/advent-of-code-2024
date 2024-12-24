use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Button {
    Up,
    Down,
    Left,
    Right,
    Activate
}

fn numeric_position(button: char) -> (i32, i32) {
    match button {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => (0, 0)
    }
}

fn directional_position(button: &Button) -> (i32, i32) {
    match button {
        Button::Up => (1, 0),
        Button::Down => (1, 1),
        Button::Left => (0, 1),
        Button::Right => (2, 1),
        Button::Activate => (2, 0)
    }
}

fn numeric_x_first(x_pos: i32, y_pos: i32, target_x: i32, target_y: i32) -> bool {
    if y_pos == 3 && target_x == 0 {
        return false;
    }
    if x_pos == 0 && target_y == 3 {
        return true;
    }

    let x_dist = target_x - x_pos;
    let first_x_button = if x_dist < 0 { Button::Left } else { Button::Right };
    if first_x_button == Button::Left {
        true
    } else {
        false
    }
}

fn directional_x_first(x_pos: i32, y_pos: i32, target_x: i32, target_y: i32) -> bool {
    if x_pos == 0 && target_y == 0 {
        return true;
    }
    if y_pos == 0 && target_x == 0 {
        return false;
    }

    let x_dist = target_x - x_pos;
    let first_x_button = if x_dist < 0 { Button::Left } else { Button::Right };
    if first_x_button == Button::Left {
        true
    } else {
        false
    }
}

fn numeric_presses(code: String) -> Vec<Button> {
    let mut buttons = Vec::new();
    let mut position = 'A';
    for c in code.chars() {
        let (x_pos, y_pos) = numeric_position(position);
        let (target_x, target_y) = numeric_position(c);
        let x_dist = target_x - x_pos;
        let y_dist = target_y - y_pos;
        if numeric_x_first(x_pos, y_pos, target_x, target_y) {
            if x_dist < 0 {
                buttons.extend(vec![Button::Left; -x_dist as usize]);
            } else {
                buttons.extend(vec![Button::Right; x_dist as usize]);
            }
            if y_dist < 0 {
                buttons.extend(vec![Button::Up; -y_dist as usize]);
            } else {
                buttons.extend(vec![Button::Down; y_dist as usize]);
            }
        } else {
            if y_dist < 0 {
                buttons.extend(vec![Button::Up; -y_dist as usize]);
            } else {
                buttons.extend(vec![Button::Down; y_dist as usize]);
            }
            if x_dist < 0 {
                buttons.extend(vec![Button::Left; -x_dist as usize]);
            } else {
                buttons.extend(vec![Button::Right; x_dist as usize]);
            }
        }
        buttons.push(Button::Activate);

        position = c;
    }

    buttons
}

fn num_presses(map: &mut HashMap<(Button, Button, i32), usize>, current_button: &Button, target_button: &Button, depth: i32) -> usize {
    let key = (current_button.clone(), target_button.clone(), depth);
    if map.contains_key(&key) {
        return map.get(&key).unwrap().clone();
    }

    let mut buttons = Vec::new();
    let (x_pos, y_pos) = directional_position(&current_button);
    let (target_x, target_y) = directional_position(&target_button);
    let x_dist = target_x - x_pos;
    let y_dist = target_y - y_pos;
    let mut total = 0;

    if directional_x_first(x_pos, y_pos, target_x, target_y) {
        if x_dist < 0 {
            buttons.extend(vec![Button::Left; -x_dist as usize]);
        } else {
            buttons.extend(vec![Button::Right; x_dist as usize]);
        }
        if y_dist < 0 {
            buttons.extend(vec![Button::Up; -y_dist as usize]);
        } else {
            buttons.extend(vec![Button::Down; y_dist as usize]);
        }
    } else {
        if y_dist < 0 {
            buttons.extend(vec![Button::Up; -y_dist as usize]);
        } else {
            buttons.extend(vec![Button::Down; y_dist as usize]);
        }
        if x_dist < 0 {
            buttons.extend(vec![Button::Left; -x_dist as usize]);
        } else {
            buttons.extend(vec![Button::Right; x_dist as usize]);
        }
    }
    buttons.push(Button::Activate);

    if depth == 1 {
        return buttons.len();
    }
    let mut previous = Button::Activate;
    for button in buttons {
        let result = num_presses(map, &previous, &button, depth - 1);
        map.insert((previous.clone(), button.clone(), depth - 1), result);
        total += result;
        previous = button;
    }

    total
}

fn main() {
    let input = fs::read_to_string("inputs/input21.txt").unwrap();

    let mut total = 0;
    let mut map: HashMap<(Button, Button, i32), usize> = HashMap::new();

    for line in input.lines() {
        let mut presses = numeric_presses(String::from(line));
        let mut n = 0;
        let mut previous = Button::Activate;
        for button in presses {
            n += num_presses(&mut map, &previous, &button, 2);
            previous = button;
        }

        total += n * &line[..3].parse::<usize>().unwrap();
    }

    println!("Total (2 robots): {}", total);

    let mut total = 0;
    let mut map: HashMap<(Button, Button, i32), usize> = HashMap::new();

    for line in input.lines() {
        let mut presses = numeric_presses(String::from(line));
        let mut n = 0;
        let mut previous = Button::Activate;
        for button in presses {
            n += num_presses(&mut map, &previous, &button, 25);
            previous = button;
        }

        total += n * &line[..3].parse::<usize>().unwrap();
    }

    println!("Total (25 robots): {}", total);
}