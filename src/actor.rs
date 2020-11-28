use rand::Rng;

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

enum Action {
    Hold,
    Move,
}

#[derive(Clone)]
pub struct Actor {
    pub x: u32,
    pub y: u32,
    pub img_char: char,
    dir: Direction,
}

impl Actor {
    pub fn new(x: u32, y: u32, img_char: char) -> Actor {
        Actor {
            x,
            y,
            img_char,
            dir: Direction::Left,
        }
    }
    pub fn redir(&mut self, direction: Direction) {
        self.dir = direction;
    }
    fn step(&mut self) {
        match self.dir {
            Direction::Up => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
        }
    }
    pub fn think(&self) -> Action {
        Action::Hold // TODO: get an input and decide what to do
    }
    pub fn operate(&mut self) {
        if self.think() != Action::Hold {
            // TODO: do something
            self.step(); // TODO: possibly step
        }
    }
}
