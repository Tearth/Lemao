use super::*;
use crate::renderer::context::RendererContext;
use crate::renderer::textures::Texture;
use crate::utils::storage::StorageItem;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::any::Any;
use std::ffi::c_void;
use std::mem;
use std::ops::{Add, Sub};
use std::ptr;
use std::rc::Rc;

pub struct Frame {
    pub(crate) id: usize,
    pub(crate) vao_gl_id: u32,
    pub(crate) vbo_gl_id: u32,
    pub(crate) ebo_gl_id: u32,
    pub(crate) texture_id: usize,
    pub(crate) texture_gl_id: u32,
    gl: Rc<OpenGLPointers>,

    position: Vec2,
    scale: Vec2,
    rotation: f32,
    size: Vec2,
    anchor: Vec2,
    color: Color,
    thickness: FrameThickness,
    corner_rounding: CornerRounding,
    elements_count: u32,
    vertices: Vec<f32>,
    indices: Vec<u32>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct FrameThickness {
    pub top: f32,
    pub bottom: f32,
    pub right: f32,
    pub left: f32,
}

impl Frame {
    pub fn new(renderer: &RendererContext, texture: &Texture, size: Vec2) -> Self {
        let mut frame = Frame {
            id: 0,
            vao_gl_id: 0,
            vbo_gl_id: 0,
            ebo_gl_id: 0,
            texture_id: texture.id,
            texture_gl_id: texture.texture_gl_id,
            gl: renderer.gl.clone(),

            position: Default::default(),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
            size,
            anchor: Default::default(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            thickness: FrameThickness::new(1.0, 1.0, 1.0, 1.0),
            corner_rounding: Default::default(),
            elements_count: 0,
            vertices: Vec::new(),
            indices: Vec::new(),
        };

        unsafe {
            (frame.gl.glGenVertexArrays)(1, &mut frame.vao_gl_id);
            (frame.gl.glBindVertexArray)(frame.vao_gl_id);

            (frame.gl.glGenBuffers)(1, &mut frame.vbo_gl_id);
            (frame.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, frame.vbo_gl_id);

            (frame.gl.glGenBuffers)(1, &mut frame.ebo_gl_id);
            (frame.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, frame.ebo_gl_id);

            let attrib_size = (9 * mem::size_of::<f32>()) as i32;
            (frame.gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, ptr::null_mut());
            (frame.gl.glVertexAttribPointer)(1, 4, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (3 * mem::size_of::<f32>()) as *const c_void);
            (frame.gl.glVertexAttribPointer)(2, 2, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (7 * mem::size_of::<f32>()) as *const c_void);

            (frame.gl.glEnableVertexAttribArray)(0);
            (frame.gl.glEnableVertexAttribArray)(1);
            (frame.gl.glEnableVertexAttribArray)(2);
        }

        frame.update();
        frame
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_texture_id(&self) -> usize {
        self.texture_id
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.texture_id = texture.id;
        self.texture_gl_id = texture.texture_gl_id;
    }

    pub fn get_thickness(&self) -> FrameThickness {
        self.thickness
    }

    pub fn set_thickness(&mut self, thickness: FrameThickness) {
        self.thickness = thickness;
        self.update();
    }

    pub fn get_corner_rounding(&self) -> CornerRounding {
        self.corner_rounding
    }

    pub fn set_corner_rounding(&mut self, corner_rounding: CornerRounding) {
        self.corner_rounding = corner_rounding;
        self.update();
    }

    fn update(&mut self) {
        unsafe {
            self.vertices.clear();
            self.indices.clear();

            // Left-bottom
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.corner_rounding.left_bottom, 0.0),
                Vec2::new(self.corner_rounding.left_bottom, 0.0) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.thickness.left + self.corner_rounding.left_bottom, self.thickness.bottom),
                Vec2::new(self.thickness.left + self.corner_rounding.left_bottom, self.thickness.bottom) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));

