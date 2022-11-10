use super::*;

#[derive(Default)]
pub struct ShaderStorage {
    data: Vec<Option<Shader>>,
}

impl ShaderStorage {
    pub fn store(&mut self, mut shader: Shader) -> usize {
        let id = self.data.len();
        shader.id = id;
        self.data.push(Some(shader));

        id
    }

    pub fn get(&self, id: usize) -> Result<&Shader, String> {
        if id >= self.data.len() {
            return Err(format!("Shader with id {} not found", id));
        }

        self.data[id].as_ref().ok_or(format!("Shader with id {} not found", id))
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut Shader, String> {
        if id >= self.data.len() {
            return Err(format!("Shader with id {} not found", id));
        }

        self.data[id].as_mut().ok_or(format!("Shader with id {} not found", id))
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Shader with id {} not found", id));
        }
        self.data[id] = None;

        Ok(())
    }
}
