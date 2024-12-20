use std::collections::HashMap;

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
struct Stone {
    number: u64,
}

impl Stone {
    fn new(number: u64) -> Stone {
        Stone { number }
    }
}

fn main() {
    let data = std::fs::read_to_string("day_11/src/input.txt").unwrap();
    // let data = "125 17";

    const ITERATIONS: usize = 75;

    let stones = data
        .split_whitespace()
        .map(|n| Stone::new(n.parse::<u64>().unwrap()))
        .collect::<Vec<Stone>>();

    let mut stone_cache: HashMap<(Stone, usize), u64> = HashMap::new();

    let stone_count: u64 = stones
        .iter()
        .map(|&s| blink(s, ITERATIONS, &mut stone_cache))
        .sum();

    println!("The len is {}", stone_count);
}

fn blink(stone: Stone, iterations: usize, stone_cache: &mut HashMap<(Stone, usize), u64>) -> u64 {
    if iterations == 0 {
        return 1; // Reached the end: 1 Stone
    }

    if let Some(stones) = stone_cache.get(&(stone, iterations)) {
        return *stones;
    }

    let number = stone.number;

    if number == 0 {
        let next_stone = Stone::new(1);
        let result = blink(next_stone, iterations - 1, stone_cache);
        stone_cache.insert((stone, iterations), result);
        
        return result;
    }

    let number_str = number.to_string();
    if number_str.len() % 2 == 0 {
        let left_number = number_str[0..(number_str.len() / 2)].parse::<u64>().unwrap();
        let right_number = number_str[(number_str.len() / 2)..].parse::<u64>().unwrap();

        let next_stones = (Stone::new(left_number), Stone::new(right_number));
        let result = blink(next_stones.0, iterations - 1, stone_cache)
            + blink(next_stones.1, iterations - 1, stone_cache);
        stone_cache.insert((stone, iterations), result);
        
        return result;
    }

    let next_stone = Stone::new(number * 2024);
    let result = blink(next_stone, iterations - 1, stone_cache);
    stone_cache.insert((stone, iterations), result);

    result
}
