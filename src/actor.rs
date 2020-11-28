use rand::Rng;

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
    Hold,
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
    pub fn operate(&mut self) {
        self.redir(self.think());
        self.step();
    }
}
