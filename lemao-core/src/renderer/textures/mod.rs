pub mod bmp;
pub mod storage;

pub struct Texture {
    pub id: usize,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}
