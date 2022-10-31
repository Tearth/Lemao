use lemao_opengl::pointers::OpenGLPointers;

pub mod sprite;

pub trait Drawable {
    fn draw(&self, gl: &OpenGLPointers);
}
