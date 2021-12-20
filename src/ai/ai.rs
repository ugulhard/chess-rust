
mod ai;
use core::panic;
use std::cmp;

use crate::chess::{chess_move::{ChessMove, self}, board::Board, evaluation::evaluate, color::Color, game_result::GameResult};

pub struct minimax_ai {
    max_depth: i32
}

impl minimax_ai {
    pub fn new(max_depth: i32) -> minimax_ai {
        if max_depth <= 0 {
            panic!("Need to calculate at least one move deep");
        }
        minimax_ai{max_depth}
    }

    pub fn find_best_move(&self, board: &Board, color: Color) -> ChessMove {
        let mut chess_moves = board.legal_moves();
        let mut best_move = chess_moves.pop().expect("No legal moves, the game should be over");
        let mut best_value = evaluate(&board.make_move_with_struct(best_move)) as f64;
        for chess_move in chess_moves {
            let new_board = board.make_move_with_struct(chess_move);
            let move_value = minimax_ai::minimax(&new_board, self.max_depth -1, false, color);
            if minimax_ai::move_maximizes_for_color(color, best_value, move_value) {
                best_value = move_value;
                best_move = chess_move;
            }
        }

        best_move

    }

    fn minimax(board: &Board, depth: i32, maximizing_player: bool, color_to_maximize: Color) -> f64 {
        if depth == 0 || board.result() != GameResult::Ongoing {
            return evaluate(board) as f64;
        }
        let mut value = -10000.0;
        if maximizing_player {
            for chess_move in board.legal_moves(){
                let new_board = board.make_move_with_struct(chess_move);
                let node_value = minimax_ai::minimax(&new_board, depth -1, false, color_to_maximize);
                if minimax_ai::move_maximizes_for_color(color_to_maximize, value, node_value) {
                    value = node_value;
                }
            }
        } else {
            value = -value;
            for chess_move in board.legal_moves(){
                let new_board = board.make_move_with_struct(chess_move);
                let node_value = minimax_ai::minimax(&new_board, depth -1, false, color_to_maximize);
                if minimax_ai::move_minimizes_for_color(color_to_maximize, value, node_value) {
                    value = node_value;
                }
            }
        }
        value
    }

    fn move_maximizes_for_color(color_to_maximize: Color, best_value: f64, new_value: f64) -> bool{
        match color_to_maximize {
            Color::White => best_value - new_value < 0.001,
            Color::Black => best_value - new_value > 0.001,
            Color::Empty => panic!("Can't maximize for empty color")
        }
    }

    fn move_minimizes_for_color(color_to_minimize: Color, best_value: f64, new_value: f64) -> bool{
        match color_to_minimize {
            Color::White => best_value - new_value > 0.001,
            Color::Black => best_value - new_value < 0.001,
            Color::Empty => panic!("Can't maximize for empty color")
        }
    }
}
mod tests {
    use crate::{chess::{tile::Tile, board::Board, piece::Piece, color::Color, chess_move::ChessMove}, ai::ai::minimax_ai};
    #[test]
    #[should_panic]
    fn create_ai_with_depth_0_panics(){
        let ai = minimax_ai::new(0);

    }
    #[test]
    fn maximize(){
        assert!(!minimax_ai::move_maximizes_for_color(Color::White, 5.0, 0.0));
        assert!(minimax_ai::move_maximizes_for_color(Color::White, 0.0, 5.0));
        assert!(minimax_ai::move_maximizes_for_color(Color::Black, 5.0, 0.0));
        assert!(!minimax_ai::move_maximizes_for_color(Color::Black, 0.0, 5.0));
    }
    #[test]
    fn minimize(){
        assert!(minimax_ai::move_minimizes_for_color(Color::White, 5.0, 0.0));
        assert!(!minimax_ai::move_minimizes_for_color(Color::White, 0.0, 5.0));
        assert!(!minimax_ai::move_minimizes_for_color(Color::Black, 5.0, 0.0));
        assert!(minimax_ai::move_minimizes_for_color(Color::Black, 0.0, 5.0));
    }
    #[test]
    fn white_win(){
        let mut board = Board::empty();
        board.tiles[0][0] = Tile{piece: Piece::King, color: Color::Black};
        board.tiles[2][2] = Tile{piece: Piece::King, color: Color::White};
        board.tiles[2][1] = Tile{piece: Piece::Queen, color: Color::White};
        let ai = minimax_ai::new(3);
        let best_move = ai.find_best_move(&board, Color::White);
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
        let ai = minimax_ai::new(3);
        let best_move = ai.find_best_move(&board, Color::Black);
        let expected_move = ChessMove::from("2 1 1 1".to_string()).unwrap();
        assert_eq!(best_move, expected_move);
    }
    #[test]
    fn should_take_queen(){
        let mut board = Board::empty();
        board.tiles[1][0] = Tile{piece: Piece::King, color: Color::White};
        board.tiles[7][7] = Tile{piece: Piece::King, color: Color::Black};
        board.tiles[2][1] = Tile{piece: Piece::Queen, color: Color::Black};
        let ai = minimax_ai::new(3);
        let best_move = ai.find_best_move(&board, Color::White);
        let expected_move = ChessMove::from("1 0 2 1".to_string()).unwrap();
        assert_eq!(best_move, expected_move);
    }

}