#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Black, 
    White,
    Empty
}

impl Color {
    pub fn opposing_color(color: Color) -> Color{
        match color {
            Color::Black => Color::White,
            Color::White => Color::Black,
            Color::Empty => panic!()
        }
    }
}
