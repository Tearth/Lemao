use super::context::AudioContext;
use lemao_openal::bindings::openal;
use std::ffi::c_void;

pub mod storage;
pub mod wav;

pub struct Sample {
    pub id: usize,
    pub buffer_id: u32,

    pub channels_count: u32,
    pub frequency: u32,
    pub bits_per_sample: u32,
    pub data: Vec<u8>,
}

impl Sample {
    pub fn new(audio: &AudioContext, channels_count: u32, frequency: u32, bits_per_sample: u32, data: Vec<u8>) -> Result<Self, String> {
        unsafe {
            let format = match channels_count {
                1 => match bits_per_sample {
                    8 => openal::AL_FORMAT_MONO8,
                    16 => openal::AL_FORMAT_MONO16,
                    _ => return Err(format!("{} bits per sample not supported", bits_per_sample)),
                },
                2 => match bits_per_sample {
                    8 => openal::AL_FORMAT_STEREO8,
                    16 => openal::AL_FORMAT_STEREO16,
                    _ => return Err(format!("{} bits per sample not supported", bits_per_sample)),
                },
                _ => return Err(format!("{} channels not supported", channels_count)),
            };

            let mut buffer_id = 0;

            openal::alGenBuffers(1, &mut buffer_id);
            openal::alBufferData(buffer_id, format as i32, data.as_ptr() as *const c_void, data.len() as i32, frequency as i32);

            let error = openal::alGetError();
            if error != openal::AL_NO_ERROR as i32 {
                return Err(format!("OpenAL error, code {}", error));
            }

            Ok(Self { id: 0, buffer_id, channels_count, frequency, bits_per_sample, data })
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
