use super::{color::Color, piece::Piece, tile::Tile};
use std::fmt;

#[derive(Debug, Clone)]
pub struct Board {
    tiles: Vec<Vec<Tile>>,
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
        Board {tiles}
    }

    pub fn make_move(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> Board {
        let mut new_board =self.clone();
        new_board.tiles[end_x][end_y] = self.tiles[start_x][start_y];
        new_board.tiles[start_x][start_y] = Tile::new(Color::Empty, Piece::Empty);
        new_board
    }

}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut boardString = String::new();
        for y in 0..8 {
            let mut row_string = String::new();
            for x in 0..8 {
                row_string += self.tiles[x][y].get_symbol_for_tile();
            }
            boardString.insert_str(0, "\n");
            boardString.insert_str(0, row_string.as_str());
        }
        write!(f, "{}", boardString)
    }
}