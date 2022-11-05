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
