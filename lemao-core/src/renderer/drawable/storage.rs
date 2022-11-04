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

    pub fn get(&self, id: usize) -> &dyn Drawable {
        match &self.data[id] {
            Some(drawable) => drawable.as_ref(),
            None => panic!(""),
        }
    }

    pub fn get_mut(&mut self, id: usize) -> &mut dyn Drawable {
        match &mut self.data[id] {
            Some(drawable) => drawable.as_mut(),
            None => panic!(""),
        }
    }
}
