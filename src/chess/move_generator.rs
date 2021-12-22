use super::{board::Board, chess_move::ChessMove, color::Color, piece::Piece};



pub struct MoveGenerator<'a> {
    board: &'a Board
}

impl MoveGenerator<'_> {

    pub fn new(board: &Board) -> MoveGenerator{
        MoveGenerator {board}
    }

    pub fn possible_moves(&self) -> Vec<ChessMove> {
        let mut possible_moves = Vec::<ChessMove>::new();
        for (start_x, start_y) in self.board.all_squares() {
            let piece = self.board.tiles[start_x][start_y].piece;
            for (end_x, end_y) in piece.squares_that_fit_move_pattern(start_x, start_y) {
                if self.piece_can_reach(start_x, start_y, end_x, end_y) {
                    possible_moves.push(ChessMove{start_pos: (start_x, start_y), end_pos : (end_x, end_y)});
                }
            }
        }
        possible_moves
    }
    pub fn piece_can_reach(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        if start_x == end_x && start_y == end_y {
            return false
        }
        let tile = self.board.tiles[start_x][start_y];
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
        let color = self.board.tiles[start_x][start_y].color;
        let y_difference = end_y as i128 - start_y as i128;
        let x_difference = end_x as i128 - start_x as i128;

        match color {
            Color::White => {
                //Captures
                if x_difference.abs() == 1 
                && y_difference == 1 
                && Color::opposing_color(self.board.tiles[start_x][start_y].color) == self.board.tiles[end_x][end_y].color {
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
                && Color::opposing_color(self.board.tiles[start_x][start_y].color) == self.board.tiles[end_x][end_y].color {
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
            return self.unobstructed_file(start_x, start_y, end_x, end_index) && self.board.tiles[start_x][start_y].color != self.board.tiles[end_x][end_y].color
        } else if start_y == end_y {
            let end_index = if start_x < end_x {end_x - 1} else {end_x + 1};
            return  self.unobstructed_rank(start_x, start_y, end_index, end_y) && self.board.tiles[start_x][start_y].color != self.board.tiles[end_x][end_y].color
        }
        false
    }

    fn can_bishop_reach(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        self.unobstructed_diagonal(start_x, start_y, end_x, end_y) && self.board.tiles[start_x][start_y].color != self.board.tiles[end_x][end_y].color
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
        self.board.tiles[start_x][start_y].color != self.board.tiles[end_x][end_y].color
    }

    fn can_knight_reach(&self, start_x: usize,start_y: usize, end_x: usize, end_y: usize) -> bool {
        let y_difference = end_y as i128 - start_y as i128;
        let x_difference = end_x as i128 - start_x as i128;
        self.fits_knight_pattern(x_difference, y_difference) && self.board.tiles[start_x][start_y].color != self.board.tiles[end_x][end_y].color
    }

    fn fits_knight_pattern(&self, x_difference: i128, y_difference: i128) -> bool {
        (x_difference.abs() == 2 && y_difference.abs() == 1) || (x_difference.abs() == 1 && y_difference.abs() == 2)
    }

    fn unobstructed_file(&self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> bool {
        if start_x != end_x {
            return false;
        }
        if start_y < end_y {
            ((start_y + 1)..(end_y + 1)).all(|idx| self.board.tiles[start_x][idx].piece == Piece::Empty)
        } else {
            (end_y..start_y).all(|idx| self.board.tiles[start_x][idx].piece == Piece::Empty)
        }
    }

    fn unobstructed_rank(&self, start_x: usize, start_y: usize, end_x: usize, end_y: usize) -> bool {
        if start_y != end_y {
            return false;
        }
        if start_x < end_x {
            ((start_x + 1)..(end_x + 1)).all(|idx| self.board.tiles[idx][start_y].piece == Piece::Empty)
        } else {
            (end_x..start_x).all(|idx| self.board.tiles[idx][start_y].piece == Piece::Empty)
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
            if self.board.tiles[x_idx as usize][y_idx as usize].piece != Piece::Empty {
                return false;
            }
            x_idx += x_order;
            y_idx += y_order;
        }
        true
    }
}


#[test]
fn unobstructed_file_all_empty(){
    let board = Board::new();
    let move_generator = MoveGenerator::new(&board);
    assert!(move_generator.unobstructed_file(4, 2, 4, 5));
}

#[test]
fn unobstructed_file_one_step(){
    let board = Board::new();
    let move_generator = MoveGenerator::new(&board);
    assert!(move_generator.unobstructed_file(4, 4, 4, 5));
}

#[test]
fn unobstructed_file_start_with_piece(){
    let board = Board::new();
    let move_generator = MoveGenerator::new(&board);
    assert!(move_generator.unobstructed_file(4, 1, 4, 5));
}

#[test]
fn obstructed_file(){
    let board = Board::new();
    let move_generator = MoveGenerator::new(&board);
    assert!(!move_generator.unobstructed_file(4, 1, 4, 6));
}

#[test]
fn obstructed_file_one_step(){
    let board = Board::new();
    let move_generator = MoveGenerator::new(&board);
    assert!(!move_generator.unobstructed_file(4, 5, 4, 6));
}

#[test]
fn unobstructed_file_start_with_piece_start_larger_than_end(){
    let board = Board::new();
    let move_generator = MoveGenerator::new(&board);
    assert!(move_generator.unobstructed_file(4, 6, 4, 2));
}

#[test]
fn obstructed_file_start_with_piece_start_larger_than_end(){
    let board = Board::new();
    let move_generator = MoveGenerator::new(&board);
    assert!(!move_generator.unobstructed_file(4, 6, 4, 1));
}
