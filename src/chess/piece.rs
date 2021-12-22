#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Empty
}

const BOARD_SIZE: usize = 8;

impl Piece {
    pub fn squares_that_fit_move_pattern(&self, start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
        match self {
            Piece::Pawn => Piece::pawn_pattern(start_x, start_y),
            Piece::Rook => Piece::rook_pattern(start_x, start_y),
            Piece::Knight => Piece::knight_pattern(start_x, start_y),
            Piece::Bishop => Piece::bishop_pattern(start_x, start_y),
            Piece::Queen => Piece::queen_pattern(start_x, start_y),
            Piece::King => Piece::king_pattern(start_x, start_y),
            _ => vec!()
        }
        
    }

    fn pawn_pattern(start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
        let mut white_pawn_pattern: Vec::<(i32, i32)> = vec!((0, 1), (0,2), (1,1), (-1, 1)).into_iter()
        .map(|(x, y)| (start_x as i32 + x, start_y as i32 + y))
        .collect();
        let mut black_pawn_pattern: Vec::<(i32, i32)> = vec!((0, -1), (0, -2), (1, -1), (-1, -1)).into_iter()
        .map(|(x, y)| (start_x as i32 + x, start_y as i32 + y))
        .collect();
        black_pawn_pattern.append(&mut white_pawn_pattern);
        black_pawn_pattern.into_iter()
        .filter(|(end_x, end_y)| Piece::inside_board(end_x) && Piece::inside_board(end_y))
        .map(i32_tuple_to_usize)
        .collect()
    }

    fn rook_pattern(start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
        let mut x_pattern: Vec::<(usize, usize)> = (0..BOARD_SIZE)
        .map(|x| (x, start_y))
        .filter(|(end_x, _end_y)| start_x != *end_x)
        .collect();
        let mut y_pattern: Vec::<(usize, usize)> = (0..BOARD_SIZE)
        .map(|y| (start_x, y))
        .filter(|(_end_x, end_y)| start_y != *end_y)
        .collect();
        x_pattern.append(&mut y_pattern);
        x_pattern
    }

    fn knight_pattern(start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
        let knight_pattern : Vec::<(i32, i32)> = vec!((1, 2), (2, 1),
            (-1, 2), (-2, 1),
            (-1, -2), (-2, -1),
            (1, -2), (2, -1));
        knight_pattern.into_iter()
        .map(|(x_diff, y_diff)| (start_x as i32 + x_diff, start_y as i32 + y_diff))
        .filter(|(end_x, end_y)| Piece::inside_board(end_x) && Piece::inside_board(end_y))
        .map(i32_tuple_to_usize)
        .collect()        
    }

    fn bishop_pattern(start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
        let start_x_i32 = start_x as i32;
        let start_y_i32 = start_y as i32;
        //Magic +1 to handle zero indexation
        let up_left_x_min_value = i32::max(0, start_x_i32 + start_y_i32 - BOARD_SIZE as i32 + 1) as usize;
        let up_left_y_max_value = i32::min(BOARD_SIZE as i32 - 1, start_x_i32 + start_y_i32) as usize;
        //While x increases, y decreases -> reverse order
        let mut up_left: Vec::<(usize, usize)> = (up_left_x_min_value..start_x).into_iter()
        .zip(((start_y + 1)..(up_left_y_max_value + 1)).rev().into_iter())
        .collect();

        let up_right_x_max_value = i32::min(BOARD_SIZE as i32 - 1, BOARD_SIZE as i32 - 1 - start_y_i32 + start_x_i32) as usize;
        let up_right_y_max_value = i32::min(BOARD_SIZE as i32 - 1, BOARD_SIZE as i32 - 1 - start_x_i32 + start_y_i32) as usize;
        //From smaller to larger for both indices woth since both are increasing
        let mut up_right: Vec::<(usize, usize)> = ((start_x + 1)..(up_right_x_max_value + 1)).into_iter()
        .zip(((start_y + 1)..(up_right_y_max_value + 1)).into_iter())
        .collect();

        let down_left_x_min_value = i32::max(0, start_x_i32 - start_y_i32) as usize;
        let down_left_y_min_value =  i32::max(0, start_y_i32 - start_x_i32) as usize;
        //From smaller to larger for both indices woth since both are decreasing
        let mut down_left: Vec::<(usize, usize)> = (down_left_x_min_value..start_x).into_iter()
        .zip((down_left_y_min_value..start_y).into_iter())
        .collect();

        let down_right_x_max_value = i32::min(BOARD_SIZE as i32 - 1, start_y_i32 + start_x_i32) as usize;
        let down_right_y_min_value = i32::max(0, start_y_i32 + start_x_i32 - BOARD_SIZE as i32 + 1) as usize;
        //While x decreases, y increases -> reverse order
        let mut down_right: Vec::<(usize, usize)> = ((start_x + 1)..(down_right_x_max_value + 1)).into_iter()
        .zip((down_right_y_min_value..start_y).rev().into_iter())
        .collect();

        let mut pattern: Vec::<(usize, usize)> = Vec::new();
        pattern.append(&mut &mut up_left);
        pattern.append(&mut &mut up_right);
        pattern.append(&mut &mut down_left);
        pattern.append(&mut &mut down_right);


        pattern
    }

