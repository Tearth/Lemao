pub mod storage;
pub mod wav;

pub struct Sample {
    pub id: usize,
    pub channels_count: u16,
    pub frequency: u32,
    pub bits_per_sample: u16,
    pub data: Vec<u8>,
}
