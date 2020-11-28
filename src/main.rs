use rand::Rng;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct Actor {
    x: u32,
    y: u32,
    img_char: char,
    dir: Direction,
}
impl Actor {
    fn new(x: u32, y: u32, img_char: char) -> Actor {
        Actor {
            x,
            y,
            img_char,
            dir: Direction::Hold,
        }
    }
    fn redir(&mut self, direction: Direction) {
        self.dir = direction;
    }
    fn step(&mut self) {
        match self.dir {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Hold => {}
        }
    }
    fn think(&self) -> Direction {
        let dir = rand::thread_rng().gen_range(0, 5);
        match dir {
            0 => Direction::Hold,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            4 => Direction::Up,
            _ => Direction::Hold,
        }
    }
    fn operate(&mut self) {
        self.redir(self.think());
        self.step();
    }
}
#[derive(Clone, Copy)]
struct Tile {
    img_char: char,
    walkable: bool,
}
impl Tile {
    fn new(img_char: char, walkable: bool) -> Tile {
        Tile { img_char, walkable }
    }
    fn new_empty() -> Tile {
        Tile::new(' ', true)
    }
    fn new_block() -> Tile {
        Tile::new('█', false)
    }
}

struct Layer10x10 {
    tiles: [[Tile; 10]; 10],
}
impl Layer10x10 {
    fn new_empty() -> Layer10x10 {
        Layer10x10 {
            tiles: [[Tile::new_empty(); 10]; 10],
        }
    }
    fn new_border() -> Layer10x10 {
        let mut layer = Layer10x10::new_empty();
        for i in 0..10 {
            layer.put(Tile::new_block(), 0, i);
            layer.put(Tile::new_block(), 9, i);
            layer.put(Tile::new_block(), i, 0);
            layer.put(Tile::new_block(), i, 9);
        }
        layer
    }
    fn put(&mut self, tile: Tile, x: u32, y: u32) {
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

struct Arena10x10 {
    player: Actor,
    layers: Vec<Layer10x10>,
}
impl Arena10x10 {
    fn new(player: Actor) -> Arena10x10 {
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
    fn add_layer(&mut self, layer: Layer10x10) {
        self.layers.push(layer);
    }
    fn show(&self) {
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
    fn is_not_walkable(&self, x: u32, y: u32) -> bool {
        !self.is_walkable(x, y)
    }
    fn tick(&mut self) {
        self.player.operate();
    }
}

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    Hold,
}

// utilities
fn clrscr() {
    print!("\x1B[2J\x1B[1;1H"); // clears screen
}

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
