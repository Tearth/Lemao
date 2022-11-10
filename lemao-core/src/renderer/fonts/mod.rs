pub mod bff;
pub mod storage;

pub struct Font {
    pub id: usize,
    pub width: u32,
    pub height: u32,
    pub cell_width: u32,
    pub cell_height: u32,
    pub base_character_offset: u8,
    pub character_widths: Vec<u8>,
    pub data: Vec<u8>,
}

impl Font {
    pub fn new(width: u32, height: u32, cell_width: u32, cell_height: u32, base_character_offset: u8, character_widths: Vec<u8>, data: Vec<u8>) -> Self {
        Self { id: 0, width, height, cell_width, cell_height, base_character_offset, character_widths, data }
    }
}
