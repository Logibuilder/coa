use week3::game;
use week3::tictactoe;
use week3::nim;


use tictactoe::TicTacToe;
use nim::Nim;
use game::Game;
use game::game_loop;

fn main() {
    
    loop {
        let mut choice = String::new();
        
        println!("Which game do you want to play? (1 for tic tac toe, 2 for nim, q to quit)");

        std::io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let game = TicTacToe::new();
                game_loop(game);
            },
            "2" => {
                let game = Nim::new();
                game_loop(game);
                println!("Game over!");
            },
            "q" | "Q" => {
                println!("Goodbye!");
                break;
            },
            _ => println!("Invalid choice"),
        }
    }
}