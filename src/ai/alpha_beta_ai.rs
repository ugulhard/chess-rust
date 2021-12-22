
use core::panic;
use crate::{ai::ai::Ai, chess::{tile::Tile, piece::Piece}};
use std::{f64::{MIN, MAX}, sync::Mutex};
use log::info;
use crate::chess::{chess_move::ChessMove, board::Board, evaluation::evaluate, color::Color, game_result::GameResult};

pub struct AlphaBetaAi {
    max_depth: i32,
    color: Color,
    alpha: Mutex<f64>,
    beta: Mutex<f64>,
    best_value: Mutex<f64>,
    best_move: Mutex<ChessMove>
}

impl AlphaBetaAi {
    pub fn new(color: Color, max_depth: i32) -> AlphaBetaAi {
        if max_depth <= 0 {
            panic!("Need to calculate at least one move deep");
        }
        if color == Color::Empty {
            panic!("You can't play as an empty color");
        }
        AlphaBetaAi{color, max_depth, 
            alpha: Mutex::new(f64::MIN + 500.0), beta: Mutex::new(f64::MAX - 500.0), 
            best_value: Mutex::new(f64::MIN+ 500.0), 
            best_move: Mutex::new(ChessMove::from("0 0 1 1".to_string()).unwrap())}
    }

    fn alpha_beta_search(&mut self, board: &Board, depth: i32, maximizing_player: bool) -> f64 {
        if depth == 0 || board.result() != GameResult::Ongoing {
            return self.evaluate(board) + depth as f64;
        }
        if maximizing_player {
            let mut value = MIN;
            let moves = board.legal_moves();
            for chess_move in board.legal_moves().into_iter(){
                let new_board = board.make_move_with_struct(chess_move);
                let node_value = self.alpha_beta_search(&new_board, depth -1, false);
                if self.is_larger(&value, &node_value) {
                    value = node_value;
                }
                if depth == self.max_depth {
                    let mut best_value = self.best_value.lock().unwrap();
                    if self.is_larger(&best_value, &node_value) {
                        let mut best_move = self.best_move.lock().unwrap();
                        *best_value = node_value;
                        *best_move = chess_move; 
                    }
                }

                let beta_result = self.beta.lock();
                if self.is_larger( &beta_result.unwrap(), &value) {
                    break;
                }
                
                let lock = self.alpha.lock();
                let mut alpha = lock.unwrap();
                if self.is_larger(&alpha, &value) {
                    *alpha = value;
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
                {
                    let lock = self.alpha.lock();
                    if self.is_larger(&value, &lock.unwrap()) {
                        break;
                    }
                }
                {
                    let lock = self.beta.lock();
                    let mut result = lock.unwrap();
                    if self.is_larger(&result, &value) {
                        *result = value;
                    }
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

    fn is_larger(&self, value_1: &f64, value_2: &f64) -> bool{
        value_1 - value_2 < 0.00001
    }

    fn is_smaller(&self, worst_value: &f64, new_value: &f64) -> bool{
        worst_value - new_value > 0.00001
    }
}

impl Ai for AlphaBetaAi {
    fn find_best_move(&mut self, board: &Board) -> ChessMove {
        self.alpha_beta_search(board, self.max_depth, true);
        let best_move = self.best_move.lock().unwrap();
        let best_value = self.best_value.lock().unwrap();
        info!("Picked the move {} with value {}", best_move, best_value);
        *best_move

    }
}

#[test]
#[should_panic]
fn create_ai_with_depth_0_panics(){
    let _ai = AlphaBetaAi::new(Color::Black, 0);

}
#[test]
#[should_panic]
fn create_ai_with_empty_color_panics(){
    let _ai = AlphaBetaAi::new(Color::Empty, 1);

}
#[test]
fn maximize(){
    let ai = AlphaBetaAi::new(Color::White, 1);
    assert!(!ai.is_larger(&5.0, &0.0));
    assert!(ai.is_larger(&0.0, &5.0));
    assert!(!ai.is_larger(&0.0, &-5.0));
    assert!(ai.is_larger(&-5.0, &0.0));
}
#[test]
fn minimize(){
    let ai = AlphaBetaAi::new(Color::White, 1);
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
    let mut ai = AlphaBetaAi::new(Color::White,3);
    let best_move = ai.find_best_move(&board);
    let expected_move = ChessMove::from("2 1 1 1".to_string()).unwrap();
    assert_eq!(expected_move, best_move);
}
#[test]
fn black_win(){
    let mut board = Board::empty();
    board.tiles[1][0] = Tile{piece: Piece::King, color: Color::White};
    board.tiles[2][2] = Tile{piece: Piece::King, color: Color::Black};
    board.tiles[2][1] = Tile{piece: Piece::Queen, color: Color::Black};
    board = board.make_move(1, 0, 0, 0);
    let mut ai = AlphaBetaAi::new(Color::Black, 3);
    let best_move = ai.find_best_move(&board);
    let expected_move = ChessMove::from("2 1 1 1".to_string()).unwrap();
    assert_eq!(expected_move, best_move);
}
#[test]
fn should_take_queen(){
    let mut board = Board::empty();
    board.tiles[1][0] = Tile{piece: Piece::King, color: Color::White};
    board.tiles[0][2] = Tile{piece: Piece::King, color: Color::Black};
    board.tiles[2][1] = Tile{piece: Piece::Queen, color: Color::Black};
    let mut ai = AlphaBetaAi::new(Color::White, 3);
    let best_move = ai.find_best_move(&board);
    let expected_move = ChessMove::from("1 0 2 1".to_string()).unwrap();
    assert_eq!(expected_move, best_move);
}

#[test]
fn should_prevent_scholars_mate_depth_3(){
    let mut board = Board::new();
    board = board.make_move(4, 1, 5, 2);
    board = board.make_move(1, 7, 0, 5);
    board = board.make_move(3, 0, 5, 2);
    board = board.make_move(0, 5, 1, 7);
    board = board.make_move(5, 0, 2, 3);
    let mut ai = AlphaBetaAi::new(Color::Black, 3);
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
    let mut ai = AlphaBetaAi::new(Color::Black, 4);
    let best_move = ai.find_best_move(&board);
    board = board.make_move_with_struct(best_move);
    if board.legal_move(5, 2, 5, 6) {
        board = board.make_move(5, 2, 5, 6);
        assert_eq!(GameResult::Ongoing, board.result());
    }
}
