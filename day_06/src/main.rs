use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fs;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Position {
    x: i16,
    y: i16,
}

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct PositionWithOrientation {
    position: Position,
    orientation: Orientation
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
enum Orientation {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Orientation {
    fn next(&self) -> Self {
        match self {
            Orientation::UP => Orientation::RIGHT,
            Orientation::RIGHT => Orientation::DOWN,
            Orientation::DOWN => Orientation::LEFT,
            Orientation::LEFT => Orientation::UP,
        }
    }
}

#[derive(PartialEq)]
enum MoveError {
    Obstacle,
    OutOfBounds,
}

fn main() {
    let data = fs::read_to_string("day_06/src/input.txt").unwrap();
//     let data = "....#.....
// ....^....#
// ..........
// ..#.......
// .......#..
// ..........
// .#........
// ........#.
// #.........
// ......#...";
    let mut positions: HashSet<Position> = HashSet::new();
    let mut obstacles: HashSet<Position> = HashSet::new();
    let mut original_guard_position = Position { x: 0, y: 0 }; // This will be overwritten
    let mut guard_position = Position { x: 0, y: 0 }; // This will be overwritten
    let mut orientation = Orientation::UP;

    let max_x = data.lines().next().unwrap().len() - 1;
    let max_y = data.lines().count() - 1;

    for (i, line) in data.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char == '#' {
                obstacles.insert(Position {
                    x: j as i16,
                    y: i as i16,
                });
            }

            if char == '^' {
                let position = Position {
                    x: j as i16,
                    y: i as i16,
                };
                positions.insert(position);
                original_guard_position = position;
                guard_position = position;
            }
        }
    }

    loop {
        match _move(&guard_position, &obstacles, &orientation, &max_x, &max_y) {
            Ok(new_position) => {
                guard_position = new_position;
                positions.insert(new_position);
            }
            Err(MoveError::Obstacle) => {
                orientation = orientation.next();
            }
            Err(MoveError::OutOfBounds) => {
                break;
            }
        }
    }

    println!("Positions {}", positions.len());
    
    let mut obstacle_loop_count = 0;
    let mut tested_obstacle = Position { x:0, y:0 };

    'position_loop: for position in positions.iter() {
        if obstacles.contains(position) {
            continue;
        }

        let mut guard_position = original_guard_position;
        let mut orientation = Orientation::UP;
        let mut positions_with_orientation: HashSet<PositionWithOrientation> = HashSet::new();
        positions_with_orientation.insert(PositionWithOrientation { position: original_guard_position, orientation });

        obstacles.remove(&tested_obstacle);
        tested_obstacle = *position;
        obstacles.insert(tested_obstacle);

        loop {
            match _move(&guard_position, &obstacles, &orientation, &max_x, &max_y) {
                Ok(new_position) => {
                    if positions_with_orientation.contains(&PositionWithOrientation { position: new_position, orientation }) {
                        obstacle_loop_count += 1;
                        continue 'position_loop;
                    }
                    guard_position = new_position;
                    positions_with_orientation.insert(PositionWithOrientation { position: new_position, orientation });
                }
                Err(MoveError::Obstacle) => {
                    orientation = orientation.next();
                }
                Err(MoveError::OutOfBounds) => {
                    break;
                }
            }
        }
    }

    println!("Obstacles that cause loop: {}", obstacle_loop_count);
}

fn _move(
    position: &Position,
    obstacles: &HashSet<Position>,
    orientation: &Orientation,
    max_x: &usize,
    max_y: &usize,
) -> Result<Position, MoveError> {
    let new_position = match orientation {
        Orientation::UP => Position {
            x: position.x,
            y: position.y - 1,
        },
        Orientation::RIGHT => Position {
            x: position.x + 1,
            y: position.y,
        },
        Orientation::DOWN => Position {
            x: position.x,
            y: position.y + 1,
        },
        Orientation::LEFT => Position {
            x: position.x - 1,
            y: position.y,
        },
    };

    if obstacles.contains(&new_position) {
        return Err(MoveError::Obstacle);
    }

    if new_position.x < 0
        || new_position.x > *max_x as i16
        || new_position.y < 0
        || new_position.y > *max_y as i16
    {
        return Err(MoveError::OutOfBounds);
    }

    Ok(new_position)
}
