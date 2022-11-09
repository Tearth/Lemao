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

        match &self.data[id] {
            Some(sound) => Some(sound),
            None => None,
        }
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Sound with id {} doesn't exist, so it can't be removed", id));
        }

        Ok(self.data[id] = None)
    }
}
