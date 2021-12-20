#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChessMove {
    pub start_pos: (usize, usize),
    pub end_pos: (usize, usize),
}

impl ChessMove {
    pub fn from(chess_move_string: String) -> Option<ChessMove> {
        let index_strings = chess_move_string.split(' ');
        if chess_move_string.split(' ').count() != 4 {
            return None;
        }
        let mut indices = Vec::new();
        for index_string in index_strings {
            let index = parse_chess_move(index_string)?;
            if !valid_index(index) {
                return None;
            } else {
                indices.push(index);
            }
        }
        let start_x = indices.get(0)?;
        let start_y = indices.get(1)?;
        let end_x = indices.get(2)?;
        let end_y = indices.get(3)?;
        Some(ChessMove {start_pos: (*start_x, *start_y), end_pos: (*end_x, *end_y)})
    }

    pub fn get_start_x(&self) -> usize {
        return self.start_pos.0;
    }
    pub fn get_start_y(&self) -> usize {
        return self.start_pos.1;
    }
    pub fn get_end_x(&self) -> usize {
        return self.end_pos.0;
    }
    pub fn get_end_y(&self) -> usize {
        return self.end_pos.1;
    }
}

fn parse_chess_move(move_string: &str) -> Option<usize> {
    let move_usize = move_string.parse::<usize>();
    match move_usize {
        Ok(move_usize) => Some(move_usize),
        _ => None
    }
}

fn valid_index(index: usize) -> bool {
    index <= 7
}

#[test]
fn normal_move(){
    let move_string = String::from("0 1 0 3");
    let chess_move = ChessMove::from(move_string).unwrap();
    assert_eq!(chess_move.start_pos.0, 0);
    assert_eq!(chess_move.start_pos.1, 1);
    assert_eq!(chess_move.end_pos.0, 0);
    assert_eq!(chess_move.end_pos.1, 3);
}

#[test]
#[should_panic]
fn too_many_entries(){
    let move_string = String::from("0 1 0 3 4");
    ChessMove::from(move_string).unwrap();
}

#[test]
#[should_panic]
fn too_few_indices(){
    let move_string = String::from("0 1 0");
    ChessMove::from(move_string).unwrap();
}

#[test]
#[should_panic]
fn too_large_index(){
    let move_string = String::from("0 1 0 8");
    ChessMove::from(move_string).unwrap();
}

#[test]
#[should_panic]
fn too_small_index(){
    let move_string = String::from("0 -1 0 6");
    ChessMove::from(move_string).unwrap();
}

#[test]
#[should_panic]
fn empty_string(){
    let move_string = String::from("");
    ChessMove::from(move_string).unwrap();
}