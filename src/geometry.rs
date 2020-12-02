use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}
impl Position {
    pub fn new(x: u32, y: u32) -> Position {
        Position { x, y }
    }
}
impl Add<(i32, i32)> for Position {
    type Output = Position;
    fn add(self, other: (i32, i32)) -> Position {
        Position {
            x: (self.x as i32 + other.0) as u32,
            y: (self.y as i32 + other.1) as u32,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    // TODO: valued enum for differential
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
pub const DIRECTIONS: [Direction; 8] = [
    Direction::N,
    Direction::NE,
    Direction::E,
    Direction::SE,
    Direction::S,
    Direction::SW,
    Direction::W,
    Direction::NW,
];

impl Direction {
    pub fn dir_to_diff(direction: Direction) -> (i32, i32) {
        match direction {
            Direction::N => (0, -1),
            Direction::NE => (1, -1),
            Direction::E => (1, 0),
            Direction::SE => (1, 1),
            Direction::S => (0, 1),
            Direction::SW => (-1, 1),
            Direction::W => (-1, 0),
            Direction::NW => (-1, -1),
        }
    }
}
