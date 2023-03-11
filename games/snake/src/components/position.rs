pub struct PositionComponent {
    pub row: u8,
    pub col: u8,
}

impl PositionComponent {
    pub fn new(row: u8, col: u8) -> Self {
        Self { row, col }
    }
}
