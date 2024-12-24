use std::fs;

#[derive(Debug, Clone)]
enum Block {
    File(usize),
    Free
}

#[derive(Debug, Clone)]
enum Chunk {
    File {
        count: usize,
        id: usize
    },
    Free {
        count: usize,
    }
}

fn main() {
    let input = fs::read_to_string("inputs/input9.txt").unwrap();
    let mut extended: Vec<Block> = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_whitespace() {
            continue;
        }
        if i % 2 == 0 {
            extended.extend(vec![Block::File(i / 2); c.to_digit(10).unwrap() as usize]);
        } else {
            extended.extend(vec![Block::Free; c.to_digit(10).unwrap() as usize]);
        }
    }

    for (i, block) in extended.clone().iter().enumerate() {
        if let Block::Free = block {
            let last = extended.iter().rposition(|b| if let Block::File(_) = *b { true } else { false }).unwrap().clone();
            if last > i {
                extended[i] = extended[last].clone();
                extended[last] = Block::Free;
            }
        }
    }

    let mut checksum = 0;
    for (i, block) in extended.iter().enumerate() {
        if let Block::File(id) = block {
            checksum += i * id;
        }
    }

    println!("Checksum: {}", checksum);

    let mut chunks: Vec<Chunk> = Vec::new();
    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_whitespace() {
            continue;
        }
        if i % 2 == 0 {
            chunks.push(Chunk::File {
                count: c.to_digit(10).unwrap() as usize,
                id: i / 2
            });
        } else {
            chunks.push(Chunk::Free { count: c.to_digit(10).unwrap() as usize });
        }
    }

    let mut highest_id = input.len() / 2;
    if input.len() % 2 == 0 {
        highest_id -= 1;
    }
    for id in (0..=highest_id).rev() {
        let index = chunks
            .iter()
            .position(|x| if let Chunk::File { id: id2, .. } = x { id == *id2 } else { false })
            .unwrap();
        let file = chunks[index].clone();
        let Chunk::File { count, .. } = file else {
            continue;
        };
        let Some(free_index) = chunks.iter().position(|c| {
            if let Chunk::Free { count: count_free, .. } = c {
                count_free >= &count
            } else {
                false
            }
        }) else {
            continue;
        };
        let free_space = chunks[free_index].clone();
        let Chunk::Free { count: count_free } = free_space else {
            continue;
        };
        if free_index > index {
            continue;
        }
        chunks[index] = Chunk::Free {
            count
        };
        chunks[free_index] = Chunk::Free {
            count: count_free - count
        };
        chunks.insert(free_index, file);
    }

    let mut extended: Vec<Block> = Vec::new();
    for chunk in chunks {
        match chunk {
            Chunk::File { count, id } => {
                extended.extend(vec![Block::File(id); count]);
            }
            Chunk::Free { count } => {
                extended.extend(vec![Block::Free; count]);
            }
        }
    }

    let mut checksum = 0;
    for (i, block) in extended.iter().enumerate() {
        if let Block::File(id) = block {
            checksum += i * id;
        }
    }

    println!("Checksum 2: {}", checksum);
}