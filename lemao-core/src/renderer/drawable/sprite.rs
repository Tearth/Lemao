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

pub struct Sprite {
    id: usize,
    position: Vec2,
    scale: Vec2,
    rotation: f32,
    width: u32,
    height: u32,
    anchor: Vec2,
    color: Color,
    texture_id: usize,
    shape_vao_gl_id: u32,
    texture_gl_id: u32,
    gl: Rc<OpenGLPointers>,
}

impl Sprite {
    pub fn new(renderer: &RendererContext, shape: &Shape, texture: &Texture) -> Self {
        let mut sprite = Sprite {
            id: 0,
            position: Default::default(),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
            width: 0,
            height: 0,
            anchor: Default::default(),
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            texture_id: texture.id,
            shape_vao_gl_id: shape.vao_gl_id,
            texture_gl_id: 0,
            gl: renderer.gl.clone(),
        };

        sprite.set_texture(texture);
        sprite
    }

    pub fn get_texture(&self) -> usize {
        self.texture_id
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        unsafe {
            self.width = texture.width;
            self.height = texture.height;
            self.texture_id = texture.id;
            self.texture_gl_id = texture.texture_gl_id;
            self.set_anchor(Vec2::new(0.5, 0.5));
        }
    }
}

impl Drawable for Sprite {
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

    fn set_anchor(&mut self, anchor: Vec2) -> Result<(), String> {
        self.anchor = anchor;
        Ok(())
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) -> Result<(), String> {
        unsafe {
            /*if self.vbo_gl_id == 0 {
                return Err("Sprite not initialized".to_string());
            }

            let vertices = self.get_vertices(self.anchor, self.width, self.height, color);
            let vertices_size = (mem::size_of::<f32>() * vertices.len()) as i64;

            (self.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, self.vbo_gl_id);
            (self.gl.glBufferData)(opengl::GL_ARRAY_BUFFER, vertices_size, vertices.as_ptr() as *const c_void, opengl::GL_STATIC_DRAW);

            self.color = color;*/
            Ok(())
        }
    }

    fn draw(&self, shader: &Shader) -> Result<(), String> {
        unsafe {
            let translation = Mat4x4::translate(Vec3::new(self.position.x, self.position.y, 0.0));
            let anchor_positive_offset = Mat4x4::translate(Vec3::new(self.anchor.x, self.anchor.y, 0.0));
            let anchor_negative_offset = Mat4x4::translate(Vec3::new(-self.anchor.x, -self.anchor.y, 0.0));
            let scale = Mat4x4::scale(Vec3::new(self.scale.x * self.width as f32, self.scale.y * self.height as f32, 1.0));
            let rotation = Mat4x4::rotate(self.rotation);
            let model = translation * anchor_positive_offset * rotation * scale * anchor_negative_offset;

            shader.set_parameter("model", model.as_ptr())?;

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
