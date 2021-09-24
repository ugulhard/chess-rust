#[derive(Debug, Clone)]
pub struct ChessMove {
    pub start_pos: (usize, usize),
    pub end_pos: (usize, usize),
}

impl ChessMove {
    pub fn from(chess_move_string: String) -> ChessMove {
        let tile_strings = chess_move_string.split(" ");
        let tile_strings_vec: Vec<String> = tile_strings.map(String::from).collect();
        let start_x_string = tile_strings_vec.get(0);
        let start_y_string = tile_strings_vec.get(1);
        let end_x_string = tile_strings_vec.get(2);
        let end_y_string = tile_strings_vec.get(3);
        let error_msg = "Incorrect format for input";
        println!("Vec: {:?}", tile_strings_vec);
        let start_x : usize = start_x_string.expect(error_msg).parse().expect(error_msg);
        let start_y : usize = start_y_string.expect(error_msg).parse().expect(error_msg);
        let end_x : usize = end_x_string.expect(error_msg).parse().expect(error_msg);
        let end_y : usize = end_y_string.expect(error_msg).parse().expect(error_msg);
        ChessMove {start_pos: (start_x, start_y), end_pos: (end_x, end_y)}
    }
}
