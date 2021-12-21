mod game;
mod chess;
pub mod ai;

use std::io::Write;
use std::io::stdout;
use std::io::stdin;

use chess::color::Color;

use crate::game::game::Game;
use crate::chess::chess_move::ChessMove;
use crate::ai::ai::MinimaxAi;


fn main() {
    let mut game = Game::new();
    let ai = MinimaxAi::new(9);
    loop {
        let mut input=String::new();
        println!("Please enter the next move: ");
        let _=stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter a correct string");
        let chess_move_string = input.replace("\n", "");
        let parsed_chess_move = ChessMove::from(chess_move_string);
        match parsed_chess_move {
            None => println!("Incorrect move"),
            Some(chess_move) => make_move(&mut game, &chess_move, &ai)

        }
    
        print!("{}", game);
    }
}

fn make_move(game: &mut Game, chess_move: &ChessMove, ai: &MinimaxAi) {
    if game.legal_move(chess_move) {
        game.make_move(chess_move);
    }
    let best_move = ai.find_best_move(&game.board, Color::Black);
    game.make_move(&best_move)
}
