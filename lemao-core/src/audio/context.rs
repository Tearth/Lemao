use super::samples::wav;
use super::samples::Sample;
use super::sounds::Sound;
use crate::utils::storage::Storage;
use lemao_openal::bindings::openal;
use std::ptr;

pub struct AudioContext {
    device: *mut openal::ALCdevice_struct,
    context: *mut openal::ALCcontext_struct,

    pub samples: Storage<Sample>,
    pub sounds: Storage<Sound>,
}

impl AudioContext {
    pub fn new() -> Result<Self, String> {
        unsafe {
            let device_id = openal::alcOpenDevice(ptr::null());
            let error = openal::alcGetError(device_id);

            if device_id.is_null() || error != openal::AL_NO_ERROR as i32 {
                return Err(format!("Error while creating a new device: {}", error));
            }

            let context_id = openal::alcCreateContext(device_id, ptr::null_mut());
            let error = openal::alcGetError(device_id);

            if context_id.is_null() || error != openal::AL_NO_ERROR as i32 {
                return Err(format!("Error while creating a new context: {}", error));
            }

            let success = openal::alcMakeContextCurrent(context_id);
            let error = openal::alcGetError(device_id);

            if success == 0 || error != openal::AL_NO_ERROR as i32 {
                return Err(format!("Error while making context as current: {}", error));
            }

            Ok(Self { device: device_id, context: context_id, samples: Default::default(), sounds: Default::default() })
        }
    }

    pub fn create_sample(&mut self, path: &str) -> Result<usize, String> {
        let sample = Sample::new(self, &wav::load(path)?)?;
        let id = self.samples.store(sample);
        self.samples.get_mut(id)?.id = id;

        Ok(id)
    }

    pub fn create_sound(&mut self, sample_id: usize) -> Result<usize, String> {
        let sample = self.samples.get(sample_id)?;
        let id = self.sounds.store(Sound::new(sample)?);
        self.sounds.get_mut(id)?.id = id;

        Ok(id)
    }
}

impl Drop for AudioContext {
    fn drop(&mut self) {
        unsafe {
            openal::alcMakeContextCurrent(ptr::null_mut());

            if !self.context.is_null() {
                openal::alcDestroyContext(self.context);
            }

            if !self.device.is_null() {
                openal::alcCloseDevice(self.device);
            }
        }
    }
}
