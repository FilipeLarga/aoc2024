use std::collections::HashSet;
use std::fmt::Display;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Position {
    x: usize,
    y: usize,
    height: u8,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x:{}, y:{}, height:{})", self.x, self.y, self.height)
    }
}

impl Position {
    fn new(x: usize, y: usize, height: char) -> Position {
        Position {
            x,
            y,
            height: height.to_digit(10).unwrap() as u8,
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("day_10/src/input.txt").unwrap();
//     let data = "89010123
// 78121874
// 87430965
// 96549874
// 45678903
// 32019012
// 01329801
// 10456732";

    let map = data
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| Position::new(x, y, c))
                .collect::<Vec<Position>>()
        })
        .collect::<Vec<Vec<Position>>>();

    let count = map
        .iter()
        .flatten()
        .filter(|p| p.height == 0)
        .map(|p| find_nine_heights(p, &map))
        .flatten()
        .count();

    let sum: u32 = map
        .iter()
        .flatten()
        .filter(|p| p.height == 0)
        .map(|p| find_trails(p, &map))
        .sum();

    println!("{}", count);
    println!("{}", sum);
}

fn find_nine_heights<'a>(position: &'a Position, map: &'a Vec<Vec<Position>>) -> HashSet<&'a Position> {
    let mut nine_heights = HashSet::new();

    if position.height == 9 {
        nine_heights.insert(position);
        return nine_heights;
    }

    if position.x > 0 {
        let next_position = &map[position.y][position.x - 1];
        if next_position.height == position.height + 1 {
            nine_heights.extend(find_nine_heights(next_position, map));
        }
    }

    if position.x < map[0].len() - 1 {
        let next_position = &map[position.y][position.x + 1];
        if next_position.height == position.height + 1 {
            nine_heights.extend(find_nine_heights(next_position, map));
        }
    }

    if position.y > 0 {
        let next_position = &map[position.y - 1][position.x];
        if next_position.height == position.height + 1 {
            nine_heights.extend(find_nine_heights(next_position, map));
        }
    }

    if position.y < map.len() - 1 {
        let next_position = &map[position.y + 1][position.x];
        if next_position.height == position.height + 1 {
            nine_heights.extend(find_nine_heights(next_position, map));
        }
    }

    nine_heights
}


fn find_trails<'a>(position: &'a Position, map: &'a Vec<Vec<Position>>) -> u32 {
    let mut trails = 0;

    if position.height == 9 {
        return 1;
    }

    if position.x > 0 {
        let next_position = &map[position.y][position.x - 1];
        if next_position.height == position.height + 1 {
            trails += find_trails(next_position, map);
        }
    }

    if position.x < map[0].len() - 1 {
        let next_position = &map[position.y][position.x + 1];
        if next_position.height == position.height + 1 {
            trails += find_trails(next_position, map);
        }
    }

    if position.y > 0 {
        let next_position = &map[position.y - 1][position.x];
        if next_position.height == position.height + 1 {
            trails += find_trails(next_position, map);
        }
    }

    if position.y < map.len() - 1 {
        let next_position = &map[position.y + 1][position.x];
        if next_position.height == position.height + 1 {
            trails += find_trails(next_position, map);
        }
    }

    trails
}