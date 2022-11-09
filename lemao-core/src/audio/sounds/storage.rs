use super::*;

#[derive(Default)]
pub struct SoundStorage {
    data: Vec<Option<Sound>>,
}

impl SoundStorage {
    pub fn store(&mut self, mut sound: Sound) -> Result<usize, String> {
        let id = self.data.len();
        sound.id = id;
        self.data.push(Some(sound));

        Ok(id)
    }

    pub fn get(&self, id: usize) -> Option<&Sound> {
        if id >= self.data.len() {
            return None;
        }

        self.data[id].as_ref()
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Sound> {
        if id >= self.data.len() {
            return None;
        }

        self.data[id].as_mut()
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Sound with id {} not found", id));
        }
        self.data[id] = None;

        Ok(())
    }
}
