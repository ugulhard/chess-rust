use crate::chess::tile::Tile;
use crate::chess::piece::Piece;
use crate::chess::color::Color;
use crate::chess::board::Board;

pub fn evaluate(board: &Board) -> i32 {
    let mut score = 0;
    for column in &board.tiles {
        for tile in column {
            score += evaluate_piece_value(tile);
        }
    }
    score
}

fn evaluate_piece_value(tile: &Tile) -> i32{
    let value = match tile.piece {
        Piece::Pawn => 1,
        Piece::Rook => 5,
        Piece::Bishop => 3,
        Piece::Knight => 3,
        Piece::Queen => 9,
        _ => 0
    };
    if tile.color == Color::Black {
        return -value;
    }
    value
}


#[test]
fn white_advantage(){
    let mut board = Board::empty();
    board.tiles[0][0] = Tile{piece: Piece::King, color: Color::White};
    board.tiles[0][5] = Tile{piece: Piece::King, color: Color::Black};
    board.tiles[0][7] = Tile{piece: Piece::Queen, color: Color::White};
    assert!(evaluate(&board) > 0);
}

#[test]
fn black_advantage(){
    let mut board = Board::empty();
    board.tiles[0][0] = Tile{piece: Piece::King, color: Color::White};
    board.tiles[0][5] = Tile{piece: Piece::King, color: Color::Black};
    board.tiles[0][7] = Tile{piece: Piece::Queen, color: Color::Black};
    assert!(evaluate(&board) < 0);
}


#[test]
fn dead_draw(){
    let mut board = Board::empty();
    board.tiles[0][0] = Tile{piece: Piece::King, color: Color::White};
    board.tiles[0][5] = Tile{piece: Piece::King, color: Color::Black};
    assert_eq!(0, evaluate(&board));
}

