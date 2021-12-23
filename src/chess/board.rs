use super::{chess_move::ChessMove, color::Color, game_result::GameResult, move_generator::MoveGenerator, piece::Piece, tile::Tile};
use std::{fmt};

#[derive(Debug, Clone)]
pub struct Board {
    pub tiles: Vec<Vec<Tile>>,
    player_to_move: Color
}

fn get_piece_from_column(x: usize) -> Piece {
    match x {
        0 => Piece::Rook,
        1 => Piece::Knight,
        2 => Piece::Bishop,
        3 => Piece::Queen,
        4 => Piece::King,
        5 => Piece::Bishop,
        6 => Piece::Knight,
        7 => Piece::Rook,
        _ => panic!()
    }
}

fn get_piece_for_starting_tile( x: usize, y: usize ) -> Tile {
    match y {
        0 => Tile::new(Color::White, get_piece_from_column(x)),
        1 => Tile::new(Color::White, Piece::Pawn),
        6 => Tile::new(Color::Black, Piece::Pawn),
        7 => Tile::new(Color::Black, get_piece_from_column(x)),
        _ => Tile::new(Color::Empty, Piece::Empty)
    }
}

impl Board {
    //Standard chess board
    pub fn new() -> Board{
        let mut tiles = Vec::<Vec::<Tile>>::new();
        for x in 0..8 {
            tiles.push(Vec::<Tile>::new());
            for y in 0..8 {
                tiles[x].push(get_piece_for_starting_tile(x, y));
            }
        }
        Board {tiles, player_to_move: Color::White}
    }
    //Empty chess board for tests or other purposes
    pub fn empty() -> Board{
        let mut tiles = Vec::<Vec::<Tile>>::new();
        for x in 0..8 {
            tiles.push(Vec::<Tile>::new());
            for _y in 0..8 {
                tiles[x].push(Tile{piece: Piece::Empty, color: Color::Empty});
            }
        }
        Board {tiles, player_to_move: Color::White}
    }

    pub fn make_move(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> Board {
        let mut new_board =self.clone();
        new_board.player_to_move = Color::opposing_color(self.player_to_move);
        new_board.tiles[end_x][end_y] = self.tiles[start_x][start_y];
        new_board.tiles[start_x][start_y] = Tile::new(Color::Empty, Piece::Empty);
        new_board
    }

    pub fn make_move_with_struct(&self, chess_move: ChessMove) -> Board {
        self.make_move(chess_move.get_start_x(), chess_move.get_start_y(), chess_move.get_end_x(), chess_move.get_end_y())
    }

    pub fn legal_move(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        let color_before_move = self.tiles[start_x][start_y].color;
        if color_before_move != self.player_to_move {
            return false;
        }
        if MoveGenerator::new(self).piece_can_reach(start_x, start_y, end_x, end_y) {
            //Check if resulting position is check for the player that moved
            let new_board = self.make_move(start_x, start_y, end_x, end_y);
            return !new_board.is_check(color_before_move);
        }
        false
    }

    pub fn result(&self) -> GameResult {
        if self.legal_moves().is_empty() {
            if self.player_to_move == Color::White {
                if self.is_check(Color::White){
                    return GameResult::BlackWin
                } 
            } else if self.is_check(Color::Black) {
                    return GameResult::WhiteWin
                } 
                return GameResult::Draw

        }
        GameResult::Ongoing
    }

    pub fn legal_moves(&self) -> Vec<ChessMove> {
        MoveGenerator::new(self).possible_moves().into_iter()
        .filter(|chess_move| self.legal_move(chess_move.start_pos.0, chess_move.start_pos.1, chess_move.end_pos.0, chess_move.end_pos.1))
        .collect()
    }



    fn is_check(&self, color_to_check: Color) -> bool {
        let king_position: Option<(usize, usize)> = self.all_squares().into_iter().find(|(x ,y)| self.tiles[*x][*y].color == color_to_check && self.tiles[*x][*y].piece == Piece::King);
        let (king_x, king_y) = king_position.unwrap();
        self.all_squares().into_iter().any(|(x, y)| self.tiles[x][y].color != color_to_check && MoveGenerator::new(self).piece_can_reach(x, y, king_x, king_y))
    }

    pub fn all_squares(&self) -> Vec<(usize, usize)> {
        let mut squares = Vec::<(usize, usize)>::new();
        for x in 0..self.tiles.len() {
            for y in 0..self.tiles[x].len() {
                squares.push((x, y));
            }
        }
        squares
    }
}



impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_string = String::new();
        for y in 0..8 {
            let mut row_string = String::new();
            for x in 0..8 {
                row_string += self.tiles[x][y].get_symbol_for_tile();
            }
            board_string.insert(0, '\n');
            board_string.insert_str(0, row_string.as_str());
        }
        write!(f, "{}", board_string)
    }
}

