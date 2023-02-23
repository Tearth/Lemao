use super::context::AudioContext;
use crate::utils::storage::StorageItem;
use lemao_openal::bindings::openal;
use std::any::Any;
use std::ffi::c_void;

pub mod wav;

pub struct RawSample {
    channels_count: u32,
    frequency: u32,
    bits_per_sample: u32,
    data: Vec<u8>,
}

pub struct Sample {
    pub(crate) id: usize,
    pub(crate) buffer_id: u32,
}

impl RawSample {
    pub fn new(channels_count: u32, frequency: u32, bits_per_sample: u32, data: Vec<u8>) -> Self {
        Self { channels_count, frequency, bits_per_sample, data }
    }

    pub fn get_channels_count(&self) -> u32 {
        self.channels_count
    }

    pub fn set_channels_count(&mut self, channels_count: u32) {
        self.channels_count = channels_count;
    }

    pub fn get_frequency(&self) -> u32 {
        self.frequency
    }

    pub fn set_frequency(&mut self, frequency: u32) {
        self.frequency = frequency;
    }

    pub fn get_bits_per_sample(&self) -> u32 {
        self.bits_per_sample
    }

    pub fn set_bits_per_sample(&mut self, bits_per_sample: u32) {
        self.bits_per_sample = bits_per_sample;
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }
}

impl Sample {
    pub fn new(_audio: &AudioContext, raw: &RawSample) -> Result<Self, String> {
        unsafe {
            let format = match raw.channels_count {
                1 => match raw.bits_per_sample {
                    8 => openal::AL_FORMAT_MONO8,
                    16 => openal::AL_FORMAT_MONO16,
                    _ => return Err(format!("{} bits per sample not supported", raw.bits_per_sample)),
                },
                2 => match raw.bits_per_sample {
                    8 => openal::AL_FORMAT_STEREO8,
                    16 => openal::AL_FORMAT_STEREO16,
                    _ => return Err(format!("{} bits per sample not supported", raw.bits_per_sample)),
                },
                _ => return Err(format!("{} channels not supported", raw.channels_count)),
            };

            let mut buffer_id = 0;

            openal::alGenBuffers(1, &mut buffer_id);
            openal::alBufferData(buffer_id, format as i32, raw.data.as_ptr() as *const c_void, raw.data.len() as i32, raw.frequency as i32);

            let error = openal::alGetError();
            if error != openal::AL_NO_ERROR as i32 {
                return Err(format!("OpenAL error, code {}", error));
            }

            Ok(Self { id: 0, buffer_id })
        }
    }
}

impl StorageItem for Sample {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Drop for Sample {
    fn drop(&mut self) {
        unsafe {
            if self.buffer_id != 0 {
                openal::alDeleteBuffers(1, &self.buffer_id);
            }
        }
    }
}
