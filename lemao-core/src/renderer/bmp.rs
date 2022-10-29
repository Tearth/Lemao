use std::fs::File;
use std::io::Read;

pub struct Texture {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub fn load(path: &str) -> Result<Texture, String> {
    let mut file = File::open(path).unwrap();
    let mut content = Vec::new();

    file.read_to_end(&mut content).unwrap();
    if content[0] != b'B' && content[1] != b'M' {
        panic!()
    }

    let size = read_u32(&content, 2);
    if content.len() != size as usize {
        panic!()
    }

    let data_address = read_u32(&content, 10);
    let header_size = read_u32(&content, 14);
    let width = read_u32(&content, 18);
    let height = read_u32(&content, 22);
    let bits_per_pixel = read_u16(&content, 28);
    let compression_method = read_u32(&content, 30);
    if compression_method != 3 {
        panic!()
    }

    let r_mask = read_u32(&content, 54);
    let g_mask = read_u32(&content, 54 + 4);
    let b_mask = read_u32(&content, 54 + 8);
    let a_mask = read_u32(&content, 54 + 12);

    let r_offset = r_mask.trailing_zeros();
    let g_offset = g_mask.trailing_zeros();
    let b_offset = b_mask.trailing_zeros();
    let a_offset = a_mask.trailing_zeros();

    let mut data = Vec::new();
    for i in 0..(width * height) {
        let pixel = read_u32(&content, (data_address + i * 4) as usize);
        let r = (pixel & r_mask) >> r_offset;
        let g = (pixel & g_mask) >> g_offset;
        let b = (pixel & b_mask) >> b_offset;
        let a = (pixel & a_mask) >> a_offset;

        data.push(r as u8);
        data.push(g as u8);
        data.push(b as u8);
        data.push(a as u8);
    }

    Ok(Texture { width, height, data })
}

fn read_u8(content: &Vec<u8>, index: usize) -> u8 {
    content[index]
}

fn read_u16(content: &Vec<u8>, index: usize) -> u16 {
    ((read_u8(content, index + 1) as u16) << 8) | (read_u8(content, index) as u16)
}

fn read_u32(content: &Vec<u8>, index: usize) -> u32 {
    ((read_u16(content, index + 2) as u32) << 16) | (read_u16(content, index) as u32)
}

fn read_u64(content: &Vec<u8>, index: usize) -> u64 {
    ((read_u32(content, index + 4) as u64) << 32) | (read_u32(content, index) as u64)
}
