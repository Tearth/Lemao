pub mod storage;
pub mod wav;

pub struct Sample {
    pub id: usize,
    pub channels_count: u32,
    pub frequency: u32,
    pub bits_per_sample: u32,
    pub data: Vec<u8>,
}

impl Sample {
    pub fn new(channels_count: u32, frequency: u32, bits_per_sample: u32, data: Vec<u8>) -> Self {
        Self { id: 0, channels_count, frequency, bits_per_sample, data }
    }
}
