use super::{
    samples::storage::SampleStorage,
    sounds::{storage::SoundStorage, Sound},
};
use lemao_openal::bindings::openal;
use std::{
    ptr,
    sync::{Arc, Mutex},
};

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
            if device_id.is_null() {
                return Err("Error while creating a new device".to_string());
            }

            let context_id = openal::alcCreateContext(device_id, ptr::null_mut());
            if context_id.is_null() {
                return Err("Error while creating a new context".to_string());
            }

            openal::alcMakeContextCurrent(context_id);

            Ok(Self { device: device_id, context: context_id, samples, sounds: Default::default() })
        }
    }

    pub fn create_sound(&mut self, sample_id: usize) -> Result<usize, String> {
        let sample_storage = self.samples.lock().unwrap();
        let sample = match sample_storage.get(sample_id) {
            Some(sample) => sample,
            None => return Err(format!("Sample with id {} not found, the sound can't be created", sample_id)),
        };

        Ok(self.sounds.store(Sound::new(sample)?)?)
    }

    pub fn get_sound(&self, sound_id: usize) -> Option<&Sound> {
        self.sounds.get(sound_id)
    }

    pub fn get_volume(&self, sound_id: usize) -> f32 {
        unsafe {
            let sound = self.get_sound(sound_id).unwrap();
            let mut volume = 0.0;

            openal::alGetSourcef(sound.source_id, openal::AL_GAIN as i32, &mut volume);
            volume
        }
    }

    pub fn set_volume(&self, sound_id: usize, volume: f32) {
        unsafe {
            let sound = self.get_sound(sound_id).unwrap();
            openal::alSourcef(sound.source_id, openal::AL_GAIN as i32, volume);
        }
    }

    pub fn is_playing(&self, sound_id: usize) -> bool {
        unsafe {
            let sound = self.get_sound(sound_id).unwrap();
            let mut state = 0;

            openal::alGetSourcei(sound.source_id, openal::AL_SOURCE_STATE as i32, &mut state);
            state == openal::AL_PLAYING as i32
        }
    }

    pub fn play(&self, sound_id: usize) {
        unsafe {
            let sound = self.get_sound(sound_id).unwrap();
            openal::alSourcePlay(sound.source_id);
        }
    }

    pub fn pause(&self, sound_id: usize) {
        unsafe {
            let sound = self.get_sound(sound_id).unwrap();
            openal::alSourcePause(sound.source_id);
        }
    }

    pub fn stop(&self, sound_id: usize) {
        unsafe {
            let sound = self.get_sound(sound_id).unwrap();
            openal::alSourceStop(sound.source_id);
        }
    }

    pub fn rewind(&self, sound_id: usize) {
        unsafe {
            let sound = self.get_sound(sound_id).unwrap();
            openal::alSourceRewind(sound.source_id);
        }
    }
}

impl Drop for AudioContext {
    fn drop(&mut self) {
        unsafe {
            if !self.context.is_null() {
                openal::alcDestroyContext(self.context);
            }

            if !self.device.is_null() {
                openal::alcCloseDevice(self.device);
            }
        }
    }
}
