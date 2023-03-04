use super::context::AudioContext;
use lemao_openal::bindings::openal;
use std::ffi::c_void;

pub mod wav;

pub struct RawSample {
    pub channels_count: u32,
    pub frequency: u32,
    pub bits_per_sample: u32,
    pub data: Vec<u8>,
}

pub struct Sample {
    pub(crate) id: usize,
    pub(crate) buffer_id: u32,
}

impl RawSample {
    pub fn new(channels_count: u32, frequency: u32, bits_per_sample: u32, data: Vec<u8>) -> Self {
        Self { channels_count, frequency, bits_per_sample, data }
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

impl Drop for Sample {
    fn drop(&mut self) {
        unsafe {
            if self.buffer_id != 0 {
                openal::alDeleteBuffers(1, &self.buffer_id);
            }
        }
    }
}
