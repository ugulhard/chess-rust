use crate::chess::{board::Board, chess_move::ChessMove};


pub trait Ai {
    fn find_best_move(&mut self, board: &Board) -> ChessMove;
}