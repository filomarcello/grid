mod actor;
mod arena;
mod tools;

use actor::Actor;
use arena::{Arena10x10, Layer10x10, Tile};
use rand::Rng;
use std::thread;
use std::time::Duration;
use tools::clrscr;

fn main() {
    let mut player = Actor::new(3, 3, '*');
    let mut pillars = Layer10x10::new_empty();
    pillars.put(Tile::new_block(), 4, 4);
    pillars.put(Tile::new_block(), 4, 5);
    pillars.put(Tile::new_block(), 5, 4);
    pillars.put(Tile::new_block(), 5, 5);
    let squared = Layer10x10::new_border();
    let mut arena = Arena10x10::new(player);
    arena.add_layer(squared);
    arena.add_layer(pillars);

    loop {
        clrscr();
        arena.show();
        arena.tick();
        thread::sleep(Duration::from_millis(500));
    }
}