#[test]
fn pawn_move_one_step() {
    let board = Board::new();
    assert!(board.legal_move(4, 1, 4, 2));
}

#[test]
fn pawn_move_two_steps() {
    let board = Board::new();
    assert!(board.legal_move(4, 1, 4, 3));
}

#[test]
fn pawn_move_two_steps_not_start_square() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(4, 6, 3, 3);
    board = board.make_move(4, 1, 4, 5);
    assert!(!board.legal_move(4, 3, 4, 5));
}

#[test]
fn pawn_move_one_step_not_start_square() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 2);
    board = board.make_move(4, 6, 3, 5);
    assert!(board.legal_move(4, 2, 4, 3));
}

#[test]
fn pawn_capture() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(5, 6, 5, 4);
    assert!(board.legal_move(4, 3, 5, 4));
}

#[test]
fn pawn_cant_capture_empty_square() {
    let board = Board::new();
    assert!(!board.legal_move(4, 1, 5, 2));
}

#[test]
fn far_away_pawn_capture() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(6, 6, 6, 4);
    assert!(!board.legal_move(4, 3, 6, 4));
}

#[test]
fn blocked_pawn_movement() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(4, 6, 4, 4);
    assert!(!board.legal_move(4, 3, 4, 4));
}

#[test]
fn pawn_move_three_steps() {
    let board = Board::new();
    assert!(!board.legal_move(4, 1, 4, 4));
}

#[test]
fn pawn_move_black_two_steps() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    assert!(board.legal_move(1, 6, 1, 4));
}

#[test]
fn pawn_move_black_one_step() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    assert!(board.legal_move(1, 6, 1, 5));
}

#[test]
fn blocked_pawn_movement_and_capture_black() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(4, 6, 4, 4);
    board = board.make_move(5, 1, 5, 3);
    assert!(board.legal_move(4, 4, 5, 3));
    assert!(!board.legal_move(4, 4, 4, 3));
}

#[test]
fn move_no_piece() {
    let board = Board::new();
    assert!(!board.legal_move(2, 2, 3, 3));
}

#[test]
fn pawn_move_wrong_color() {
    let board = Board::new();
    assert!(!board.legal_move(4, 6, 4, 4));
}

#[test]
fn rook_can_move(){
    let mut board = Board::new();
    board = board.make_move(0, 1, 0, 3);
    board = board.make_move(0, 6, 0, 4);
    assert!(board.legal_move(0, 0, 0, 2));
}


#[test]
fn rook_is_obstructed(){
    let board = Board::new();
    assert!(!board.legal_move(0, 0, 0, 2));
    assert!(!board.legal_move(0, 0, 0, 1));

}

#[test]
fn rook_can_capture_file(){
    let mut board = Board::new();
    board = board.make_move(0, 1, 0, 3);
    board = board.make_move(1, 6, 1, 4);
    board = board.make_move(0, 3, 1, 4);
    board = board.make_move(2, 6, 2, 4);
    assert!(board.legal_move(0, 0, 0, 6));
    board = board.make_move(0, 0, 0, 6);
    assert!(board.legal_move(0, 7, 0, 6));
}

