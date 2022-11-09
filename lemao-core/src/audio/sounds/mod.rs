use super::samples::Sample;
use lemao_openal::bindings::openal;
use std::ffi::c_void;

pub mod storage;

pub struct Sound {
    pub id: usize,

    sample_id: usize,
    source_id: u32,
    buffer_id: u32,
}

impl Sound {
    pub fn new(sample: &Sample) -> Result<Self, String> {
        let mut sound = Sound { id: 0, sample_id: 0, source_id: 0, buffer_id: 0 };
        sound.set_sample(sample)?;

        Ok(sound)
    }

    pub fn get_sample(&self) -> usize {
        self.sample_id
    }

    pub fn set_sample(&mut self, sample: &Sample) -> Result<(), String> {
        unsafe {
            let format = match sample.channels_count {
                1 => match sample.bits_per_sample {
                    8 => openal::AL_FORMAT_MONO8,
                    16 => openal::AL_FORMAT_MONO16,
                    _ => return Err(format!("{} bits per sample not supported", sample.bits_per_sample)),
                },
                2 => match sample.bits_per_sample {
                    8 => openal::AL_FORMAT_STEREO8,
                    16 => openal::AL_FORMAT_STEREO16,
                    _ => return Err(format!("{} bits per sample not supported", sample.bits_per_sample)),
                },
                _ => return Err(format!("{} channels not supported", sample.channels_count)),
            };

            openal::alGenBuffers(1, &mut self.buffer_id);
            self.check_al_error()?;

            openal::alBufferData(self.buffer_id, format as i32, sample.data.as_ptr() as *const c_void, sample.data.len() as i32, sample.frequency as i32);
            self.check_al_error()?;

            openal::alGenSources(1, &mut self.source_id);
            self.check_al_error()?;

            openal::alSourcei(self.source_id, openal::AL_BUFFER as i32, self.buffer_id as i32);
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

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe {
            if self.buffer_id != 0 {
                openal::alDeleteBuffers(1, &self.buffer_id);
            }

            if self.source_id != 0 {
                openal::alDeleteSources(1, &self.source_id);
            }
        }
    }
}
