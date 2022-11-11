use super::*;

#[derive(Default)]
pub struct ShapeStorage {
    data: Vec<Option<Shape>>,
}

impl ShapeStorage {
    pub fn store(&mut self, mut shape: Shape) -> usize {
        let id = self.data.len();
        shape.id = id;
        self.data.push(Some(shape));

        id
    }

    pub fn get(&self, id: usize) -> Result<&Shape, String> {
        if id >= self.data.len() {
            return Err(format!("Shape with id {} not found", id));
        }

        self.data[id].as_ref().ok_or(format!("Shape with id {} not found", id))
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut Shape, String> {
        if id >= self.data.len() {
            return Err(format!("Shape with id {} not found", id));
        }

        self.data[id].as_mut().ok_or(format!("Shape with id {} not found", id))
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Shape with id {} not found", id));
        }
        self.data[id] = None;

        Ok(())
    }
}
