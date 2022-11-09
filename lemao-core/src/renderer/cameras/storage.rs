use super::*;

#[derive(Default)]
pub struct CameraStorage {
    data: Vec<Option<Camera>>,
}

impl CameraStorage {
    pub fn store(&mut self, mut camera: Camera) -> usize {
        let id = self.data.len();
        camera.id = id;
        self.data.push(Some(camera));

        id
    }

    pub fn get(&self, id: usize) -> Option<&Camera> {
        if id >= self.data.len() {
            return None;
        }

        self.data[id].as_ref()
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Camera> {
        if id >= self.data.len() {
            return None;
        }

        self.data[id].as_mut()
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Camera with id {} not found", id));
        }
        self.data[id] = None;

        Ok(())
    }
}
