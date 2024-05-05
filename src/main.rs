use crate::snake::Snake;

mod snake;

fn main() {
    println!("Hello, world!");
    let mut snake1: Snake = Snake::new();
    snake1.eat();
    snake1.eat();
    snake1.eat();
    snake1.eat();
    snake1.eat();

    snake1.translate(1.0, 1.0);
    snake1.translate(-1.0, 1.0);
}
