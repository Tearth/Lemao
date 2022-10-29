pub mod bmp;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub format: TextureFormat,
    pub data: Vec<u8>,
}

#[derive(Eq, PartialEq)]
pub enum TextureFormat {
    RGB,
    RGBA,
}