#[test]
fn rook_can_capture_rank(){
    let mut board = Board::new();
    //Move out pawns
    board = board.make_move(0, 1, 0, 3);
    board = board.make_move(0, 6, 0, 4);
    board = board.make_move(7, 1, 7, 3);
    board = board.make_move(7, 6, 7, 4);
    //Put rooks in middle of board to be ready to capture
    board = board.make_move(0, 0, 0, 2);
    board = board.make_move(0, 7, 0, 5);
    board = board.make_move(0, 2, 2, 2);
    board = board.make_move(0, 5, 1, 5);
    board = board.make_move(2, 2, 2, 4);
    board = board.make_move(1, 5, 1, 3);
    //Check if white can capture
    assert!(board.legal_move(2, 4, 0, 4));
    assert!(board.legal_move(2, 4, 7, 4));
    board = board.make_move(2, 4, 0, 4);
    //Likewise for black
    assert!(board.legal_move(1, 3, 0, 3));
    assert!(board.legal_move(1, 3, 7, 3));
}

#[test]
fn bishop_can_travel_each_direction(){
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(4, 6, 4, 4);
    board = board.make_move(5, 0, 2, 3);
    board = board.make_move(0, 6, 0, 4);
    assert!(board.legal_move(2, 3, 0, 5));
    assert!(board.legal_move(2, 3, 1, 4));
    assert!(board.legal_move(2, 3, 1, 2));
    assert!(board.legal_move(2, 3, 3, 4));
    assert!(board.legal_move(2, 3, 4, 5));
}

#[test]
fn bishop_is_blocked_each_direction(){
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(3, 6, 3, 4);
    board = board.make_move(5, 0, 2, 3);
    board = board.make_move(1, 6, 1, 4);
    assert!(!board.legal_move(2, 3, 0, 1));
    assert!(!board.legal_move(2, 3, 0, 5));
    assert!(!board.legal_move(2, 3, 4, 5));
    board = board.make_move(3, 1, 3, 2);
    board = board.make_move(1, 4, 1, 3);
    assert!(!board.legal_move(2, 3, 3, 2));
    assert!(!board.legal_move(2, 3, 4, 1));
}

#[test]
fn knight_movements(){
    let mut board = Board::new();
    board = board.make_move(6, 0, 5, 2);
    board = board.make_move(6, 7, 5, 5);
    board = board.make_move(5, 2, 6, 4);
    board = board.make_move(5, 5, 6, 3);
    assert!(board.legal_move(6, 4, 4, 3));
    assert!(board.legal_move(6, 4, 4, 5));
    assert!(board.legal_move(6, 4, 5, 2));
    assert!(board.legal_move(6, 4, 5, 6));
    assert!(board.legal_move(6, 4, 7, 2));
    assert!(board.legal_move(6, 4, 7, 6));
    board = board.make_move(6, 4, 5, 6);
    assert!(board.legal_move(6, 3, 4, 2));
    assert!(board.legal_move(6, 3, 4, 4));
    assert!(board.legal_move(6, 3, 5, 1));
    assert!(board.legal_move(6, 3, 5, 5));
    assert!(board.legal_move(6, 3, 7, 1));
    assert!(board.legal_move(6, 3, 7, 5));
}

#[test]
fn queen_movement(){
    let mut board = Board::new();
    assert!(!board.legal_move(3, 0, 5, 2));
    assert!(!board.legal_move(3, 0, 3, 2));
    assert!(!board.legal_move(3, 0, 4, 0));
    assert!(!board.legal_move(3, 0, 3, 0));
    assert!(!board.legal_move(3, 0, 2, 0));
    assert!(!board.legal_move(3, 0, 1, 0));
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(4, 6, 4, 4);
    board = board.make_move(3, 0, 5, 2);
    board = board.make_move(0, 6, 0, 4);
    assert!(board.legal_move(5, 2, 5, 6));
    assert!(board.legal_move(5, 2, 5, 5));
    assert!(board.legal_move(5, 2, 5, 3));
    assert!(board.legal_move(5, 2, 0, 2));
    assert!(board.legal_move(5, 2, 4, 2));
    assert!(board.legal_move(5, 2, 6, 2));
    assert!(board.legal_move(5, 2, 7, 2));
}

