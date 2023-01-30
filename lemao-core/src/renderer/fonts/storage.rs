use super::*;

#[derive(Default)]
pub struct FontStorage {
    data: Vec<Option<Font>>,
}

impl FontStorage {
    pub fn store(&mut self, mut font: Font) -> usize {
        let id = self.get_free_component_id();
        font.id = id;
        self.data[id] = Some(font);

        id
    }

    pub fn get(&self, id: usize) -> Result<&Font, String> {
        if id >= self.data.len() {
            return Err(format!("Font with id {} not found", id));
        }

        self.data[id].as_ref().ok_or(format!("Font with id {} not found", id))
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut Font, String> {
        if id >= self.data.len() {
            return Err(format!("Font with id {} not found", id));
        }

        self.data[id].as_mut().ok_or(format!("Font with id {} not found", id))
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Font with id {} not found", id));
        }
        self.data[id] = None;

        Ok(())
    }

    fn get_free_component_id(&mut self) -> usize {
        if let Some(id) = self.data.iter().position(|p| p.is_none()) {
            id
        } else {
            self.data.push(None);
            self.data.len() - 1
        }
    }
}
