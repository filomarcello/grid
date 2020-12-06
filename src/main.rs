mod actor;
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
    let player = Actor::new(Position::new(1, 1), '*');
    let mut pillars = Layer10x10::new_empty();
    pillars.put(Tile::new_block(), Position::new(4, 4));
    pillars.put(Tile::new_block(), Position::new(4, 5));
    pillars.put(Tile::new_block(), Position::new(5, 4));
    pillars.put(Tile::new_block(), Position::new(5, 5));
    let squared = Layer10x10::new_border();
    let mut arena = Arena10x10::new(player);
    arena.add_layer(squared);
    arena.add_layer(pillars);
    arena.show();
    println!("{:#?}", arena.player.think(arena.player_observe()));

    loop {
        clrscr();
        arena.show();
        arena.tick();
        println!("{}", arena.player);
        thread::sleep(Duration::from_millis(500));
    }
}
