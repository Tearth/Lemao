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

    pub fn get(&self, id: usize) -> Option<&Shader> {
        if id >= self.data.len() {
            return None;
        }

        self.data[id].as_ref()
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Shader> {
        if id >= self.data.len() {
            return None;
        }

        self.data[id].as_mut()
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() {
            return Err(format!("Shader with id {} doesn't exist, can't be removed", id));
        }
        self.data[id] = None;

        Ok(())
    }
}
