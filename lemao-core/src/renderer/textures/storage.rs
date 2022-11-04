use super::*;

#[derive(Default)]
pub struct TextureStorage {
    data: Vec<Option<Texture>>,
}

impl TextureStorage {
    pub fn load(&mut self, path: &str) -> Result<usize, String> {
        let id = self.data.len();
        let mut texture = bmp::load(path)?;

        texture.id = id;
        self.data.push(Some(texture));

        Ok(id)
    }

    pub fn get(&self, id: usize) -> Option<&Texture> {
        if id >= self.data.len() {
            return None;
        }

        match &self.data[id] {
            Some(texture) => Some(texture),
            None => None,
        }
    }
}
