#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coordinates {
    pub row: u8,
    pub col: u8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Coordinates {
    pub fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }
}
