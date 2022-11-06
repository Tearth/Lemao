use crate::utils::log;

use super::samples::Sample;
use lemao_openal::bindings::openal;
use std::ffi::c_void;

pub mod storage;

pub struct Sound {
    pub id: usize,
    pub sample_id: usize,
    pub source_id: u32,
    pub buffer_id: u32,
}

impl Sound {
    pub fn new(sample: &Sample) -> Self {
        unsafe {
            let format = match sample.channels_count {
                1 => match sample.bits_per_sample {
                    8 => openal::AL_FORMAT_MONO8,
                    _ => openal::AL_FORMAT_MONO16,
                },
                _ => match sample.bits_per_sample {
                    8 => openal::AL_FORMAT_STEREO8,
                    _ => openal::AL_FORMAT_STEREO16,
                },
            };

            let mut buffer_id = 0;
            openal::alGenBuffers(1, &mut buffer_id);
            openal::alBufferData(buffer_id, format as i32, sample.data.as_ptr() as *const c_void, sample.data.len() as i32, sample.frequency as i32);

            let mut source_id = 0;
            openal::alGenSources(1, &mut source_id);
            openal::alSourcei(source_id, openal::AL_BUFFER as i32, buffer_id as i32);

            Sound { id: 0, sample_id: sample.id, source_id, buffer_id }
        }
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe {
            if self.buffer_id != 0 {
                log::debug(&format!("Deleting buffer with al_id {}", self.buffer_id));
                openal::alDeleteBuffers(1, &self.buffer_id);
                log::debug("Deleting buffer done");
            }

            if self.source_id != 0 {
                log::debug(&format!("Deleting source with al_id {}", self.source_id));
                openal::alDeleteSources(1, &self.source_id);
                log::debug("Deleting source done");
            }
        }
    }
}