    fn queen_pattern(start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
        let mut queen_pattern = Piece::rook_pattern(start_x, start_y);
        queen_pattern.append(&mut Piece::bishop_pattern(start_x, start_y));
        queen_pattern
    }

    fn king_pattern(start_x: usize, start_y: usize) -> Vec<(usize, usize)> {
        vec!((1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1), (0,-1), (1,-1))
        .into_iter()
        .map(|(x_diff, y_diff)| (start_x as i32 + x_diff, start_y as i32 + y_diff))
        .filter(|(end_x, end_y)| Piece::inside_board(end_x) && Piece::inside_board(end_y))
        .map(i32_tuple_to_usize)
        .collect() 
    }

    fn inside_board(coordinate: &i32) -> bool {
        coordinate < &(BOARD_SIZE as i32) && coordinate >= &0
    }
}

fn i32_tuple_to_usize(indices: (i32, i32)) -> (usize, usize) {
    (indices.0 as usize, indices.1 as usize)
}

#[test]
fn test_inside_board_too_small() {
    assert!(!Piece::inside_board(&-1));
    assert!(!Piece::inside_board(&-5));

}

#[test]
fn test_inside_board_too_large() {
    assert!(!Piece::inside_board(&8));
    assert!(!Piece::inside_board(&15));
}

#[test]
fn test_inside_board() {
    assert!(Piece::inside_board(&0));
    assert!(Piece::inside_board(&7));
    assert!(Piece::inside_board(&3));
}


