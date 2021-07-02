use super::color::Color;
use super::piece::Piece;

pub struct Tile {
    color: Color,
    piece: Piece,
    coordinate: (u32, u32)
}
