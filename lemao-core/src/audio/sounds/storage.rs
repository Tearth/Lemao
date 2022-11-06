use super::*;

#[derive(Default)]
pub struct SoundStorage {
    data: Vec<Option<Sound>>,
}

impl SoundStorage {
    pub fn store(&mut self, mut sound: Sound) -> usize {
        let id = self.data.len();
        sound.id = id;
        self.data.push(Some(sound));

        id
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
}
