use super::{color::Color, piece::Piece, tile::Tile};

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
        for x in 0..9 {
            tiles.push(Vec::<Tile>::new());
            for y in 0..9 {
                tiles[x][y] = get_piece_for_starting_tile(x, y);
            }
        }
        Board {tiles}
    }

}