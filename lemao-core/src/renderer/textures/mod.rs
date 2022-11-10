pub mod bmp;
pub mod storage;

pub struct Texture {
    pub id: usize,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
        Self { id: 0, width, height, data }
    }
}
