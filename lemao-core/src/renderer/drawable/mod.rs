use lemao_opengl::context::OpenGLContext;

pub mod sprite;

pub trait Drawable {
    fn draw(&self, gl: &OpenGLContext);
}
