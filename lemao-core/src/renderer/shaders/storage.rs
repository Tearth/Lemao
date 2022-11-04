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

        match &self.data[id] {
            Some(shader) => Some(shader),
            None => None,
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Shader> {
        if id >= self.data.len() {
            return None;
        }

        match &mut self.data[id] {
            Some(shader) => Some(shader),
            None => None,
        }
    }
}
