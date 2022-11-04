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

    pub fn get(&self, id: usize) -> Option<&dyn Drawable> {
        if id >= self.data.len() {
            return None;
        }

        match &self.data[id] {
            Some(drawable) => Some(drawable.as_ref()),
            None => None,
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut dyn Drawable> {
        if id >= self.data.len() {
            return None;
        }

        match &mut self.data[id] {
            Some(drawable) => Some(drawable.as_mut()),
            None => None,
        }
    }
}
