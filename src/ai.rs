use crate::actor::Action;
use crate::arena::Detection;
use crate::geometry::Direction;
use rand::seq::SliceRandom;
use std::collections::HashMap;

fn walkable_tiles(walk_around: HashMap<Direction, bool>) -> Vec<Direction> {
    walk_around
        .iter()
        .filter(|&(_, &walkable)| walkable)
        .map(|(&direction, _)| direction)
        .collect()
}

pub fn random_walk(detect: Detection) -> Option<Direction> {
    let walkable_tiles = walkable_tiles(detect.walk_around);
    match walkable_tiles.choose(&mut rand::thread_rng()) {
        None => None,
        Some(dir) => Some(*dir),
    }
}
