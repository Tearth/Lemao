use super::*;
use crate::renderer::context::RendererContext;
use crate::renderer::shapes::Shape;
use crate::renderer::textures::Texture;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::any::Any;
use std::ptr;
use std::rc::Rc;

pub struct Rectangle {
    id: usize,
    position: Vec2,
    scale: Vec2,
    rotation: f32,
    size: Vec2,
    anchor: Vec2,
    color: Color,
    texture_id: usize,
    shape_vao_gl_id: u32,
    texture_gl_id: u32,
    gl: Rc<OpenGLPointers>,
}

impl Rectangle {
    pub fn new(renderer: &RendererContext, shape: &Shape, texture: &Texture, size: Vec2) -> Self {
        Rectangle {
            id: 0,
            position: Default::default(),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
            size,
            anchor: Default::default(),
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            texture_id: texture.id,
            shape_vao_gl_id: shape.vao_gl_id,
            texture_gl_id: texture.texture_gl_id,
            gl: renderer.gl.clone(),
        }
    }

    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    pub fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }

    pub fn get_texture(&self) -> usize {
        self.texture_id
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.texture_id = texture.id;
        self.texture_gl_id = texture.texture_gl_id;
    }
}

impl Drawable for Rectangle {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    fn move_delta(&mut self, delta: Vec2) {
        self.position += delta;
    }

    fn get_scale(&self) -> Vec2 {
        self.scale
    }

    fn set_scale(&mut self, scale: Vec2) {
        self.scale = scale;
    }

    fn get_rotation(&self) -> f32 {
        self.rotation
    }

    fn set_rotation(&mut self, rotation: f32) {
        self.rotation = rotation;
    }

    fn rotate(&mut self, delta: f32) {
        self.rotation += delta;
    }

    fn get_anchor(&self) -> Vec2 {
        self.anchor
    }

    fn set_anchor(&mut self, anchor: Vec2) {
        self.anchor = anchor;
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn draw(&self, shader: &Shader) -> Result<(), String> {
        unsafe {
            let translation = Mat4x4::translate(Vec3::from(self.position));
            let anchor_offset = Mat4x4::translate(-Vec3::from(self.anchor));
            let scale = Mat4x4::scale(Vec3::from(self.scale * self.size));
            let rotation = Mat4x4::rotate(self.rotation);
            let model = translation * rotation * scale * anchor_offset;

            shader.set_parameter("model", model.as_ptr())?;
            shader.set_parameter("color", self.color.as_ptr())?;

            (self.gl.glBindVertexArray)(self.shape_vao_gl_id);
            (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_gl_id);
            (self.gl.glDrawElements)(opengl::GL_TRIANGLES, 6, opengl::GL_UNSIGNED_INT, ptr::null());

            Ok(())
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}