            // Right-bottom
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.size.x - self.corner_rounding.right_bottom, 0.0),
                Vec2::new(self.size.x - self.corner_rounding.right_bottom, 0.0) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.size.x - self.thickness.right - self.corner_rounding.right_bottom, self.thickness.bottom),
                Vec2::new(self.size.x - self.thickness.right - self.corner_rounding.right_bottom, self.thickness.bottom) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));

            // Right-bottom
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.size.x, self.corner_rounding.right_bottom),
                Vec2::new(self.size.x, self.corner_rounding.right_bottom) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.size.x - self.thickness.right, self.thickness.bottom + self.corner_rounding.right_bottom),
                Vec2::new(self.size.x - self.thickness.right, self.thickness.bottom + self.corner_rounding.right_bottom) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));

            // Right-top
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.size.x, self.size.y - self.corner_rounding.right_top),
                Vec2::new(self.size.x, self.size.y - self.corner_rounding.right_top) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.size.x - self.thickness.right, self.size.y - self.thickness.top - self.corner_rounding.right_bottom),
                Vec2::new(self.size.x - self.thickness.right, self.size.y - self.thickness.top - self.corner_rounding.right_bottom) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));

            // Right-top
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.size.x - self.corner_rounding.right_top, self.size.y),
                Vec2::new(self.size.x - self.corner_rounding.right_top, self.size.y) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.size.x - self.thickness.right - self.corner_rounding.right_top, self.size.y - self.thickness.top),
                Vec2::new(self.size.x - self.thickness.right - self.corner_rounding.right_top, self.size.y - self.thickness.top) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));

            // Left-top
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.corner_rounding.left_top, self.size.y),
                Vec2::new(self.corner_rounding.left_top, self.size.y) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.thickness.left + self.corner_rounding.left_top, self.size.y - self.thickness.top),
                Vec2::new(self.thickness.left + self.corner_rounding.left_top, self.size.y - self.thickness.top) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));

            // Left-top
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(0.0, self.size.y - self.corner_rounding.left_top),
                Vec2::new(0.0, self.size.y - self.corner_rounding.left_top) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.thickness.left, self.size.y - self.thickness.top - self.corner_rounding.left_top),
                Vec2::new(self.thickness.left, self.size.y - self.thickness.top - self.corner_rounding.left_top) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));

            // Left-bottom
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(0.0, self.corner_rounding.left_bottom),
                Vec2::new(0.0, self.corner_rounding.left_bottom) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));
            self.vertices.extend_from_slice(&self.get_vertices(
                Vec2::new(self.thickness.left, self.thickness.bottom + self.corner_rounding.left_bottom),
                Vec2::new(self.thickness.left, self.thickness.bottom + self.corner_rounding.left_bottom) / self.size,
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ));

            self.indices.extend_from_slice(&[0, 1, 2, 1, 2, 3, 4, 5, 6, 5, 6, 7, 8, 9, 10, 9, 10, 11, 12, 13, 14, 13, 14, 15]);

            if self.corner_rounding.left_bottom > 0.0 {
                self.get_corner(
                    Vec2::new(self.corner_rounding.left_bottom, self.corner_rounding.left_bottom),
                    Vec2::new(self.thickness.left + self.corner_rounding.left_bottom, self.thickness.bottom + self.corner_rounding.left_bottom),
                    self.corner_rounding.left_bottom,
                    std::f32::consts::PI * 1.0,
                    std::f32::consts::PI * 1.5,
                );
            }

            if self.corner_rounding.right_bottom > 0.0 {
                self.get_corner(
                    Vec2::new(self.size.x - self.corner_rounding.right_bottom, self.corner_rounding.right_bottom),
                    Vec2::new(
                        self.size.x - self.thickness.right - self.corner_rounding.right_bottom,
                        self.thickness.bottom + self.corner_rounding.right_bottom,
                    ),
                    self.corner_rounding.right_bottom,
                    std::f32::consts::PI * 1.5,
                    std::f32::consts::PI * 2.0,
                );
            }

            if self.corner_rounding.right_top > 0.0 {
                self.get_corner(
                    Vec2::new(self.size.x - self.corner_rounding.right_top, self.size.y - self.corner_rounding.right_top),
                    Vec2::new(
                        self.size.x - self.thickness.right - self.corner_rounding.right_top,
                        self.size.y - self.thickness.top - self.corner_rounding.right_top,
                    ),
                    self.corner_rounding.right_top,
                    std::f32::consts::PI * 0.0,
                    std::f32::consts::PI * 0.5,
                );
            }

            if self.corner_rounding.left_top > 0.0 {
                self.get_corner(
                    Vec2::new(self.corner_rounding.left_top, self.size.y - self.corner_rounding.left_top),
                    Vec2::new(self.thickness.left + self.corner_rounding.left_top, self.size.y - self.thickness.top - self.corner_rounding.left_top),
                    self.corner_rounding.left_top,
                    std::f32::consts::PI * 0.5,
                    std::f32::consts::PI * 1.0,
                );
            }

            self.elements_count = self.indices.len() as u32;

            let vertices_size = (mem::size_of::<f32>() * self.vertices.len()) as i64;
            let vertices_ptr = self.vertices.as_ptr() as *const c_void;

            (self.gl.glBindVertexArray)(self.vao_gl_id);
            (self.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, self.vbo_gl_id);
            (self.gl.glBufferData)(opengl::GL_ARRAY_BUFFER, vertices_size, vertices_ptr, opengl::GL_STATIC_DRAW);

            let indices_size = (mem::size_of::<u32>() * self.indices.len()) as i64;
            let indices_ptr = self.indices.as_ptr() as *const c_void;

            (self.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, self.ebo_gl_id);
            (self.gl.glBufferData)(opengl::GL_ELEMENT_ARRAY_BUFFER, indices_size, indices_ptr, opengl::GL_STATIC_DRAW);
        }
    }

    #[rustfmt::skip]
    fn get_vertices(&self, position: Vec2, uv: Vec2, color: SolidColor) -> [f32; 9] {
        [
            /* v.x */ position.x,
            /* v.y */ position.y,
            /* v.z */ 0.0,
            /* c.r */ color.r,
            /* c.g */ color.g,
            /* c.b */ color.b,
            /* c.a */ color.a,
            /* t.u */ uv.x,
            /* t.v */ uv.y,
        ]
    }

    fn get_corner(&mut self, outer_center: Vec2, inner_center: Vec2, corner_rounding: f32, from_angle: f32, to_angle: f32) {
        let mut angle = from_angle;
        let step = (to_angle - from_angle) / corner_rounding;
        let base_indice = self.indices[self.indices.len() - 1] + 1;

        for n in 0..=(corner_rounding as u32) {
            let (x, y) = (angle.cos(), angle.sin());

            let position = Vec2::new(x, y);
            let outer_scaled_position = position * corner_rounding + outer_center;
            let inner_scaled_position = position * corner_rounding + inner_center;
            let outer_uv = outer_scaled_position / self.size;
            let inner_uv = inner_scaled_position / self.size;

            self.vertices.extend_from_slice(&self.get_vertices(outer_scaled_position, outer_uv, SolidColor::new(1.0, 1.0, 1.0, 1.0)));
            self.vertices.extend_from_slice(&self.get_vertices(inner_scaled_position, inner_uv, SolidColor::new(1.0, 1.0, 1.0, 1.0)));

            if n > 0 {
                self.indices.extend_from_slice(&[
                    base_indice + n * 2 - 2,
                    base_indice + n * 2 - 1,
                    base_indice + n * 2,
                    base_indice + n * 2 - 1,
                    base_indice + n * 2,
                    base_indice + n * 2 + 1,
                ]);
            }

            angle += step;

            if angle > to_angle + step / 2.0 {
                break;
            }
        }
    }
}

