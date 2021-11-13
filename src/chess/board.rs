use super::{color::Color, piece::Piece, tile::Tile};
use std::fmt;

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
        Board {tiles: tiles, player_to_move: Color::White}
    }

    pub fn make_move(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> Board {
        let mut new_board =self.clone();
        new_board.player_to_move = Color::opposing_color(self.player_to_move);
        new_board.tiles[end_x][end_y] = self.tiles[start_x][start_y];
        new_board.tiles[start_x][start_y] = Tile::new(Color::Empty, Piece::Empty);
        new_board
    }

    pub fn legal_move(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        let color = self.tiles[start_x][start_y].color;
        return color == self.player_to_move
        && self.piece_can_reach(start_x, start_y, end_x, end_y)
    }

    fn piece_can_reach(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        let tile = self.tiles[start_x][start_y];
        match tile.piece {
            Piece::Pawn => self.can_pawn_reach(start_x, start_y, end_x, end_y),
            Piece::Rook => false,
            Piece::Knight => false,
            Piece::Bishop => false,
            Piece::Queen => false,
            Piece::King => false,
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
                return false;
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
                return false;
            },
            _ => false
        }
    }

    fn unobstructed_file(&self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> bool {
        if start_x != end_x {
            return false;
        }
        if start_y < end_y {
            return ((start_y + 1)..(end_y + 1)).all(|idx| self.tiles[start_x][idx].piece == Piece::Empty);
        } else {
            return (end_y..start_y).all(|idx| self.tiles[start_x][idx].piece == Piece::Empty);
        }
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
            board_string.insert_str(0, "\n");
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

}

