use super::*;

#[derive(Default)]
pub struct DrawableStorage {
    data: Vec<Option<Box<dyn Drawable>>>,
}

impl DrawableStorage {
    pub fn store(&mut self, mut drawable: Box<dyn Drawable>) -> usize {
        let id = self.data.len();
        drawable.set_id(id);
        self.data.push(Some(drawable));

        id
    }

    pub fn get(&self, id: usize) -> Result<&dyn Drawable, String> {
        if id >= self.data.len() {
            return Err(format!("Drawable with id {} not found", id));
        }

        match &self.data[id] {
            Some(drawable) => Ok(drawable.as_ref()),
            None => return Err(format!("Drawable with id {} not found", id)),
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut dyn Drawable, String> {
        if id >= self.data.len() {
            return Err(format!("Drawable with id {} not found", id));
        }

        match &mut self.data[id] {
            Some(drawable) => Ok(drawable.as_mut()),
            None => return Err(format!("Drawable with id {} not found", id)),
        }
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Drawable with id {} not found", id));
        }
        self.data[id] = None;

        Ok(())
    }
}
