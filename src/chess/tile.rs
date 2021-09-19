use super::color::Color;
use super::piece::Piece;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    color: Color,
    piece: Piece
}

fn get_black_piece_emoji(piece: Piece) -> &'static str {
    match piece {
        Piece::Bishop => "♝",
        Piece::King => "♚",
        Piece::Knight => "♞",
        Piece::Pawn => "♟",
        Piece::Queen => "♛",
        Piece::Rook => "♜",
        _ => "x"
    }
}

fn get_white_piece_emoji(piece: Piece) -> &'static str {
    match piece {
        Piece::Bishop => "♗",
        Piece::King => "♔",
        Piece::Knight => "♘",
        Piece::Pawn => "♙",
        Piece::Queen => "♕",
        Piece::Rook => "♖",
        _ => "x"
    }
}

impl Tile {
    pub fn new(color: Color, piece: Piece) -> Tile{
        Tile {color, piece}
    }
    pub fn get_symbol_for_tile(&self) -> &'static str {
        match self.color {
            Color::Empty => "x",
            Color::Black => get_black_piece_emoji(self.piece),
            Color::White => get_white_piece_emoji(self.piece),
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_symbol_for_tile())
    }
}
