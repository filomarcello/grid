struct Actor {
    x: i32,
    y: i32,
}
impl Actor {
    fn new(x: i32, y: i32) -> Actor {
        Actor { x, y }
    }
}

struct Arena {
    width: i32,
    height: i32,
    actor: Actor,
}
impl Arena {
    fn new(width: i32, height: i32, actor: Actor) -> Arena {
        Arena {
            width,
            height,
            actor,
        }
    }
    fn show(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                if self.actor.x == x && self.actor.y == y {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn main() {
    let act = Actor::new(5, 5);
    let arena = Arena::new(10, 10, act);
    arena.show();
}
