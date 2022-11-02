use super::*;

pub struct ShaderStorage {
    data: Vec<Option<Shader>>,
    gl: Rc<OpenGLPointers>,
}

impl ShaderStorage {
    pub fn new(gl: Rc<OpenGLPointers>) -> Self {
        Self { data: Vec::new(), gl }
    }

    pub fn load(&mut self, vertex_shader: &str, fragment_shader: &str) -> Result<usize, String> {
        let id = self.data.len();
        let mut shader = Shader::new_from_string(self.gl.clone(), vertex_shader, fragment_shader).unwrap();

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
