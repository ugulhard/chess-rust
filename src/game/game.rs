use crate::chess::chess_move::ChessMove;
use crate::chess::board::Board;
use std::fmt;

enum GameResult {
WhiteWin,
BlackWin,
Draw,
Ongoing
}
pub struct Game {
    moves: Vec<ChessMove>,
    result: GameResult,
    board: Board
}

impl Game {
    pub fn new() -> Game {
        Game {moves: Vec::<ChessMove>::new(),
        result: GameResult::Ongoing,
        board: Board::new()}
    }

    pub fn make_move(&mut self, chess_move: &ChessMove) {
        self.moves.push(chess_move.clone());
        self.board = self.board.make_move(
            chess_move.start_pos.0, chess_move.start_pos.1,
             chess_move.end_pos.0, chess_move.end_pos.1);
    }

    pub fn legal_move(&self, chess_move: &ChessMove) -> bool {
        self.board.legal_move(chess_move.start_pos.0, chess_move.start_pos.1,
            chess_move.end_pos.0, chess_move.end_pos.1)
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}