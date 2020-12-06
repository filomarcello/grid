use crate::ai;
use crate::ai::Ai;
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
    ai: Ai,
    dir: Direction,
}
impl Actor {
    pub fn new(position: Position, img_char: char, ai: Ai) -> Actor {
        Actor {
            position,
            img_char,
            ai,
            dir: Direction::W,
        }
    }
    pub fn new_player(position: Position, img_char: char) -> Actor {
        Actor::new(position, img_char, ai::human_player)
    }
    fn redirect(&mut self, direction: Direction) {
        self.dir = direction;
    }
    fn step(&mut self) {
        self.position = self.position + Direction::dir_to_diff(self.dir);
    }
    fn think(&mut self, detect: Detection) -> Action {
        let dir = (self.ai)(detect);
        match dir {
            None => Action::Hold,
            Some(direction) => {
                self.redirect(direction);
                Action::Move
            }
        }
    }
    fn operate(&mut self, action: Action) {
        if action == Action::Move {
            self.step();
        } // TODO: implement other actions
    }
    pub fn tick(&mut self, detect: Detection) {
        let action = self.think(detect);
        self.operate(action);
    }
}
impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Actor pos ({}, {})", self.position.x, self.position.y)
    }
}
