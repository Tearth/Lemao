pub fn read_u8(data: &[u8], index: usize) -> u8 {
    data[index]
}

pub fn read_le_u16(data: &[u8], index: usize) -> u16 {
    (read_u8(data, index) as u16) | ((read_u8(data, index + 1) as u16) << 8)
}

pub fn read_le_u32(data: &[u8], index: usize) -> u32 {
    (read_le_u16(data, index) as u32) | ((read_le_u16(data, index + 2) as u32) << 16)
}

pub fn read_le_u64(data: &[u8], index: usize) -> u64 {
    (read_le_u32(data, index) as u64) | ((read_le_u32(data, index + 4) as u64) << 32)
}
