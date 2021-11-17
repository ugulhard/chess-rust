use super::{chess_move::ChessMove, color::Color, piece::Piece, tile::Tile, game_result::GameResult};
use std::{fmt};
extern crate itertools;

#[derive(Debug, Clone)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
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

    pub fn make_move(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> Board {
        let mut new_board =self.clone();
        new_board.player_to_move = Color::opposing_color(self.player_to_move);
        new_board.tiles[end_x][end_y] = self.tiles[start_x][start_y];
        new_board.tiles[start_x][start_y] = Tile::new(Color::Empty, Piece::Empty);
        new_board
    }

    pub fn legal_move(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        let color_before_move = self.tiles[start_x][start_y].color;
        if color_before_move != self.player_to_move {
            return false;
        }
        if self.piece_can_reach(start_x, start_y, end_x, end_y) {
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
            } else {
                if self.is_check(Color::Black){
                    return GameResult::WhiteWin
                }
                return GameResult::Draw
            }
        }
        return GameResult::Ongoing
    }

    pub fn legal_moves(&self) -> Vec<ChessMove> {
        self.possible_moves().into_iter()
        .filter(|chessMove| self.legal_move(chessMove.start_pos.0, chessMove.start_pos.1, chessMove.end_pos.0, chessMove.end_pos.1))
        .collect()
    }

    fn possible_moves(&self) -> Vec<ChessMove> {
        all_squares().into_iter().zip(all_squares())
        .filter(|((start_x, start_y),(end_x, end_y))| self.piece_can_reach(*start_x, *start_y, *end_x, *end_y) && self.tiles[*start_x][*start_y].color == self.player_to_move)
        .map(|((start_x, start_y),(end_x, end_y))| ChessMove{start_pos: (start_x, start_y), end_pos : (end_x, end_y)})
        .collect()
    }

    fn is_check(&self, color_to_check: Color) -> bool {
        let king_position: Option<(usize, usize)> = all_squares().into_iter().find(|(x ,y)| self.tiles[*x][*y].color == color_to_check && self.tiles[*x][*y].piece == Piece::King);
        let (king_x, king_y) = king_position.unwrap();
        all_squares().into_iter().any(|(x, y)| self.tiles[x][y].color != color_to_check && self.piece_can_reach(x, y, king_x, king_y))
    }

    fn piece_can_reach(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        if start_x == end_x && start_y == end_y {
            return false
        }
        let tile = self.tiles[start_x][start_y];
        match tile.piece {
            Piece::Pawn => self.can_pawn_reach(start_x, start_y, end_x, end_y),
            Piece::Rook => self.can_rook_reach(start_x, start_y, end_x, end_y),
            Piece::Knight => self.can_knight_reach(start_x, start_y, end_x, end_y),
            Piece::Bishop => self.can_bishop_reach(start_x, start_y, end_x, end_y),
            Piece::Queen => self.can_queen_reach(start_x, start_y, end_x, end_y),
            Piece::King => self.can_king_reach(start_x, start_y, end_x, end_y),
            _ => false
        }
    }

    fn can_pawn_reach(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        let color = self.tiles[start_x][start_y].color;
        let y_difference = end_y as i128 - start_y as i128;
        let x_difference = end_x as i128 - start_x as i128;

        match color {
            Color::White => {
                //Captures
                if x_difference.abs() == 1 
                && y_difference == 1 
                && Color::opposing_color(self.tiles[start_x][start_y].color) == self.tiles[end_x][end_y].color {
                    return true;
                } else if self.unobstructed_file(start_x, start_y, end_x, end_y) {
                    //Can only move two steps if on start square
                return (y_difference == 2 && start_y == 1) || y_difference == 1;
                } 
                false
            //En passant is fake news
            },
            Color::Black => {
                if x_difference.abs() == 1 
                && y_difference == -1 
                && Color::opposing_color(self.tiles[start_x][start_y].color) == self.tiles[end_x][end_y].color {
                    return true;
                } else if self.unobstructed_file(start_x, start_y, end_x, end_y) {
                return (y_difference == -2 && start_y == 6) || y_difference == -1;
                } 
                false
            },
            _ => false
        }
    }

    fn can_rook_reach(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        if start_x == end_x {
            let end_index = if start_y < end_y {end_y - 1} else {end_y + 1};
            return self.unobstructed_file(start_x, start_y, end_x, end_index) && self.tiles[start_x][start_y].color != self.tiles[end_x][end_y].color
        } else if start_y == end_y {
            let end_index = if start_x < end_x {end_x - 1} else {end_x + 1};
            return  self.unobstructed_rank(start_x, start_y, end_index, end_y) && self.tiles[start_x][start_y].color != self.tiles[end_x][end_y].color
        }
        false
    }

    fn can_bishop_reach(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        self.unobstructed_diagonal(start_x, start_y, end_x, end_y) && self.tiles[start_x][start_y].color != self.tiles[end_x][end_y].color
    }

    fn can_queen_reach(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        self.can_bishop_reach(start_x, start_y, end_x, end_y) || self.can_rook_reach(start_x, start_y, end_x, end_y)
    }

    fn can_king_reach(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        let y_difference = end_y as i128 - start_y as i128;
        let x_difference = end_x as i128 - start_x as i128;
        if x_difference.abs() > 1 || y_difference.abs() > 1 {
            return false;
        }
        self.tiles[start_x][start_y].color != self.tiles[end_x][end_y].color
    }

    fn can_knight_reach(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        let y_difference = end_y as i128 - start_y as i128;
        let x_difference = end_x as i128 - start_x as i128;
        self.fits_knight_pattern(x_difference, y_difference) && self.tiles[start_x][start_y].color != self.tiles[end_x][end_y].color
    }

    fn fits_knight_pattern(&self, x_difference: i128, y_difference: i128) -> bool {
        (x_difference.abs() == 2 && y_difference.abs() == 1) || (x_difference.abs() == 1 && y_difference.abs() == 2)
    }

    fn unobstructed_file(&self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> bool {
        if start_x != end_x {
            return false;
        }
        if start_y < end_y {
            ((start_y + 1)..(end_y + 1)).all(|idx| self.tiles[start_x][idx].piece == Piece::Empty)
        } else {
            (end_y..start_y).all(|idx| self.tiles[start_x][idx].piece == Piece::Empty)
        }
    }

    fn unobstructed_rank(&self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> bool {
        if start_y != end_y {
            return false;
        }
        if start_x < end_x {
            ((start_x + 1)..(end_x + 1)).all(|idx| self.tiles[idx][start_y].piece == Piece::Empty)
        } else {
            (end_x..start_x).all(|idx| self.tiles[idx][start_y].piece == Piece::Empty)
        }
    }

    fn unobstructed_diagonal(&self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> bool {
        let y_difference = end_y as i128 - start_y as i128;
        let x_difference = end_x as i128 - start_x as i128;
        if x_difference.abs() != y_difference.abs() || x_difference == 0 || y_difference == 0 {
            return false;
        }
        let x_order = x_difference / x_difference.abs();
        let y_order = y_difference / y_difference.abs();
        //Add the direction we're going in so we don't check if the start square is empty
        let mut x_idx = start_x as i128 + x_order;
        let mut y_idx = start_y as i128 + y_order;
        while x_idx != end_x as i128 && y_idx != end_y as i128 {
            if self.tiles[x_idx as usize][y_idx as usize].piece != Piece::Empty {
                return false;
            }
            x_idx += x_order;
            y_idx += y_order;
        }
        true
        
    }
}

fn all_squares() -> Vec<(usize, usize)> {
    let mut squares = Vec::<(usize, usize)>::new();
    for x in 0..8 {
        for y in 0..8 {
            squares.push((x, y));
        }
    }
    squares
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

mod tests {
    use super::*;
    #[test]
fn pawn_move_one_step() {
    let board = Board::new();
    assert_eq!(true, board.legal_move(4, 1, 4, 2));
}

#[test]
fn pawn_move_two_steps() {
    let board = Board::new();
    assert_eq!(true, board.legal_move(4, 1, 4, 3));
}

#[test]
fn pawn_move_two_steps_not_start_square() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(4, 6, 3, 3);
    board = board.make_move(4, 1, 4, 5);
    assert_eq!(false, board.legal_move(4, 3, 4, 5));
}

#[test]
fn pawn_move_one_step_not_start_square() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 2);
    board = board.make_move(4, 6, 3, 5);
    assert_eq!(true, board.legal_move(4, 2, 4, 3));
}

#[test]
fn pawn_capture() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(5, 6, 5, 4);
    assert_eq!(true, board.legal_move(4, 3, 5, 4));
}

