use lemao_opengl::pointers::OpenGLPointers;

use crate::renderer::textures::Texture;

use super::{sprite::Sprite, *};

pub struct SpriteStorage {
    data: Vec<Option<Sprite>>,
    gl: Rc<OpenGLPointers>,
}

impl SpriteStorage {
    pub fn new(gl: Rc<OpenGLPointers>) -> Self {
        Self { data: Vec::new(), gl }
    }

    pub fn load(&mut self, texture: &Texture) -> Result<usize, String> {
        let id = self.data.len();
        let mut sprite = Sprite::new(self.gl.clone(), texture);

        self.data.push(Some(sprite));

        Ok(id)
    }

    pub fn get(&self, id: usize) -> &Sprite {
        match &self.data[id] {
            Some(sprite) => sprite,
            None => panic!(""),
        }
    }

    pub fn get_mut(&mut self, id: usize) -> &mut Sprite {
        match &mut self.data[id] {
            Some(sprite) => sprite,
            None => panic!(""),
        }
    }
}
