use super::*;
use crate::renderer::context::RendererContext;
use crate::renderer::shapes::Shape;
use crate::renderer::textures::Texture;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::ptr;
use std::rc::Rc;

#[derive(Debug)]
pub struct Line {
    pub(crate) shape_id: usize,
    pub(crate) shape_vao_gl_id: u32,
    pub(crate) texture_gl_id: u32,
    gl: Rc<OpenGLPointers>,

    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
    pub size: Vec2,
    pub color: Color,
    pub from: Vec2,
    pub to: Vec2,
    pub thickness: f32,
}

impl Line {
    pub fn new(renderer: &RendererContext, shape: &Shape, texture: &Texture) -> Self {
        Line {
            shape_id: shape.id,
            shape_vao_gl_id: shape.vao_gl_id,
            texture_gl_id: texture.texture_gl_id,
            gl: renderer.gl.clone(),

            position: Default::default(),
            scale: Vec2::new(1.0, 1.0),
            rotation: 1.0,
            size: Default::default(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            from: Default::default(),
            to: Default::default(),
            thickness: 1.0,
        }
    }

    pub fn update(&mut self) {
        self.position = self.from;
        self.rotation = Vec2::new(0.0, 1.0).signed_angle(self.to - self.from);
        self.size = Vec2::new(self.thickness, self.from.distance(self.to) + 1.0);
    }
}

impl Drawable for Line {
    fn get_transformation_matrix(&self) -> Mat4x4 {
        let translation = Mat4x4::translate(Vec3::from(self.position + Vec2::new(0.5, 0.5)));
        let anchor_offset = Mat4x4::translate(Vec3::new(0.0, -0.5, 0.0));
        let scale = Mat4x4::scale(Vec3::from(self.scale * self.size).floor());
        let rotation = Mat4x4::rotate(self.rotation);
        translation * rotation * anchor_offset * scale
    }

    fn get_batch(&self) -> Batch {
        Batch::new(Some(self.shape_id), None, None, Some(self.texture_gl_id), Some(&self.color))
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn draw(&mut self, shader: &Shader) -> Result<(), String> {
        unsafe {
            let model = self.get_transformation_matrix();

            shader.set_parameter("model", model.as_ptr())?;
            shader.set_color(&self.color)?;

            (self.gl.glBindVertexArray)(self.shape_vao_gl_id);
            (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_gl_id);
            (self.gl.glDrawElements)(opengl::GL_TRIANGLES, 6, opengl::GL_UNSIGNED_INT, ptr::null());

            Ok(())
        }
    }
}
