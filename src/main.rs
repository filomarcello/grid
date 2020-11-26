use rand::Rng;
use std::thread;
use std::time::Duration;

#[derive(Clone)]
struct Actor {
    x: i32,
    y: i32,
    dir: Direction,
}
impl Actor {
    fn new(x: i32, y: i32) -> Actor {
        Actor { x, y, dir: Direction::Hold }
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
            Direction::Hold => {},
        }
    }
    fn think() -> Direction {
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
}
#[derive(Clone, Copy)]
struct Tile {
    img_char: char,
    walkable: bool,
}
impl Tile {
    fn new(img_char: char, walkable: bool) -> Tile {
        Tile{img_char, walkable}
    }
    fn new_empty() -> Tile {
        Tile::new(' ', true)
    }
    fn new_block() -> Tile {
        Tile::new('█', false)
    }
}

struct Layer10x10 {
    tiles: [ [Tile; 10] ; 10 ]
}
impl Layer10x10 {
    fn new_empty() -> Layer10x10 {
        Layer10x10{tiles: [ [Tile::new_empty(); 10] ; 10 ]}
    }
    fn new_border() -> Layer10x10 {
        let mut layer = Layer10x10::new_empty();
        for x in 0..10 {
            layer.put(Tile::new_block(), 0, x);
            layer.put(Tile::new_block(), 9, x);
        }
        layer

    }
    fn put(&mut self, tile: Tile, x: u32, y: u32) {
        self.tiles[y as usize][x as usize] = tile;
    }
}

struct Arena {
    width: i32,
    height: i32,
}
impl Arena {
    fn new(width: i32, height: i32) -> Arena {
        Arena { width, height }
    }
    fn show(&self, actor: &Actor) {
        for y in 0..self.width {
            for x in 0..self.height {
                if actor.x == x && actor.y == y {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
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

fn main() {
    let mut act = Actor::new(5, 5);
    let arena = Arena::new(10, 10);
    loop {
        print!("\x1B[2J\x1B[1;1H"); // clears screen
        arena.show(&act);
        act.redir(Actor::think());
        act.step();
        thread::sleep(Duration::from_millis(500));

    }
}
