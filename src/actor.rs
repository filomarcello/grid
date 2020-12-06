use crate::arena::Detection;
use crate::geometry::{Direction, Position};
use rand::seq::SliceRandom;
use std::fmt;

#[derive(PartialEq, Debug)]
pub enum Action {
    Hold,
    Move,
}

#[derive(Clone)]
pub struct Actor {
    pub position: Position,
    pub img_char: char,
    dir: Direction,
}
impl Actor {
    pub fn new(position: Position, img_char: char) -> Actor {
        Actor {
            position,
            img_char,
            dir: Direction::W,
        }
    }
    fn redirect(&mut self, direction: Direction) {
        self.dir = direction;
    }
    fn step(&mut self) {
        self.position = self.position + Direction::dir_to_diff(self.dir);
    }
    pub fn think(&mut self, detect: Detection) -> Action {
        // random AI: randomly chooses a free tile around
        let walkable_tiles: Vec<_> = detect
            .walk_around
            .iter()
            .filter(|&(_, &walkable)| walkable)
            .map(|(&direction, _)| direction)
            .collect();
        //println!("{:#?}", walkable_tiles);
        match walkable_tiles.choose(&mut rand::thread_rng()) {
            None => Action::Hold,
            Some(dir) => {
                self.redirect(*dir);
                Action::Move
            }
        }
    }
    pub fn operate(&mut self, action: Action) {
        if action == Action::Move {
            self.step();
        }
    }
}
impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Actor pos ({}, {})", self.position.x, self.position.y)
    }
}
