
use core::panic;
use crate::ai::ai::Ai;
use std::f64::{MIN, MAX};
use log::info;
use crate::chess::{chess_move::ChessMove, board::Board, evaluation::evaluate, color::Color, game_result::GameResult};

pub struct AlphaBetaAi<'a> {
    max_depth: i32,
    color: Color,
    alpha: &'a mut f64,
    beta: &'a mut f64,
}

impl <'a>AlphaBetaAi<'a> {
    pub fn new(color: Color, max_depth: i32, alpha: &'a mut f64, beta: &'a mut f64) -> AlphaBetaAi<'a> {
        if max_depth <= 0 {
            panic!("Need to calculate at least one move deep");
        }
        if color == Color::Empty {
            panic!("You can't play as an empty color");
        }
        AlphaBetaAi{color, max_depth, alpha, beta}
    }

    fn alpha_beta_search(&mut self, board: &Board, depth: i32, maximizing_player: bool) -> f64 {
        if depth == 0 || board.result() != GameResult::Ongoing {
            return self.evaluate(board) + depth as f64;
        }
        if maximizing_player {
            let mut value = MIN;
            for chess_move in board.legal_moves(){
                let new_board = board.make_move_with_struct(chess_move);
                let node_value = self.alpha_beta_search(&new_board, depth -1, false);
                if self.is_larger(&value, &node_value) {
                    value = node_value;
                }
                if self.is_larger(&value, self.beta) {
                    break;
                }
                if self.is_larger(self.alpha, &value) {
                    *self.alpha = value;
                }

            }
            value
        } else {
            let mut value = MAX;
            for chess_move in board.legal_moves(){
                let new_board = board.make_move_with_struct(chess_move);
                let node_value = self.alpha_beta_search(&new_board, depth -1, true);
                if self.is_smaller(&value, &node_value) {
                    value = node_value;
                }
                if self.is_smaller(&value, self.alpha) {
                    break;
                }
                if self.is_smaller(self.beta, &value) {
                    *self.beta = value;
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

    fn is_larger(&self, best_value: &f64, new_value: &f64) -> bool{
        best_value - new_value < 0.00001
           
    }

    fn is_smaller(&self, worst_value: &f64, new_value: &f64) -> bool{
        worst_value - new_value > 0.00001
    }
}

impl Ai for AlphaBetaAi<'_> {
    fn find_best_move(&mut self, board: &Board) -> ChessMove {
        let mut chess_moves = board.legal_moves();
        let mut best_move = chess_moves.pop().expect("No legal moves, the game should be over");
        let new_board = board.make_move_with_struct(best_move);
        let mut best_value = self.alpha_beta_search(&new_board, self.max_depth - 1, false);
        for chess_move in chess_moves {
            let new_board = board.make_move_with_struct(chess_move);
            let move_value = self.alpha_beta_search(&new_board, self.max_depth -1,  false);
            if self.is_larger(&best_value, &move_value) {
                best_value = move_value;
                best_move = chess_move;
            }
        }
        info!("Picked the move {} with value {}", best_move, best_value);
        best_move

    }
}

mod tests {
    use crate::{chess::{tile::Tile, board::Board, piece::Piece, color::Color, chess_move::ChessMove, game_result::GameResult}, ai::{alpha_beta_ai::AlphaBetaAi, ai::Ai}};
    #[test]
    #[should_panic]
    fn create_ai_with_depth_0_panics(){
        let alpha = &mut f64::MIN.clone();
        let beta = &mut f64::MIN.clone();
        let _ai = AlphaBetaAi::new(Color::Black, 0, alpha, beta);

    }
    #[test]
    #[should_panic]
    fn create_ai_with_empty_color_panics(){
        let alpha = &mut f64::MIN.clone();
        let beta = &mut f64::MIN.clone();
        let _ai = AlphaBetaAi::new(Color::Empty, 1, alpha, beta);

    }
    #[test]
    fn maximize(){
        let alpha = &mut f64::MIN.clone();
        let beta = &mut f64::MIN.clone();
        let ai = AlphaBetaAi::new(Color::White, 1, alpha, beta);
        assert!(!ai.is_larger(&5.0, &0.0));
        assert!(ai.is_larger(&0.0, &5.0));
        assert!(!ai.is_larger(&0.0, &-5.0));
        assert!(ai.is_larger(&-5.0, &0.0));
    }
    #[test]
    fn minimize(){
        let alpha = &mut f64::MIN.clone();
        let beta = &mut f64::MIN.clone();
        let ai = AlphaBetaAi::new(Color::White, 1, alpha, beta);
        assert!(ai.is_smaller(&5.0, &0.0));
        assert!(!ai.is_smaller(&0.0, &5.0));
        assert!(ai.is_smaller(&-5.0, &-10.0));
        assert!(!ai.is_smaller(&-5.0, &0.0));

    }
    #[test]
    fn white_win(){
        let mut board = Board::empty();
        board.tiles[0][0] = Tile{piece: Piece::King, color: Color::Black};
        board.tiles[2][2] = Tile{piece: Piece::King, color: Color::White};
        board.tiles[2][1] = Tile{piece: Piece::Queen, color: Color::White};
        let alpha = &mut f64::MIN.clone();
        let beta = &mut f64::MIN.clone();
        let mut ai = AlphaBetaAi::new(Color::White,3, alpha, beta);
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
        let alpha = &mut f64::MIN.clone();
        let beta = &mut f64::MIN.clone();
        let mut ai = AlphaBetaAi::new(Color::Black, 3, alpha, beta);
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
        let alpha = &mut f64::MIN.clone();
        let beta = &mut f64::MIN.clone();
        let mut ai = AlphaBetaAi::new(Color::White, 3, alpha, beta);
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
        let alpha = &mut f64::MIN.clone();
        let beta = &mut f64::MIN.clone();
        let mut ai = AlphaBetaAi::new(Color::Black, 3, alpha, beta);
        let best_move = ai.find_best_move(&board);
        board = board.make_move_with_struct(best_move);
        if board.legal_move(5, 2, 5, 6) {
            board = board.make_move(5, 2, 5, 6);
            assert_eq!(GameResult::Ongoing, board.result());
        }
    }

    #[test]
    fn should_prevent_scholars_mate_depth_4(){
        let mut board = Board::new();
        board = board.make_move(4, 1, 5, 2);
        board = board.make_move(1, 7, 0, 5);
        board = board.make_move(3, 0, 5, 2);
        board = board.make_move(0, 5, 1, 7);
        board = board.make_move(5, 0, 2, 3);
        let alpha = &mut f64::MIN.clone();
        let beta = &mut f64::MIN.clone();
        let mut ai = AlphaBetaAi::new(Color::Black, 4, alpha, beta);
        let best_move = ai.find_best_move(&board);
        board = board.make_move_with_struct(best_move);
        if board.legal_move(5, 2, 5, 6) {
            board = board.make_move(5, 2, 5, 6);
            assert_eq!(GameResult::Ongoing, board.result());
        }
    }

}