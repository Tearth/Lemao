use super::*;

#[derive(Default)]
pub struct ShaderStorage {
    data: Vec<Option<Shader>>,
}

impl ShaderStorage {
    pub fn store(&mut self, mut shader: Shader) -> Result<usize, String> {
        let id = self.data.len();
        shader.id = id;
        self.data.push(Some(shader));

        Ok(id)
    }

    pub fn get(&self, id: usize) -> &Shader {
        match &self.data[id] {
            Some(shader) => shader,
            None => panic!(""),
        }
    }
}