impl Drawable for Frame {
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
        self.update();
    }

    fn get_anchor(&self) -> Vec2 {
        self.anchor
    }

    fn set_anchor(&mut self, anchor: Vec2) {
        self.anchor = anchor;
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    fn get_transformation_matrix(&self) -> Mat4x4 {
        let translation = Mat4x4::translate(Vec3::from(self.position));
        let anchor_offset = Mat4x4::translate(-Vec3::from(self.anchor * self.size).floor());
        let scale = Mat4x4::scale(Vec3::from(self.scale));
        let rotation = Mat4x4::rotate(self.rotation);
        translation * rotation * scale * anchor_offset
    }

    fn get_batch(&self) -> Batch {
        Batch::new(None, Some(&self.vertices), Some(&self.indices), Some(self.texture_gl_id), Some(&self.color))
    }

    fn draw(&self, shader: &Shader) -> Result<(), String> {
        unsafe {
            let model = self.get_transformation_matrix();

            shader.set_parameter("model", model.as_ptr())?;
            shader.set_color(&self.color)?;

            (self.gl.glBindVertexArray)(self.vao_gl_id);
            (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_gl_id);
            (self.gl.glDrawElements)(opengl::GL_TRIANGLES, self.elements_count as i32, opengl::GL_UNSIGNED_INT, ptr::null());

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

impl StorageItem for Frame {
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

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            if self.vbo_gl_id != 0 {
                (self.gl.glDeleteBuffers)(1, &mut self.vbo_gl_id);
            }

            if self.ebo_gl_id != 0 {
                (self.gl.glDeleteBuffers)(1, &mut self.ebo_gl_id);
            }

            if self.vao_gl_id != 0 {
                (self.gl.glDeleteVertexArrays)(1, &mut self.vao_gl_id);
            }
        }
    }
}

impl FrameThickness {
    pub fn new(top: f32, bottom: f32, right: f32, left: f32) -> Self {
        Self { top, bottom, right, left }
    }
}

impl Add for FrameThickness {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self { top: self.top + other.top, bottom: self.bottom + other.bottom, right: self.right + other.right, left: self.left + other.left }
    }
}

impl Sub for FrameThickness {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { top: self.top - other.top, bottom: self.bottom - other.bottom, right: self.right - other.right, left: self.left - other.left }
    }
}
