use super::*;

#[derive(Default)]
pub struct FontStorage {
    data: Vec<Option<Font>>,
}

impl FontStorage {
    pub fn load(&mut self, path: &str) -> Result<usize, String> {
        let id = self.data.len();
        let mut font = bff::load(path)?;

        font.id = id;
        self.data.push(Some(font));

        Ok(id)
    }

    pub fn store(&mut self, mut font: Font) -> usize {
        let id = self.data.len();
        font.id = id;
        self.data.push(Some(font));

        id
    }

    pub fn get(&self, id: usize) -> Option<&Font> {
        if id >= self.data.len() {
            return None;
        }

        match &self.data[id] {
            Some(font) => Some(font),
            None => None,
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Font> {
        if id >= self.data.len() {
            return None;
        }

        match &mut self.data[id] {
            Some(font) => Some(font),
            None => None,
        }
    }
}
