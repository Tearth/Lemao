use super::*;

#[derive(Default)]
pub struct CameraStorage {
    data: Vec<Option<Camera>>,
}

impl CameraStorage {
    pub fn store(&mut self, mut camera: Camera) -> usize {
        let id = self.get_free_component_id();
        camera.id = id;
        self.data[id] = Some(camera);

        id
    }

    pub fn get(&self, id: usize) -> Result<&Camera, String> {
        if id >= self.data.len() {
            return Err(format!("Camera with id {} not found", id));
        }

        self.data[id].as_ref().ok_or(format!("Camera with id {} not found", id))
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut Camera, String> {
        if id >= self.data.len() {
            return Err(format!("Camera with id {} not found", id));
        }

        self.data[id].as_mut().ok_or(format!("Camera with id {} not found", id))
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Camera with id {} not found", id));
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