#[test]
fn pawn_cant_capture_empty_square() {
    let board = Board::new();
    assert_eq!(false, board.legal_move(4, 1, 5, 2));
}

#[test]
fn far_away_pawn_capture() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(6, 6, 6, 4);
    assert_eq!(false, board.legal_move(4, 3, 6, 4));
}

#[test]
fn blocked_pawn_movement() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(4, 6, 4, 4);
    assert_eq!(false, board.legal_move(4, 3, 4, 4));
}

#[test]
fn pawn_move_three_steps() {
    let board = Board::new();
    assert_eq!(false, board.legal_move(4, 1, 4, 4));
}

#[test]
fn pawn_move_black_two_steps() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    assert_eq!(true, board.legal_move(1, 6, 1, 4));
}

#[test]
fn pawn_move_black_one_step() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    assert_eq!(true, board.legal_move(1, 6, 1, 5));
}

#[test]
fn blocked_pawn_movement_and_capture_black() {
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(4, 6, 4, 4);
    board = board.make_move(5, 1, 5, 3);
    assert_eq!(true, board.legal_move(4, 4, 5, 3));
    assert_eq!(false, board.legal_move(4, 4, 4, 3));
}

#[test]
fn move_no_piece() {
    let board = Board::new();
    assert_eq!(false, board.legal_move(2, 2, 3, 3));
}

