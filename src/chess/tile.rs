use super::color::Color;
use super::piece::Piece;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    color: Color,
    piece: Piece
}

impl Tile {
    pub fn new(color: Color, piece: Piece) -> Tile{
        Tile {color, piece}
    }
}
