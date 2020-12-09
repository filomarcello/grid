mod actor;
mod ai;
mod arena;
mod geometry;
mod tools;

use crate::arena::{show, tick};
use crate::tools::clrscr;
use actor::Actor;
use arena::{Arena10x10, Layer10x10};
use geometry::Position;
use std::thread;
use std::time::Duration;

fn main() {
    let mut player = Actor::new_player(Position::new(1, 1), '*');
    let mut pinco = Actor::new(Position::new(2, 2), '+', ai::random_walk);
    let mut pallino = Actor::new(Position::new(3, 3), 'o', ai::random_walk);
    // let chiocciola = Actor::new(Position::new(4, 4), '@', ai::random_walk);
    let squared = Layer10x10::new_border();
    let mut arena = Arena10x10::new();
    arena.add_layer(squared);
    let mut npcs = vec![pinco, pallino];
    show(&arena, &player, &npcs);

    loop {
        clrscr();
        show(&arena, &player, &npcs);
        tick(&arena, &mut player, &mut npcs);
        thread::sleep(Duration::from_millis(1000));
    }
}