#[test]
fn pawn_move_wrong_color() {
    let board = Board::new();
    assert_eq!(false, board.legal_move(4, 6, 4, 4));
}

#[test]
fn unobstructed_file_all_empty(){
    let board = Board::new();
    assert_eq!(true, board.unobstructed_file(4, 2, 4, 5));
}

#[test]
fn unobstructed_file_one_step(){
    let board = Board::new();
    assert_eq!(true, board.unobstructed_file(4, 4, 4, 5));
}

#[test]
fn unobstructed_file_start_with_piece(){
    let board = Board::new();
    assert_eq!(true, board.unobstructed_file(4, 1, 4, 5));
}

#[test]
fn obstructed_file(){
    let board = Board::new();
    assert_eq!(false, board.unobstructed_file(4, 1, 4, 6));
}

#[test]
fn obstructed_file_one_step(){
    let board = Board::new();
    assert_eq!(false, board.unobstructed_file(4, 5, 4, 6));
}

#[test]
fn unobstructed_file_start_with_piece_start_larger_than_end(){
    let board = Board::new();
    assert_eq!(true, board.unobstructed_file(4, 6, 4, 2));
}

#[test]
fn obstructed_file_start_with_piece_start_larger_than_end(){
    let board = Board::new();
    assert_eq!(false, board.unobstructed_file(4, 6, 4, 1));
}

#[test]
fn rook_can_move(){
    let mut board = Board::new();
    board = board.make_move(0, 1, 0, 3);
    board = board.make_move(0, 6, 0, 4);
    assert_eq!(true, board.legal_move(0, 0, 0, 2));
}


#[test]
fn rook_is_obstructed(){
    let board = Board::new();
    assert_eq!(false, board.legal_move(0, 0, 0, 2));
    assert_eq!(false, board.legal_move(0, 0, 0, 1));

}

#[test]
fn rook_can_capture_file(){
    let mut board = Board::new();
    board = board.make_move(0, 1, 0, 3);
    board = board.make_move(1, 6, 1, 4);
    board = board.make_move(0, 3, 1, 4);
    board = board.make_move(2, 6, 2, 4);
    assert_eq!(true, board.legal_move(0, 0, 0, 6));
    board = board.make_move(0, 0, 0, 6);
    assert_eq!(true, board.legal_move(0, 7, 0, 6));
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
    assert_eq!(true, board.legal_move(2, 4, 0, 4));
    assert_eq!(true, board.legal_move(2, 4, 7, 4));
    board = board.make_move(2, 4, 0, 4);
    //Likewise for black
    assert_eq!(true, board.legal_move(1, 3, 0, 3));
    assert_eq!(true, board.legal_move(1, 3, 7, 3));
}

#[test]
fn bishop_can_travel_each_direction(){
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(4, 6, 4, 4);
    board = board.make_move(5, 0, 2, 3);
    board = board.make_move(0, 6, 0, 4);
    assert_eq!(true, board.legal_move(2, 3, 0, 5));
    assert_eq!(true, board.legal_move(2, 3, 1, 4));
    assert_eq!(true, board.legal_move(2, 3, 1, 2));
    assert_eq!(true, board.legal_move(2, 3, 3, 4));
    assert_eq!(true, board.legal_move(2, 3, 4, 5));
}

#[test]
fn bishop_is_blocked_each_direction(){
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(3, 6, 3, 4);
    board = board.make_move(5, 0, 2, 3);
    board = board.make_move(1, 6, 1, 4);
    assert_eq!(false, board.legal_move(2, 3, 0, 1));
    assert_eq!(false, board.legal_move(2, 3, 0, 5));
    assert_eq!(false, board.legal_move(2, 3, 4, 5));
    board = board.make_move(3, 1, 3, 2);
    board = board.make_move(1, 4, 1, 3);
    assert_eq!(false, board.legal_move(2, 3, 3, 2));
    assert_eq!(false, board.legal_move(2, 3, 4, 1));
}