#[test]
fn king_movement(){
    let mut board = Board::new();
    assert!(!board.legal_move(4, 0, 4, 1));
    assert!(!board.legal_move(4, 0, 3, 0));
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(4, 6, 4, 4);
    assert!(board.legal_move(4, 0, 4, 1));
    board = board.make_move(4, 0, 4, 1);
    assert!(board.legal_move(4, 7, 4, 6));
    board = board.make_move(4, 7, 4, 6);
    assert!(board.legal_move(4, 1, 4, 2));
    assert!(board.legal_move(4, 1, 5, 2));
    assert!(board.legal_move(4, 1, 3, 2));
    board = board.make_move(4, 1, 5, 2);
    assert!(board.legal_move(4, 6, 5, 5));
    assert!(board.legal_move(4, 6, 4, 5));
    assert!(board.legal_move(4, 6, 3, 5));
    board = board.make_move(4, 6, 3, 5);
    board = board.make_move(5, 2, 6, 3);
    board = board.make_move(3, 7, 6, 4);
    assert!(board.legal_move(6, 3, 6, 4));
}


#[test]
fn simple_checks(){
    let mut board = Board::new();
    board = board.make_move(4, 1, 5, 2);
    board = board.make_move(4, 6, 4, 4);
    board = board.make_move(3, 0, 5, 2);
    board = board.make_move(0, 6, 0, 4);
    board = board.make_move(5, 2, 5, 6);
    assert!(board.legal_move(4, 7, 5, 6));
    assert!(!board.legal_move(6, 6, 6, 5));
}

#[test]
fn simple_pin(){
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 2);
    board = board.make_move(3, 6, 3, 4);
    assert!(board.legal_move(5, 0, 1, 4));
    board = board.make_move(5, 0, 1, 4);
    board = board.make_move(1, 7, 2, 5);
    board = board.make_move(0, 1, 0, 2);
    assert!(!board.legal_move(2, 5, 1, 7));
}

#[test]
fn pinned_piece_can_check(){
    let mut board = Board::new();
    board = board.make_move(4, 1, 5, 2);
    board = board.make_move(3, 6, 3, 5);
    board = board.make_move(5, 0, 1, 4);
    board = board.make_move(1, 7, 2, 5);
    board = board.make_move(4, 0, 4, 1);
    board = board.make_move(7, 6, 7, 5);
    board = board.make_move(4, 1, 3, 2);
    board = board.make_move(7, 5, 7, 4);
    assert!(!board.legal_move(3, 2, 4, 5));
}

#[test]
fn can_take_queen(){
    let mut board = Board::empty();
    board.tiles[1][0] = Tile{piece: Piece::King, color: Color::White};
    board.tiles[7][7] = Tile{piece: Piece::King, color: Color::Black};
    board.tiles[2][1] = Tile{piece: Piece::Queen, color: Color::Black};
    assert!(board.legal_move(1, 0, 2, 1));
    assert!(board.legal_move(1, 0, 0, 0));
    assert_eq!(2, board.legal_moves().len())
}

#[test]
fn scholars_mate(){
    let mut board = Board::new();
    board = board.make_move(4, 1, 5, 2);
    board = board.make_move(1, 7, 0, 5);
    board = board.make_move(3, 0, 5, 2);
    board = board.make_move(0, 5, 1, 7);
    board = board.make_move(5, 0, 2, 3);
    board = board.make_move(1, 7, 0, 5);
    assert!(board.legal_move(5, 2, 5, 6));
    assert!(!board.legal_moves().is_empty());
    assert_eq!(GameResult::Ongoing, board.result());
    board = board.make_move(5, 2, 5, 6);
    assert!(board.legal_moves().is_empty());
    assert_eq!(GameResult::WhiteWin, board.result());
}



#[test]
fn draw(){
    let mut board = Board::empty();
    board.tiles[0][0] = Tile{piece: Piece::King, color: Color::White};
    board.tiles[0][5] = Tile{piece: Piece::King, color: Color::Black};
    board.tiles[2][1] = Tile{piece: Piece::Queen, color: Color::Black};
    assert_eq!(GameResult::Draw, board.result());
}

