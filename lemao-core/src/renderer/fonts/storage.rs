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

        self.data[id].as_ref()
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Font> {
        if id >= self.data.len() {
            return None;
        }

        self.data[id].as_mut()
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Font with id {} doesn't exist, can't be removed", id));
        }
        self.data[id] = None;

        Ok(())
    }
}
