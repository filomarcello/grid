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
}

pub struct Arena10x10 {
    pub player: Actor,
    pub npcs: Vec<Actor>,
    layers: Vec<Layer10x10>,
}
impl Arena10x10 {
    pub fn new(player: Actor) -> Arena10x10 {
        Arena10x10 {
            player,
            npcs: Vec::new(),
            layers: Vec::new(),
        }
    }
    pub fn add_layer(&mut self, layer: Layer10x10) {
        self.layers.push(layer);
    }
    pub fn add_npc(&mut self, actor: Actor) {
        self.npcs.push(actor);
    }
    fn observe_from(&self, position: Position) -> Detection {
        let mut detect = Detection::new();
        for dir in &DIRECTIONS {
            if self.is_walkable(position + Direction::dir_to_diff(*dir)) {
                detect.walk_around.insert(*dir, true);
            }
        }
        detect
    }
    fn npcs_detections(&mut self) -> Vec<Detection> {
        let mut detections = Vec::new();
        for npc in &self.npcs {
            detections.push(self.observe_from(npc.position));
        }
        detections
    }
    fn are_npcs_in(&self, position: Position) -> bool {
        self.npcs.iter().any(|npc| npc.position == position)
    }
    fn is_walkable(&self, position: Position) -> bool { //TODO: npcs superimpose!
        !(self.player.position == position)
            && !(self.are_npcs_in(position))
            && self.layers.iter().all(|layer| layer.is_walkable(position))
    }
    pub fn tick(&mut self) {
        let player_detect = self.observe_from(self.player.position);
        self.player.tick(player_detect);
        let npc_detections = self.npcs_detections();
        for (npc, detect) in self.npcs.iter_mut().zip(npc_detections) {
            npc.tick(detect);
        }
        // self.npcs.iter_mut().map(|npc| npc.tick(self.observe_from(npc.position)));
    }
    pub fn show(&self) {
        for y in 0..10 {
            for x in 0..10 {
                let place = Position::new(x, y);
                if self.player.position == place {
                    print!("{}", self.player.img_char);
                    continue;
                }
                if self.are_npcs_in(place) {
                    for npc in &self.npcs {
                        if npc.position == place {
                            print!("{}", npc.img_char);
                            break;
                        }
                    }
                    continue;
                }
                let mut img_char = ' ';
                for layer in &self.layers {
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
}
