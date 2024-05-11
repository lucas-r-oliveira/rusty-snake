use std::{
    io,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crate::snake::Snake;

mod snake;

fn main() {
    let mut snake: Snake = Snake::new();
    println!("WELCOME TO THE RUSTY SNAKE GAME!");
    // TODO: next steps
    // - a ticking clock. some sort of event loop?
    // - save last move. if a new command hasn't been sent by the end of the cycle
    //      move in the same direction as the last move
    // - screen boundaries?

    // - "canvas" in the terminal

    /*let (tx_input, rx_input) = mpsc::channel();

    thread::spawn(move || {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        tx_input.send(input).unwrap();
    });

    let time_limit = Duration::from_secs(5);

    match rx_input.recv_timeout(time_limit) {
        Ok(input) => {
            println!("Received user input: {}", input.trim());
        }
        Err(mpsc::RecvTimeoutError::Timeout) => {
            println!("Time's up! No input received...");
        }
        Err(e) => {
            println!("Error! {}", e);
        }
    }*/

    /*let time_limit = Duration::from_secs(2);
    loop {
        let start_time = Instant::now();

        // Get user input with timeout check
        let mut input = String::new();
        while let Err(_) = io::stdin().read_line(&mut input) {
            let elapsed = start_time.elapsed();
            if elapsed >= time_limit {
                println!("Time out!");
                // time out logic
                break;
            }
        }

        let input = input.trim().to_owned();
        // game logic goes here
        println!("user input: {}", input);
    }*/

    //let mut input = String::new();
    let time_limit = Duration::from_secs(2);
    let mut start_time = Instant::now();

    let (tx_input, rx_input) = mpsc::channel();

    thread::spawn(move || loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        tx_input.send(input).unwrap();
    });

    loop {
        match rx_input.try_recv() {
            Ok(input) => {
                println!("Got user input: {}", input.trim());
                //user input logic;
                //input.clear();
                start_time = Instant::now();
            }
            Err(_error) => {
                //TODO: differentiate errors?
                if start_time.elapsed() >= time_limit {
                    // time out logic
                    println!("Time's up!");
                    start_time = Instant::now();
                }
            }
        }
    }
    // -- just an example of how timers and loops work
    //while start_time.elapsed() < time_limit {
    //    println!("Trapped inside an endless loop aaaaaaag");
    //}
    //println!("Finally I am FREE!");

    //loop {
    //    println!("OH NO! I'M STUCK AGAIN AAAAAAAAHHH!");
    //}

    /*loop {
        //println!("WHEN DOES THIS END?!");
        if start_time.elapsed() >= time_limit {
            println!("Time out!");
        }
        match rx_input.try_recv() {
            Ok(mut input) => {
                println!("Got user input: {}", input.trim());
                //user input logic;
                //input.clear();
                //start_time = Instant::now();
            }
            Err(error) => {
                println!("Error reading user input! error: {}", error);
            }
        }
    }*/

    //loop {
    //    println!("new loop");

    //    let elapsed = start_time.elapsed();
    //    dbg!(elapsed.as_secs());

    //    if elapsed >= time_limit {
    //        println!("Time out!");
    //        //time out logic;
    //        start_time = Instant::now();
    //        continue;
    //    }
    //    //TODO: review unwrap()

    //    match io::stdin().read_line(&mut input) {
    //        Ok(_) => {
    //            println!("Got user input: {}", input.trim());
    //            //user input logic;
    //            input.clear();
    //            start_time = Instant::now();
    //        }
    //        Err(error) => {
    //            println!("Error reading user input! error: {}", error);
    //        }
    //    }
    //}

    /*loop {
        //let mut input = String::new();
        println!("Where do you want to move? [u | d | l | r | x]");
        println!("-- u: UP\n-- d: DOWN\n-- l: LEFT\n-- r: RIGHT\n-- x: QUIT");


        //while input.trim() != "u"
        //    || input.trim() != "d"
        //    || input.trim() != "l"
        //    || input.trim() != "r"
        //    || input.trim() != "x"
        //{
        //    input.clear();
        //    //TODO: check how to do error handling in Rust
        //    io::stdin().read_line(&mut input).unwrap();

        //    if input.trim() == "u" {
        //        snake.translate(0.0, 1.0)
        //    } else if input.trim() == "d" {
        //        snake.translate(0.0, -1.0)
        //    } else if input.trim() == "l" {
        //        snake.translate(-1.0, 0.0)
        //    } else if input.trim() == "r" {
        //        snake.translate(1.0, 0.0)
        //    } else if input.trim() == "x" {
        //        return;
        //    }
        //}
    }*/
}
