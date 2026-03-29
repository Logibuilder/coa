use crate::game::Game;
use std::io;


pub struct Nim {
    turn: usize,
    max_turns: usize,
}

impl Game for Nim {
    fn new() -> Self {
        Nim {
            turn: 0,
            max_turns: 3,
        }
    }

    fn play(&mut self) {
        self.turn += 1;
        println!("Yours action");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

    }

    fn is_over(&self) -> bool {
        self.turn >= self.max_turns
    }

    fn display(&self) {
        println!("Turn: {}, Max_turns: {}", self.turn, self.max_turns);
    }
}