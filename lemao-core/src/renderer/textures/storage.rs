use super::*;

pub struct TextureStorage {
    data: Vec<Option<Texture>>,
}

impl TextureStorage {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn load(&mut self, path: &str) -> Result<usize, String> {
        let id = self.data.len();
        let mut texture = bmp::load(path)?;

        texture.id = id;
        self.data.push(Some(texture));

        Ok(id)
    }

    pub fn get(&self, id: usize) -> &Texture {
        match &self.data[id] {
            Some(texture) => texture,
            None => panic!(""),
        }
    }
}
