use super::samples::Sample;
use super::sounds::Sound;
use crate::utils::storage::Storage;
use lemao_openal::bindings::openal;
use std::ptr;
use std::sync::Arc;
use std::sync::RwLock;

pub struct AudioContext {
    device: *mut openal::ALCdevice_struct,
    context: *mut openal::ALCcontext_struct,
    samples: Arc<RwLock<Storage>>,
    sounds: Storage,
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

    pub fn get_samples(&self) -> Arc<RwLock<Storage>> {
        self.samples.clone()
    }

    pub fn create_sound(&mut self, sample_id: usize) -> Result<usize, String> {
        let sample_storage = self.samples.read().unwrap();
        let sample = sample_storage.get_and_cast::<Sample>(sample_id)?;

        Ok(self.sounds.store(Box::new(Sound::new(sample)?)))
    }

    pub fn get_sound(&self, sound_id: usize) -> Result<&Sound, String> {
        self.sounds.get_and_cast::<Sound>(sound_id)
    }

    pub fn get_sound_mut(&mut self, sound_id: usize) -> Result<&mut Sound, String> {
        self.sounds.get_and_cast_mut::<Sound>(sound_id)
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
