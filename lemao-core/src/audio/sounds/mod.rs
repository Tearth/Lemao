use std::any::Any;

use crate::utils::storage::StorageItem;

use super::samples::Sample;
use lemao_openal::bindings::openal;

pub struct Sound {
    pub(crate) id: usize,
    pub(crate) sample_id: usize,
    pub(crate) source_id: u32,
}

impl Sound {
    pub fn new(sample: &Sample) -> Result<Self, String> {
        let mut sound = Sound { id: 0, sample_id: 0, source_id: 0 };
        sound.set_sample(sample)?;

        Ok(sound)
    }
    pub fn get_sample_id(&self) -> usize {
        self.sample_id
    }

    pub fn set_sample(&mut self, sample: &Sample) -> Result<(), String> {
        unsafe {
            if self.source_id == 0 {
                openal::alGenSources(1, &mut self.source_id);
            }

            openal::alSourcei(self.source_id, openal::AL_BUFFER as i32, sample.buffer_id as i32);

            self.sample_id = sample.id;
            self.check_al_error()?;

            Ok(())
        }
    }

    pub fn get_volume(&self) -> Result<f32, String> {
        unsafe {
            let mut volume = 0.0;

            openal::alGetSourcef(self.source_id, openal::AL_GAIN as i32, &mut volume);
            self.check_al_error()?;

            Ok(volume)
        }
    }

    pub fn set_volume(&mut self, volume: f32) -> Result<(), String> {
        unsafe {
            if !(0.0..=1.0).contains(&volume) {
                return Err("Value of the volume expected to be between 0.0 and 1.0".to_string());
            }

            openal::alSourcef(self.source_id, openal::AL_GAIN as i32, volume);
            self.check_al_error()
        }
    }

    pub fn is_playing(&self) -> Result<bool, String> {
        unsafe {
            let mut state = 0;

            openal::alGetSourcei(self.source_id, openal::AL_SOURCE_STATE as i32, &mut state);
            self.check_al_error()?;

            Ok(state == openal::AL_PLAYING as i32)
        }
    }

    pub fn play(&mut self) -> Result<(), String> {
        unsafe {
            openal::alSourcePlay(self.source_id);
            self.check_al_error()
        }
    }

    pub fn pause(&mut self) -> Result<(), String> {
        unsafe {
            openal::alSourcePause(self.source_id);
            self.check_al_error()
        }
    }

    pub fn stop(&mut self) -> Result<(), String> {
        unsafe {
            openal::alSourceStop(self.source_id);
            self.check_al_error()
        }
    }

    pub fn rewind(&mut self) -> Result<(), String> {
        unsafe {
            openal::alSourceRewind(self.source_id);
            self.check_al_error()
        }
    }

    fn check_al_error(&self) -> Result<(), String> {
        unsafe {
            let error = openal::alGetError();
            if error != openal::AL_NO_ERROR as i32 {
                return Err(format!("OpenAL error, code {}", error));
            }

            Ok(())
        }
    }
}

impl StorageItem for Sound {
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

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe {
            if self.source_id != 0 {
                openal::alDeleteSources(1, &self.source_id);
            }
        }
    }
}
