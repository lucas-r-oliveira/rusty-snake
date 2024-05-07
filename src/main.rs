use std::io;

use crate::snake::Snake;

mod snake;

fn main() {
    let mut snake: Snake = Snake::new();
    println!("WELCOME TO THE RUSTY SNAKE GAME!");
    // TODO: next steps
    // - a ticking clock.
    // - save last move. if a new command hasn't been sent by the end of the cycle
    //      move in the same direction as the last move
    // - screen boundaries? 

    loop {
        let mut input = String::new();
        println!("Where do you want to move? [u | d | l | r | x]");
        println!("-- u: UP\n-- d: DOWN\n-- l: LEFT\n-- r: RIGHT\n-- x: QUIT");

        while input.trim() != "u"
            || input.trim() != "d"
            || input.trim() != "l"
            || input.trim() != "r"
            || input.trim() != "x"
        {
            input.clear();
            //TODO: check how to do error handling in Rust
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "u" {
                snake.translate(0.0, 1.0)
            } else if input.trim() == "d" {
                snake.translate(0.0, -1.0)
            } else if input.trim() == "l" {
                snake.translate(-1.0, 0.0)
            } else if input.trim() == "r" {
                snake.translate(1.0, 0.0)
            } else if input.trim() == "x" {
                return;
            }
        }
    }
}
