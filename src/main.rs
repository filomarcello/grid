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
    let pinco = Actor::new(Position::new(2, 2), '+', ai::random_walk);
    let pallo = Actor::new(Position::new(3, 3), 'o', ai::random_walk);
    let squared = Layer10x10::new_border();
    let mut arena = Arena10x10::new(player);
    arena.add_layer(squared);
    arena.add_npc(pinco);
    arena.add_npc(pallo);
    arena.show();

    // loop {
    //     clrscr();
    //     arena.show();
    //     arena.tick();
    //     thread::sleep(Duration::from_millis(500));
    // }
}
