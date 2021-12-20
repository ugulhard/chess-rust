mod game;
mod chess;
pub mod ai;

use std::io::Write;
use std::io::stdout;
use std::io::stdin;

use crate::game::game::Game;
use crate::chess::chess_move::ChessMove;


fn main() {
    let mut game = Game::new();
    loop {
        let mut input=String::new();
        println!("Please enter the next move: ");
        let _=stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter a correct string");
        let chess_move_string = input.replace("\n", "");
        let parsed_chess_move = ChessMove::from(chess_move_string);
        match parsed_chess_move {
            None => println!("Incorrect move"),
            Some(chess_move) => make_move(&mut game, &chess_move)
        }
        print!("{}", game);
    }
}

fn make_move(game: &mut Game, chess_move: &ChessMove) {
    if game.legal_move(chess_move) {
        game.make_move(chess_move);
    }
}
