use std::collections::{HashMap, HashSet};
use std::ops::{Add, Sub};

#[derive(Eq, PartialEq, Hash, Debug)]
struct Position {
    col: i32,
    row: i32,
}

impl Position {
    fn new(col: usize, row: usize) -> Self {
        Self {
            col: col as i32,
            row: row as i32,
        }
    }

    fn clone(&self) -> Self {
        Self {
            col: self.col,
            row: self.row,
        }
    }
    
    fn is_valid(&self, col_max: usize, row_max: usize) -> bool {
        self.col >= 0 && self.row >= 0 && self.col <= col_max as i32 && self.row <= row_max as i32
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            col: self.col - other.col,
            row: self.row - other.row,
        }
    }
}

impl Sub for &Position {
    type Output = Position;

    fn sub(self, other: Self) -> Self::Output {
        Self::Output {
            col: self.col - other.col,
            row: self.row - other.row,
        }
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            col: self.col + other.col,
            row: self.row + other.row,
        }
    }
}

impl Add for &Position {
    type Output = Position;

    fn add(self, other: Self) -> Self::Output {
        Self::Output {
            col: self.col + other.col,
            row: self.row + other.row,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Antenna {
    position: Position,
    char: char,
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Antinode {
    position: Position,
}

impl Antinode {
    fn new(position: Position) -> Self {
        Self { position }
    }

    fn from_antennas(antennas: Vec<Antenna>, max_col: usize, max_row: usize) -> Vec<Self> {
        Self::_from_antennas(&antennas[0], &antennas[1..], Vec::new(), max_col, max_row)
    }

    fn _from_antennas(
        antenna: &Antenna,
        other_antennas: &[Antenna],
        mut results: Vec<Self>,
        max_col: usize,
        max_row: usize,
    ) -> Vec<Self> {
        let antinode: Vec<Antinode> = other_antennas
            .iter()
            .map(|a| Self::antinodes(antenna, a, max_col, max_row))
            .flatten()
            .collect();

        results.extend(antinode);

        if other_antennas.is_empty() {
            return results;
        }

        let antenna = &other_antennas[0];
        let remaining = &other_antennas[1..];

        Self::_from_antennas(antenna, remaining, results, max_col, max_row)
    }

    fn antinodes(a: &Antenna, b: &Antenna, max_col: usize, max_row: usize) -> Vec<Self> {
        let mut result = Vec::new();
        let diff_position = &b.position - &a.position;
        
        let mut a_position = a.position.clone();
        while a_position.is_valid(max_col, max_row) {
            result.push(Antinode::new(a_position.clone()));
            a_position = &a_position - &diff_position;
        }

        let mut b_position = b.position.clone();
        while b_position.is_valid(max_col, max_row) {
            result.push(Antinode::new(b_position.clone()));
            b_position = &b_position + &diff_position;
        }
        
        result
    }
}

impl Antenna {
    fn new(position: Position, char: char) -> Self {
        Self { position, char }
    }
}

fn main() {
    let data = std::fs::read_to_string("day_8/src/input.txt").unwrap();
//     let data = "\
// ............
// ........0...
// .....0......
// .......0....
// ....0.......
// ......A.....
// ............
// ............
// ........A...
// .........A..
// ............
// ............";
    let max_col = data.lines().next().unwrap().len() - 1;
    let max_row = data.lines().count() - 1;
    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();

    for (row, line) in data.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char != '.' {
                antennas
                    .entry(char)
                    .or_default()
                    .push(Antenna::new(Position::new(col, row), char));
            }
        }
    }

    dbg!(max_col, max_row);

    let antinodes: HashSet<Antinode> = antennas
        .into_iter()
        .map(|(_, antennas)| Antinode::from_antennas(antennas, max_col, max_row))
        .flatten()
        .collect();

    println!("Part 1: {}", antinodes.len());
}
