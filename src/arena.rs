use crate::actor::{Actor, Direction};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Detection {
    // TODO: implement how to detect environment
    pub walk_around: HashMap<Direction, bool>,
}
impl Detection {
    pub fn new() -> Detection {
        let mut around = HashMap::new();
        around.insert(Direction::N, false);
        around.insert(Direction::E, false);
        around.insert(Direction::S, false);
        around.insert(Direction::W, false);
        around.insert(Direction::NE, false);
        around.insert(Direction::SE, false);
        around.insert(Direction::SW, false);
        around.insert(Direction::NW, false);
        Detection {
            walk_around: around,
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
            layer.put(Tile::new_block(), 0, i);
            layer.put(Tile::new_block(), 9, i);
            layer.put(Tile::new_block(), i, 0);
            layer.put(Tile::new_block(), i, 9);
        }
        layer
    }
    pub fn put(&mut self, tile: Tile, x: u32, y: u32) {
        self.tiles[y as usize][x as usize] = tile;
    }
    fn get(&self, x: u32, y: u32) -> Tile {
        self.tiles[y as usize][x as usize]
    }
    fn is_walkable(&self, x: u32, y: u32) -> bool {
        self.get(x, y).walkable
    }
    fn is_not_walkable(&self, x: u32, y: u32) -> bool {
        !self.is_walkable(x, y)
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
    fn player_pos(&self) -> (u32, u32) {
        (self.player.x, self.player.y)
    }
    fn player_pos_x(&self) -> u32 {
        self.player_pos().0
    }
    fn player_pos_y(&self) -> u32 {
        self.player_pos().1
    }
    pub fn player_observe(&self) -> Detection {
        // TODO: rewrite code in rustacean
        let mut detect = Detection::new();
        detect.walk_around.insert(
            Direction::N,
            self.is_walkable(self.player_pos_x(), self.player_pos_y() - 1),
        );
        detect.walk_around.insert(
            Direction::NE,
            self.is_walkable(self.player_pos_x() + 1, self.player_pos_y() - 1),
        );
        detect.walk_around.insert(
            Direction::E,
            self.is_walkable(self.player_pos_x() + 1, self.player_pos_y()),
        );
        detect.walk_around.insert(
            Direction::SE,
            self.is_walkable(self.player_pos_x() + 1, self.player_pos_y() + 1),
        );
        detect.walk_around.insert(
            Direction::S,
            self.is_walkable(self.player_pos_x(), self.player_pos_y() + 1),
        );
        detect.walk_around.insert(
            Direction::SW,
            self.is_walkable(self.player_pos_x() - 1, self.player_pos_y() + 1),
        );
        detect.walk_around.insert(
            Direction::W,
            self.is_walkable(self.player_pos_x() - 1, self.player_pos_y()),
        );
        detect.walk_around.insert(
            Direction::NW,
            self.is_walkable(self.player_pos_x() - 1, self.player_pos_y() - 1),
        );
        detect
    }
    pub fn add_layer(&mut self, layer: Layer10x10) {
        self.layers.push(layer);
    }
    pub fn show(&self) {
        for y in 0..10 {
            for x in 0..10 {
                if self.player_pos() == (x, y) {
                    print!("{}", self.player.img_char);
                } else {
                    let mut img_char = ' ';
                    for layer in &self.layers {
                        img_char = layer.get(x, y).img_char;
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
    fn is_walkable(&self, x: u32, y: u32) -> bool {
        if self.player_pos() == (x, y) {
            return false;
        }
        let mut walkable = true;
        for layer in &self.layers {
            if layer.is_not_walkable(x, y) {
                walkable = false;
            }
        }
        walkable
    }
    pub fn tick(&mut self) {
        self.player.operate();
    }
}
