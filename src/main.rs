mod actor;
mod ai;
mod arena;
mod geometry;
mod tools;

use crate::tools::clrscr;
use actor::Actor;
use arena::{Arena10x10, Layer10x10, Tile};
use geometry::Position;
use std::thread;
use std::time::Duration;

fn main() {
    let player = Actor::new_player(Position::new(1, 1), '*');
    let mut pinco = Actor::new(Position::new(2, 2), '+', ai::random_walk);
    let squared = Layer10x10::new_border();
    let mut arena = Arena10x10::new(player, pinco);
    arena.add_layer(squared);
    arena.show();

    loop {
        clrscr();
        arena.show();
        arena.tick();
        thread::sleep(Duration::from_millis(500));
    }
}
