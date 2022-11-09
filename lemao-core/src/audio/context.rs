use super::samples::storage::SampleStorage;
use super::sounds::storage::SoundStorage;
use super::sounds::Sound;
use lemao_openal::bindings::openal;
use std::ptr;
use std::sync::Arc;
use std::sync::Mutex;

pub struct AudioContext {
    device: *mut openal::ALCdevice_struct,
    context: *mut openal::ALCcontext_struct,
    samples: Arc<Mutex<SampleStorage>>,
    sounds: SoundStorage,
}

impl AudioContext {
    pub fn new(samples: Arc<Mutex<SampleStorage>>) -> Result<Self, String> {
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

            Ok(Self { device: device_id, context: context_id, samples, sounds: Default::default() })
        }
    }

    pub fn create_sound(&mut self, sample_id: usize) -> Result<usize, String> {
        let sample_storage = self.samples.lock().unwrap();
        let sample = match sample_storage.get(sample_id) {
            Some(sample) => sample,
            None => return Err(format!("Sample with id {} not found", sample_id)),
        };

        self.sounds.store(Sound::new(sample)?)
    }

    pub fn get_sound(&self, sound_id: usize) -> Option<&Sound> {
        self.sounds.get(sound_id)
    }

    pub fn get_sound_scope<F>(&self, sound_id: usize, scope: F) -> Result<(), String>
    where
        F: Fn(&Sound) -> Result<(), String>,
    {
        match self.sounds.get(sound_id) {
            Some(sound) => scope(sound),
            None => return Err(format!("Sound with id {} not found", sound_id)),
        }
    }

    pub fn get_sound_mut(&mut self, sound_id: usize) -> Option<&mut Sound> {
        self.sounds.get_mut(sound_id)
    }

    pub fn get_sound_scope_mut<F>(&mut self, sound_id: usize, mut scope: F) -> Result<(), String>
    where
        F: FnMut(&mut Sound) -> Result<(), String>,
    {
        match self.sounds.get_mut(sound_id) {
            Some(sound) => scope(sound),
            None => return Err(format!("Sound with id {} not found", sound_id)),
        }
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
