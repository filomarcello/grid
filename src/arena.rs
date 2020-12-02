use crate::actor::Actor;
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
    fn is_not_walkable(&self, position: Position) -> bool {
        !self.is_walkable(position)
    }
}

pub struct Arena10x10 {
    pub player: Actor,
    layers: Vec<Layer10x10>,
}
impl Arena10x10 {
    pub fn new(player: Actor) -> Arena10x10 {
        Arena10x10 {
            player,
            layers: Vec::new(),
        }
    }
    fn player_pos(&self) -> Position {
        self.player.position
    }
    pub fn player_observe(&self) -> Detection {
        let mut detect = Detection::new();
        for dir in &DIRECTIONS {
            if self.is_walkable(self.player_pos() + Direction::dir_to_diff(*dir)) {
                detect.walk_around.insert(*dir, true);
            }
        }
        detect
    }
    pub fn add_layer(&mut self, layer: Layer10x10) {
        self.layers.push(layer);
    }
    pub fn show(&self) {
        for y in 0..10 {
            for x in 0..10 {
                if self.player_pos() == Position::new(x, y) {
                    print!("{}", self.player.img_char);
                } else {
                    let mut img_char = ' ';
                    for layer in &self.layers {
                        img_char = layer.get(Position::new(x, y)).img_char;
                        if img_char != ' ' {
                            break;
                        }
                    }
                    print!("{}", img_char);
                }
            }
            println!();
        }
    }
    fn is_walkable(&self, position: Position) -> bool {
        if self.player_pos() == position {
            return false;
        }
        let mut walkable = true; // TODO: rewrite in rustacean
        for layer in &self.layers {
            if layer.is_not_walkable(position) {
                walkable = false;
            }
        }
        walkable
    }
    pub fn tick(&mut self) {
        self.player_observe();
        // self.player.operate(self.player.think(self.player_observe()));
    }
}
