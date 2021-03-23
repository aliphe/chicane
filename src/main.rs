use std::io;

mod core;
mod domain;
mod application;

use crate::core::main_loop::MainLoop as Loop;

struct InputLoop {
    iteration: i32,
}

impl Loop for InputLoop {
    fn next(&mut self) -> bool {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input");

        self.iteration -= 1;

        if self.iteration > 0 {
            return true;
        }
        return false;
    }
}

fn main() {
    println!("Hello, world!");

    let mut main_loop = InputLoop { iteration: 10 };
    while main_loop.next() == true {
        println!("Next step")
    }
}
