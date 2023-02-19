use super::*;
use crate::renderer::context::RendererContext;
use crate::renderer::shapes::Shape;
use crate::renderer::textures::Texture;
use crate::utils::storage::StorageItem;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::any::Any;
use std::ptr;
use std::rc::Rc;

pub struct Line {
    pub(crate) id: usize,
    pub(crate) shape_id: usize,
    pub(crate) shape_vao_gl_id: u32,
    pub(crate) texture_id: usize,
    pub(crate) texture_gl_id: u32,
    gl: Rc<OpenGLPointers>,

    position: Vec2,
    scale: Vec2,
    rotation: f32,
    size: Vec2,
    color: Color,
    from: Vec2,
    to: Vec2,
    thickness: f32,
}

impl Line {
    pub fn new(renderer: &RendererContext, shape: &Shape, texture: &Texture, from: Vec2, to: Vec2) -> Self {
        let mut line = Line {
            id: 0,
            shape_id: shape.id,
            shape_vao_gl_id: shape.vao_gl_id,
            texture_id: texture.id,
            texture_gl_id: texture.texture_gl_id,
            gl: renderer.gl.clone(),

            position: Default::default(),
            scale: Vec2::new(1.0, 1.0),
            rotation: 1.0,
            size: Default::default(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            from,
            to,
            thickness: 1.0,
        };

        line.calculate_position_rotation_scale();
        line
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_texture_id(&self) -> usize {
        self.texture_id
    }

    pub fn get_from(&self) -> Vec2 {
        self.from
    }

    pub fn set_from(&mut self, from: Vec2) {
        self.from = from;
        self.calculate_position_rotation_scale();
    }

    pub fn get_to(&self) -> Vec2 {
        self.to
    }

    pub fn set_to(&mut self, to: Vec2) {
        self.to = to;
        self.calculate_position_rotation_scale();
    }

    pub fn get_thickness(&self) -> f32 {
        self.thickness
    }

    pub fn set_thickness(&mut self, thickness: f32) {
        self.thickness = thickness;
        self.calculate_position_rotation_scale();
    }

    fn calculate_position_rotation_scale(&mut self) {
        self.position = self.from;
        self.rotation = Vec2::new(0.0, 1.0).signed_angle(self.to - self.from);
        self.size = Vec2::new(self.thickness, self.from.distance(self.to) + 1.0);
    }
}

impl Drawable for Line {
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

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn set_size(&mut self, size: Vec2) {
        self.size = size;
    }

    fn get_anchor(&self) -> Vec2 {
        panic!("Line doesn't support anchor property");
    }

    fn set_anchor(&mut self, _anchor: Vec2) {
        panic!("Line doesn't support anchor property");
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

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

    fn draw(&self, shader: &Shader) -> Result<(), String> {
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl StorageItem for Line {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_drawable(&self) -> Option<&dyn Drawable> {
        Some(self)
    }

    fn as_drawable_mut(&mut self) -> Option<&mut dyn Drawable> {
        Some(self)
    }
}
