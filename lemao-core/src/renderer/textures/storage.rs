use super::*;

#[derive(Default)]
pub struct TextureStorage {
    data: Vec<Option<Texture>>,
}

impl TextureStorage {
    pub fn store(&mut self, mut texture: Texture) -> usize {
        let id = self.data.len();
        texture.id = id;
        self.data.push(Some(texture));

        id
    }

    pub fn get(&self, id: usize) -> Option<&Texture> {
        if id >= self.data.len() {
            return None;
        }

        self.data[id].as_ref()
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Texture with id {} doesn't exist, can't be removed", id));
        }
        self.data[id] = None;

        Ok(())
    }
}