#[test]
fn test_pawn_pattern() {
    let mut expected_pattern = vec!((0,0), (1,0), (2,0), (1,2), (2,2), (0,2), (1,3));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::pawn_pattern(1,1);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_pawn_pattern_leftmost() {
    let mut expected_pattern = vec!((0,0), (1,0), (1,2), (0,2), (0,3));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::pawn_pattern(0,1);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_pawn_pattern_rightmost() {
    let mut expected_pattern = vec!((7,7), (6,7), (6,5), (7,5), (7,4));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::pawn_pattern(7,6);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_rook_pattern_bottom_left_corner() {
    let mut expected_pattern = vec!((1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0),
        (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::rook_pattern(0,0);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_rook_pattern_top_right_corner() {
    let mut expected_pattern = vec!((1,7), (2,7), (3,7), (4,7), (5,7), (6,7), (0,7),
        (7,1), (7,2), (7,3), (7,4), (7,5), (7,6), (7,0));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::rook_pattern(7,7);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_rook_pattern_middle() {
    let mut expected_pattern = vec!((0,5), (1,5), (2,5), (3,5), (5,5), (6,5), (7,5),
        (4,0), (4,1), (4,2), (4,3), (4,4), (4,6), (4,7));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::rook_pattern(4,5);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_knight_pattern_middle() {
    let mut expected_pattern = vec!((2,4), (3,3), (5,3), (6,4), (2,6), (3,7), (5,7),
        (6,6));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::knight_pattern(4,5);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_knight_pattern_top_left() {
    let mut expected_pattern = vec!((1,5), (2,6));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::knight_pattern(0,7);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_knight_pattern_bottom_right() {
    let mut expected_pattern = vec!((6,2), (5,1));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::knight_pattern(7,0);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_bishop_pattern_middle() {
    let mut expected_pattern = vec!((3, 4), (2, 3), (1, 2), (0,1), 
    (3, 6), (2, 7), 
    (5,6), (6, 7),
    (5,4), (6,3), (7,2));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::bishop_pattern(4,5);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_bishop_pattern_bottom_left() {
    let mut expected_pattern = vec!((1,1), (2,2), (3,3), (4,4), (5,5), (6,6), (7,7));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::bishop_pattern(0,0);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_bishop_pattern_top_right() {
    let mut expected_pattern = vec!((0,0), (1,1), (2,2), (3,3), (4,4), (5,5), (6,6));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::bishop_pattern(7,7);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_bishop_pattern_bottom_right() {
    let mut expected_pattern = vec!((6,1), (5,2), (4,3), (3,4), (2,5), (1,6), (0,7));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::bishop_pattern(7,0);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_bishop_pattern_top_left() {
    let mut expected_pattern = vec!((7,0), (6,1), (5,2), (4,3), (3,4), (2,5), (1,6));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::bishop_pattern(0,7);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_queen_pattern_middle() {
    let mut expected_pattern = vec!((3, 4), (2, 3), (1, 2), (0,1), 
    (3, 6), (2, 7), 
    (5,6), (6, 7),
    (5,4), (6,3), (7,2),
    (0,5), (1,5), (2,5), (3,5), (5,5), (6,5), (7,5),
    (4,0), (4,1), (4,2), (4,3), (4,4), (4,6), (4,7) );
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::queen_pattern(4,5);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_queen_pattern_bottom_left() {
    let mut expected_pattern = vec!((1,1), (2,2), (3,3), (4,4), (5,5), (6,6), (7,7),
    (1,0), (2,0), (3,0), (4,0), (5,0), (6,0), (7,0),
    (0,1), (0,2), (0,3), (0,4), (0,5), (0,6), (0,7));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::queen_pattern(0,0);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_queen_pattern_top_right() {
    let mut expected_pattern = vec!((0,0), (1,1), (2,2), (3,3), (4,4), (5,5), (6,6),
    (1,7), (2,7), (3,7), (4,7), (5,7), (6,7), (0,7),
    (7,1), (7,2), (7,3), (7,4), (7,5), (7,6), (7,0));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::queen_pattern(7,7);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}

#[test]
fn test_king_pattern_top_right() {
    let mut expected_pattern = vec!((6,7), (6,6), (7,6));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::king_pattern(7,7);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}
#[test]
fn test_king_pattern_top_left() {
    let mut expected_pattern = vec!((1,7), (1,6), (0,6));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::king_pattern(0,7);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}
#[test]
fn test_king_pattern_bottom_left() {
    let mut expected_pattern = vec!((1,0), (0,1), (1,1));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::king_pattern(0,0);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}
#[test]
fn test_king_pattern_bottom_right() {
    let mut expected_pattern = vec!((6,0), (6,1), (7,1));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::king_pattern(7,0);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}
#[test]
fn test_king_pattern_middle() {
    let mut expected_pattern = vec!((3,4), (4,4), (4,3), (4,2), (3,2), (2,2), (2,3), (2,4));
    expected_pattern.sort_unstable();
    let mut actual_pattern = Piece::king_pattern(3,3);
    actual_pattern.sort_unstable();
    assert_eq!(expected_pattern, actual_pattern);
}