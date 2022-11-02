use super::shaders::Shader;
use std::rc::Rc;

pub mod sprite;
pub mod storage;

pub trait Drawable {
    fn draw(&self, shader: &Shader);
}
