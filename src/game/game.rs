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

    pub fn make_move(&mut self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) {
        self.board = self.board.make_move(start_x, start_y, end_x, end_y);
    }
}

impl fmt::Display for Game {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", self.board)
    }
}