#[test]
fn knight_movements(){
    let mut board = Board::new();
    board = board.make_move(6, 0, 5, 2);
    board = board.make_move(6, 7, 5, 5);
    board = board.make_move(5, 2, 6, 4);
    board = board.make_move(5, 5, 6, 3);
    assert_eq!(true, board.legal_move(6, 4, 4, 3));
    assert_eq!(true, board.legal_move(6, 4, 4, 5));
    assert_eq!(true, board.legal_move(6, 4, 5, 2));
    assert_eq!(true, board.legal_move(6, 4, 5, 6));
    assert_eq!(true, board.legal_move(6, 4, 7, 2));
    assert_eq!(true, board.legal_move(6, 4, 7, 6));
    board = board.make_move(6, 4, 5, 6);
    assert_eq!(true, board.legal_move(6, 3, 4, 2));
    assert_eq!(true, board.legal_move(6, 3, 4, 4));
    assert_eq!(true, board.legal_move(6, 3, 5, 1));
    assert_eq!(true, board.legal_move(6, 3, 5, 5));
    assert_eq!(true, board.legal_move(6, 3, 7, 1));
    assert_eq!(true, board.legal_move(6, 3, 7, 5));
}

#[test]
fn queen_movement(){
    let mut board = Board::new();
    assert_eq!(false, board.legal_move(3, 0, 5, 2));
    assert_eq!(false, board.legal_move(3, 0, 3, 2));
    assert_eq!(false, board.legal_move(3, 0, 4, 0));
    assert_eq!(false, board.legal_move(3, 0, 3, 0));
    assert_eq!(false, board.legal_move(3, 0, 2, 0));
    assert_eq!(false, board.legal_move(3, 0, 1, 0));
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(4, 6, 4, 4);
    board = board.make_move(3, 0, 5, 2);
    board = board.make_move(0, 6, 0, 4);
    assert_eq!(true, board.legal_move(5, 2, 5, 6));
    assert_eq!(true, board.legal_move(5, 2, 5, 5));
    assert_eq!(true, board.legal_move(5, 2, 5, 3));
    assert_eq!(true, board.legal_move(5, 2, 0, 2));
    assert_eq!(true, board.legal_move(5, 2, 4, 2));
    assert_eq!(true, board.legal_move(5, 2, 6, 2));
    assert_eq!(true, board.legal_move(5, 2, 7, 2));
}

#[test]
fn king_movement(){
    let mut board = Board::new();
    assert_eq!(false, board.legal_move(4, 0, 4, 1));
    assert_eq!(false, board.legal_move(4, 0, 3, 0));
    board = board.make_move(4, 1, 4, 3);
    board = board.make_move(4, 6, 4, 4);
    assert_eq!(true, board.legal_move(4, 0, 4, 1));
    board = board.make_move(4, 0, 4, 1);
    assert_eq!(true, board.legal_move(4, 7, 4, 6));
    board = board.make_move(4, 7, 4, 6);
    assert_eq!(true, board.legal_move(4, 1, 4, 2));
    assert_eq!(true, board.legal_move(4, 1, 5, 2));
    assert_eq!(true, board.legal_move(4, 1, 3, 2));
    board = board.make_move(4, 1, 5, 2);
    assert_eq!(true, board.legal_move(4, 6, 5, 5));
    assert_eq!(true, board.legal_move(4, 6, 4, 5));
    assert_eq!(true, board.legal_move(4, 6, 3, 5));
    board = board.make_move(4, 6, 3, 5);
    board = board.make_move(5, 2, 6, 3);
    board = board.make_move(3, 7, 6, 4);
    assert_eq!(true, board.legal_move(6, 3, 6, 4));
}


#[test]
fn simple_checks(){
    let mut board = Board::new();
    board = board.make_move(4, 1, 5, 2);
    board = board.make_move(4, 6, 4, 4);
    board = board.make_move(3, 0, 5, 2);
    board = board.make_move(0, 6, 0, 4);
    board = board.make_move(5, 2, 5, 6);
    assert_eq!(true, board.legal_move(4, 7, 5, 6));
    assert_eq!(false, board.legal_move(6, 6, 6, 5));
}

#[test]
fn simple_pin(){
    let mut board = Board::new();
    board = board.make_move(4, 1, 4, 2);
    board = board.make_move(3, 6, 3, 4);
    assert_eq!(true, board.legal_move(5, 0, 1, 4));
    board = board.make_move(5, 0, 1, 4);
    board = board.make_move(1, 7, 2, 5);
    board = board.make_move(0, 1, 0, 2);
    assert_eq!(false, board.legal_move(2, 5, 1, 7));
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
    assert_eq!(false, board.legal_move(3, 2, 4, 5));
}

}

