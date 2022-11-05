use super::*;
use crate::utils::binary;
use crate::utils::log;
use std::fs::File;
use std::io::Read;

pub fn load(path: &str) -> Result<Font, String> {
    /////////////////////////////////////////////////////////////////////////////////////////////////
    // BFF specification: https://documentation.help/Codehead-Bitmap-Font-Generator/bffformat.html //
    /////////////////////////////////////////////////////////////////////////////////////////////////

    log::debug(&format!("Loading a new BFF file {}", path));

    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => return Err(format!("File {} not found", path)),
    };

    let mut bff = Vec::new();
    if let Err(message) = file.read_to_end(&mut bff) {
        return Err(format!("Error while reading file: {}", message));
    }

    if binary::read_le_u16(&bff, 0) != 0xf2bf {
        return Err("Invalid signature, not recognized as BFF file".to_string());
    }

    let width = binary::read_le_u32(&bff, 2);
    let height = binary::read_le_u32(&bff, 6);
    let cell_width = binary::read_le_u32(&bff, 10);
    let cell_height = binary::read_le_u32(&bff, 14);
    let bits_per_pixel = binary::read_u8(&bff, 18);
    let base_character_offset = binary::read_u8(&bff, 19);

    let mut character_widths = Vec::new();
    for index in 0..256 {
        character_widths.push(binary::read_u8(&bff, 20 + index));
    }

    let mut data = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let index = x + (height - y - 1) * height;
            let (r, g, b, a) = match bits_per_pixel {
                8 => (0xff, 0xff, 0xff, binary::read_u8(&bff, (276 + index) as usize)),
                24 => (
                    binary::read_u8(&bff, (276 + index * 3 + 0) as usize),
                    binary::read_u8(&bff, (276 + index * 3 + 1) as usize),
                    binary::read_u8(&bff, (276 + index * 3 + 2) as usize),
                    0xff,
                ),
                32 => (
                    binary::read_u8(&bff, (276 + index * 4 + 0) as usize),
                    binary::read_u8(&bff, (276 + index * 4 + 1) as usize),
                    binary::read_u8(&bff, (276 + index * 4 + 2) as usize),
                    binary::read_u8(&bff, (276 + index * 4 + 3) as usize),
                ),
                _ => return Err("Unsupported format, only 8, 24 and 32 bits per pixel are allowed".to_string()),
            };

            data.push(r);
            data.push(g);
            data.push(b);
            data.push(a);
        }
    }

    Ok(Font { id: 0, width, height, cell_width, cell_height, bits_per_pixel, base_character_offset, character_widths, data })
}
