
use core::panic;
use std::f64::{MIN, MAX};
use log::info;
use crate::chess::{chess_move::ChessMove, board::Board, evaluation::evaluate, color::Color, game_result::GameResult};

pub struct MinimaxAi {
    max_depth: i32,
    color: Color
}

impl MinimaxAi {
    pub fn new(color: Color, max_depth: i32) -> MinimaxAi {
        if max_depth <= 0 {
            panic!("Need to calculate at least one move deep");
        }
        if color == Color::Empty {
            panic!("You can't play as an empty color");
        }
        MinimaxAi{color, max_depth}
    }

    pub fn find_best_move(&self, board: &Board) -> ChessMove {
        let mut chess_moves = board.legal_moves();
        let mut best_move = chess_moves.pop().expect("No legal moves, the game should be over");
        let new_board = board.make_move_with_struct(best_move);
        let mut best_value = self.minimax(&new_board, self.max_depth - 1, false);
        for chess_move in chess_moves {
            let new_board = board.make_move_with_struct(chess_move);
            let move_value = self.minimax(&new_board, self.max_depth -1, false);
            if self.move_maximizes(best_value, move_value) {
                best_value = move_value;
                best_move = chess_move;
            }
        }
        info!("Picked the move {} with value {}", best_move, best_value);
        best_move

    }

    fn minimax(&self, board: &Board, depth: i32, maximizing_player: bool) -> f64 {
        if depth == 0 || board.result() != GameResult::Ongoing {
            return self.evaluate(board) + depth as f64;
        }
        if maximizing_player {
            let mut value = MIN;
            for chess_move in board.legal_moves(){
                let new_board = board.make_move_with_struct(chess_move);
                let node_value = self.minimax(&new_board, depth -1, false);
                if self.move_maximizes(value, node_value) {
                    value = node_value;
                }
            }
            value
        } else {
            let mut value = MAX;
            for chess_move in board.legal_moves(){
                let new_board = board.make_move_with_struct(chess_move);
                let node_value = self.minimax(&new_board, depth -1, true);
                if self.move_minimizes(value, node_value) {
                    value = node_value;
                }
            }
            value
        }
    }

    fn evaluate(&self, board: &Board) -> f64 {
        match self.color {
            Color::White => evaluate(board) as f64,
            Color::Black => -evaluate(board) as f64,
            _ => panic!("Can only evaluate for players")
        }
    }

    fn move_maximizes(&self, best_value: f64, new_value: f64) -> bool{
        best_value - new_value < 0.00001
           
    }

    fn move_minimizes(&self, worst_value: f64, new_value: f64) -> bool{
        worst_value - new_value > 0.00001
    }
}
mod tests {
    use crate::{chess::{tile::Tile, board::Board, piece::Piece, color::Color, chess_move::ChessMove, game_result::GameResult}, ai::minimax_ai::MinimaxAi};
    #[test]
    #[should_panic]
    fn create_ai_with_depth_0_panics(){
        let _ai = MinimaxAi::new(Color::Black, 0);

    }
    #[test]
    #[should_panic]
    fn create_ai_with_empty_color_panics(){
        let _ai = MinimaxAi::new(Color::Empty, 1);

    }
    #[test]
    fn maximize(){
        let ai = MinimaxAi::new(Color::White, 1);
        assert!(!ai.move_maximizes(5.0, 0.0));
        assert!(ai.move_maximizes(0.0, 5.0));
        assert!(!ai.move_maximizes(0.0, -5.0));
        assert!(ai.move_maximizes(-5.0, 0.0));
    }
    #[test]
    fn minimize(){
        let ai = MinimaxAi::new(Color::White, 1);
        assert!(ai.move_minimizes(5.0, 0.0));
        assert!(!ai.move_minimizes(0.0, 5.0));
        assert!(ai.move_minimizes(-5.0, -10.0));
        assert!(!ai.move_minimizes(-5.0, 0.0));

    }
    #[test]
    fn white_win(){
        let mut board = Board::empty();
        board.tiles[0][0] = Tile{piece: Piece::King, color: Color::Black};
        board.tiles[2][2] = Tile{piece: Piece::King, color: Color::White};
        board.tiles[2][1] = Tile{piece: Piece::Queen, color: Color::White};
        let ai = MinimaxAi::new(Color::White,3);
        let best_move = ai.find_best_move(&board);
        let expected_move = ChessMove::from("2 1 1 1".to_string()).unwrap();
        assert_eq!(best_move, expected_move);
    }
    #[test]
    fn black_win(){
        let mut board = Board::empty();
        board.tiles[1][0] = Tile{piece: Piece::King, color: Color::White};
        board.tiles[2][2] = Tile{piece: Piece::King, color: Color::Black};
        board.tiles[2][1] = Tile{piece: Piece::Queen, color: Color::Black};
        board = board.make_move(1, 0, 0, 0);
        let ai = MinimaxAi::new(Color::Black, 3);
        let best_move = ai.find_best_move(&board);
        let expected_move = ChessMove::from("2 1 1 1".to_string()).unwrap();
        assert_eq!(best_move, expected_move);
    }
    #[test]
    fn should_take_queen(){
        let mut board = Board::empty();
        board.tiles[1][0] = Tile{piece: Piece::King, color: Color::White};
        board.tiles[7][7] = Tile{piece: Piece::King, color: Color::Black};
        board.tiles[2][1] = Tile{piece: Piece::Queen, color: Color::Black};
        let ai = MinimaxAi::new(Color::White, 3);
        let best_move = ai.find_best_move(&board);
        let expected_move = ChessMove::from("1 0 2 1".to_string()).unwrap();
        assert_eq!(best_move, expected_move);
    }

    #[test]
    fn should_prevent_scholars_mate_depth_3(){
        let mut board = Board::new();
        board = board.make_move(4, 1, 5, 2);
        board = board.make_move(1, 7, 0, 5);
        board = board.make_move(3, 0, 5, 2);
        board = board.make_move(0, 5, 1, 7);
        board = board.make_move(5, 0, 2, 3);
        let ai = MinimaxAi::new(Color::Black, 3);
        let best_move = ai.find_best_move(&board);
        board = board.make_move_with_struct(best_move);
        if board.legal_move(5, 2, 5, 6) {
            board = board.make_move(5, 2, 5, 6);
            assert_eq!(GameResult::Ongoing, board.result());
        }
    }

    #[test]
    fn should_prevent_scholars_mate_depth_2(){
        let mut board = Board::new();
        board = board.make_move(4, 1, 5, 2);
        board = board.make_move(1, 7, 0, 5);
        board = board.make_move(3, 0, 5, 2);
        board = board.make_move(0, 5, 1, 7);
        board = board.make_move(5, 0, 2, 3);
        let ai = MinimaxAi::new(Color::Black, 2);
        let best_move = ai.find_best_move(&board);
        board = board.make_move_with_struct(best_move);
        let legal_moves = board.legal_moves();
        if board.legal_move(5, 2, 5, 6) {
            board = board.make_move(5, 2, 5, 6);
            assert_eq!(GameResult::Ongoing, board.result());
        }
    }

}