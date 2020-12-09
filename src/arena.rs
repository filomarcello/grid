use crate::actor::{are_in, are_not_in, Actor};
use crate::geometry::{Direction, Position, DIRECTIONS};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Detection {
    // TODO: implement how to detect environment
    pub walk_around: HashMap<Direction, bool>,
}
impl Detection {
    pub fn new() -> Detection {
        Detection {
            walk_around: HashMap::new(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub img_char: char,
    walkable: bool,
}
impl Tile {
    pub fn new(img_char: char, walkable: bool) -> Tile {
        Tile { img_char, walkable }
    }
    pub fn new_empty() -> Tile {
        Tile::new(' ', true)
    }
    pub fn new_block() -> Tile {
        Tile::new('█', false)
    }
}

pub struct Layer10x10 {
    tiles: [[Tile; 10]; 10],
}
impl Layer10x10 {
    pub fn new_empty() -> Layer10x10 {
        Layer10x10 {
            tiles: [[Tile::new_empty(); 10]; 10],
        }
    }
    pub fn new_border() -> Layer10x10 {
        let mut layer = Layer10x10::new_empty();
        for i in 0..10 {
            layer.put(Tile::new_block(), Position::new(0, i));
            layer.put(Tile::new_block(), Position::new(9, i));
            layer.put(Tile::new_block(), Position::new(i, 0));
            layer.put(Tile::new_block(), Position::new(i, 9));
        }
        layer
    }
    pub fn put(&mut self, tile: Tile, position: Position) {
        self.tiles[position.y as usize][position.x as usize] = tile;
    }
    fn get(&self, position: Position) -> Tile {
        self.tiles[position.y as usize][position.x as usize]
    }
    fn is_walkable(&self, position: Position) -> bool {
        self.get(position).walkable
    }
}

pub struct Arena10x10 {
    layers: Vec<Layer10x10>,
}
impl Arena10x10 {
    pub fn new() -> Arena10x10 {
        Arena10x10 { layers: Vec::new() }
    }
    pub fn add_layer(&mut self, layer: Layer10x10) {
        self.layers.push(layer);
    }
    fn is_walkable(&self, position: Position) -> bool {
        self.layers.iter().all(|layer| layer.is_walkable(position))
    }
}

pub fn observe_from(
    position: Position,
    arena: &Arena10x10,
    player: &Actor,
    npcs: &Vec<Actor>,
) -> Detection {
    let mut detect = Detection::new();
    for dir in &DIRECTIONS {
        let place = position + Direction::dir_to_diff(*dir);
        if arena.is_walkable(place) && player.is_not_in(place) && are_not_in(place, &npcs) {
            detect.walk_around.insert(*dir, true);
        }
    }
    detect
}

pub fn show(arena: &Arena10x10, player: &Actor, npcs: &Vec<Actor>) {
    for y in 0..10 {
        for x in 0..10 {
            let place = Position::new(x, y);
            if player.is_in(place) {
                print!("{}", player.img_char);
                continue;
            }
            if are_in(place, npcs) {
                for npc in npcs.iter() {
                    if npc.is_in(place) {
                        print!("{}", npc.img_char);
                        break;
                    }
                }
                continue;
            }
            let mut img_char = ' ';
            for layer in &arena.layers {
                img_char = layer.get(Position::new(x, y)).img_char;
                if img_char != ' ' {
                    break;
                }
            }
            print!("{}", img_char);
        }
        println!();
    }
}

pub fn tick(arena: &Arena10x10, player: &mut Actor, npcs: &mut Vec<Actor>) {
    player.tick(arena, npcs);
    for npc in npcs.iter_mut() {
        npc.tick(arena, npcs);
    }
}
