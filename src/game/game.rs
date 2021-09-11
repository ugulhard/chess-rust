use crate::chess::chess_move::ChessMove;
use crate::chess::board::Board;

enum GameResult {
WhiteWin,
BlackWin,
Draw,
Ongoing
}
struct Game {
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
}