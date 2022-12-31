use super::*;
use crate::utils::binary;
use std::fs::File;
use std::io::Read;

pub fn load(path: &str) -> Result<RawTexture, String> {
    //////////////////////////////////////////////////////////////////////
    // BMP specification: https://en.wikipedia.org/wiki/BMP_file_format //
    //////////////////////////////////////////////////////////////////////

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(format!("File {} not found", path)),
    };

    let mut bmp = Vec::new();
    if let Err(message) = file.read_to_end(&mut bmp) {
        return Err(format!("Error while reading file: {}", message));
    }

    if binary::read_le_u16(&bmp, 0) != 0x4d42 {
        return Err("Invalid signature, not recognized as BMP file".to_string());
    }

    let file_size = binary::read_le_u32(&bmp, 2) as usize;
    if file_size != bmp.len() {
        return Err("Invalid file, expected size doesn't match the real one".to_string());
    }

    let data_address = binary::read_le_u32(&bmp, 10);
    let width = binary::read_le_u32(&bmp, 18);
    let height = binary::read_le_u32(&bmp, 22);
    let compression_method = binary::read_le_u32(&bmp, 30);

    let mut data = Vec::new();
    let mut data_index = data_address;

    match compression_method {
        0 => {
            for _ in 0..height {
                for _ in 0..width {
                    let b = binary::read_u8(&bmp, (data_index + 0) as usize);
                    let g = binary::read_u8(&bmp, (data_index + 1) as usize);
                    let r = binary::read_u8(&bmp, (data_index + 2) as usize);

                    data.push(r);
                    data.push(g);
                    data.push(b);
                    data.push(0xff);

                    data_index += 3;
                }

                data_index += (data_index - data_address) % 4;
            }
        }
        3 => {
            let r_mask = binary::read_le_u32(&bmp, 54);
            let g_mask = binary::read_le_u32(&bmp, 54 + 4);
            let b_mask = binary::read_le_u32(&bmp, 54 + 8);
            let a_mask = binary::read_le_u32(&bmp, 54 + 12);

            let r_offset = r_mask.trailing_zeros();
            let g_offset = g_mask.trailing_zeros();
            let b_offset = b_mask.trailing_zeros();
            let a_offset = a_mask.trailing_zeros();

            for i in 0..(width * height) {
                let pixel = binary::read_le_u32(&bmp, (data_address + i * 4) as usize);
                let r = (pixel & r_mask) >> r_offset;
                let g = (pixel & g_mask) >> g_offset;
                let b = (pixel & b_mask) >> b_offset;
                let a = (pixel & a_mask) >> a_offset;

                data.push(r as u8);
                data.push(g as u8);
                data.push(b as u8);
                data.push(a as u8);
            }
        }
        _ => return Err("Unsupported compression method, only BI_RGB and BI_BITFIELDS are supported".to_string()),
    }

    Ok(RawTexture::new(Vec2::new(width as f32, height as f32), data))
}
