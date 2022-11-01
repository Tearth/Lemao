use super::shaders::Shader;
use std::rc::Rc;

pub mod sprite;

pub trait Drawable {
    fn draw(&self, shader: &Rc<Shader>);
}
