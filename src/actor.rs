use crate::arena::Detection;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}
impl Direction {
    pub fn differential() -> HashMap<Direction, (i32, i32)> {
        let mut differential = HashMap::new();
        differential.insert(Direction::N, (0, -1));
        differential.insert(Direction::NE, (1, -1));
        differential.insert(Direction::E, (1, 0));
        differential.insert(Direction::SE, (1, 1));
        differential.insert(Direction::S, (0, 1));
        differential.insert(Direction::SW, (-1, 1));
        differential.insert(Direction::W, (-1, 0));
        differential.insert(Direction::NW, (-1, -1));
        differential
    }
}
#[derive(PartialEq, Debug)]
pub enum Action {
    Hold,
    Move,
}

#[derive(Clone)]
pub struct Actor {
    pub(crate) x: u32,
    pub(crate) y: u32,
    pub img_char: char,
    dir: Direction,
}
impl Actor {
    pub fn new(x: u32, y: u32, img_char: char) -> Actor {
        Actor {
            x,
            y,
            img_char,
            dir: Direction::W,
        }
    }
    fn redirect(&mut self, direction: Direction) {
        self.dir = direction;
    }
    fn step(&mut self) {
        match self.dir {
            Direction::N => self.y -= 1,
            Direction::E => self.x += 1,
            Direction::S => self.y += 1,
            Direction::W => self.x -= 1,
            Direction::NE => {
                self.x += 1;
                self.y -= 1
            }
            Direction::SE => {
                self.x += 1;
                self.y += 1
            }
            Direction::SW => {
                self.x -= 1;
                self.y += 1
            }
            Direction::NW => {
                self.x -= 1;
                self.y -= 1
            }
        }
    }
    pub fn think(&mut self, detect: Detection) -> Action {
        let walkable_tiles: Vec<_> = detect
            .walk_around
            .iter()
            .filter(|&(_, &walkable)| walkable == true)
            .map(|(&direction, _)| direction.clone())
            .collect();
        //println!("{:#?}", walkable_tiles);
        match walkable_tiles.choose(&mut rand::thread_rng()) {
            None => Action::Hold,
            Some(dir) => {
                self.redirect(dir.clone());
                Action::Move
            }
        }
    }
    pub fn operate(&mut self) {
        //if self.think() != Action::Hold {
        // TODO: do something
        self.step(); // TODO: possibly step
    }
}
