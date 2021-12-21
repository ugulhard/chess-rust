use std::io::Write;
use std::io::stdout;
use std::io::stdin;

use chess_rust::chess::chess_move::ChessMove;
use chess_rust::ai::alpha_beta_ai::AlphaBetaAi;
use chess_rust::chess::color::Color;
use chess_rust::game::game::Game;
use chess_rust::ai::ai::Ai;


fn main() {
    env_logger::init();
    let mut game = Game::new();
    let alpha = &mut f64::MIN.clone();
    let beta = &mut f64::MAX.clone();
    let mut ai = AlphaBetaAi::new(Color::Black,9, alpha, beta);
    loop {
        let mut input=String::new();
        println!("Please enter the next move: ");
        let _=stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter a correct string");
        let chess_move_string = input.replace("\n", "");
        let parsed_chess_move = ChessMove::from(chess_move_string);
        match parsed_chess_move {
            None => println!("Incorrect move"),
            Some(chess_move) => make_move(&mut game, &chess_move, &mut ai)
        }
    
        print!("{}", game);
    }
}

fn make_move(game: &mut Game, chess_move: &ChessMove, ai: &mut Ai) {
    if game.legal_move(chess_move) {
        game.make_move(chess_move);
    }
    let best_move = ai.find_best_move(&game.board);
    game.make_move(&best_move)
}
