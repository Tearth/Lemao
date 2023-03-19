#[derive(Copy, Clone, PartialEq)]
pub struct Coordinates {
    pub row: u8,
    pub col: u8,
}

impl Coordinates {
    pub fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }
}
