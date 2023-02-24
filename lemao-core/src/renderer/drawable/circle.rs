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
use std::ptr;
use std::rc::Rc;

pub struct Circle {
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
    sides: u32,
    start_angle: f32,
    end_angle: f32,
    thickness: Vec2,
    squircle_factor: f32,
    elements_count: u32,
    vertices: Vec<f32>,
    indices: Vec<u32>,
    dirty: bool,
}

impl Circle {
    pub fn new(renderer: &RendererContext, texture: &Texture, radius: f32, sides: u32) -> Self {
        let mut circle = Circle {
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
            size: Vec2::new(radius * 2.0, radius * 2.0),
            anchor: Default::default(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            sides,
            start_angle: 0.0,
            end_angle: 2.0 * std::f32::consts::PI,
            thickness: Vec2::new(1.0, 1.0),
            squircle_factor: 0.0,
            elements_count: 0,
            vertices: Vec::new(),
            indices: Vec::new(),
            dirty: true,
        };

        unsafe {
            (circle.gl.glGenVertexArrays)(1, &mut circle.vao_gl_id);
            (circle.gl.glBindVertexArray)(circle.vao_gl_id);

            (circle.gl.glGenBuffers)(1, &mut circle.vbo_gl_id);
            (circle.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, circle.vbo_gl_id);

            (circle.gl.glGenBuffers)(1, &mut circle.ebo_gl_id);
            (circle.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, circle.ebo_gl_id);

            let attrib_size = (9 * mem::size_of::<f32>()) as i32;
            (circle.gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, ptr::null_mut());
            (circle.gl.glVertexAttribPointer)(1, 4, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (3 * mem::size_of::<f32>()) as *const c_void);
            (circle.gl.glVertexAttribPointer)(2, 2, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (7 * mem::size_of::<f32>()) as *const c_void);

            (circle.gl.glEnableVertexAttribArray)(0);
            (circle.gl.glEnableVertexAttribArray)(1);
            (circle.gl.glEnableVertexAttribArray)(2);
        }

        circle
    }

    pub fn get_texture_id(&self) -> usize {
        self.texture_id
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.texture_id = texture.id;
        self.texture_gl_id = texture.texture_gl_id;
    }

    pub fn get_sides(&self) -> u32 {
        self.sides
    }

    pub fn set_sides(&mut self, sides: u32) {
        self.sides = sides;
        self.dirty = true;
    }

    pub fn get_start_angle(&self) -> f32 {
        self.start_angle
    }

    pub fn set_start_angle(&mut self, angle: f32) {
        self.start_angle = angle;
        self.dirty = true;
    }

    pub fn get_end_angle(&self) -> f32 {
        self.end_angle
    }

    pub fn set_end_angle(&mut self, angle: f32) {
        self.end_angle = angle;
        self.dirty = true;
    }

    pub fn get_thickness(&self) -> Vec2 {
        self.thickness
    }

    pub fn set_thickness(&mut self, thickness: Vec2) {
        self.thickness = thickness;
        self.dirty = true;
    }

    pub fn get_squircle_factor(&self) -> f32 {
        self.squircle_factor
    }

    pub fn set_squircle_factor(&mut self, squircle_factor: f32) {
        self.squircle_factor = squircle_factor;
        self.dirty = true;
    }

    fn update(&mut self) {
        unsafe {
            let fixed_start_angle = self.start_angle + std::f32::consts::PI / 2.0;
            let fixed_end_angle = self.end_angle + std::f32::consts::PI / 2.0;

            let mut angle = fixed_start_angle;
            let step = 2.0 * std::f32::consts::PI / (self.sides as f32);
            let radius = self.size / 2.0;
            let mut last_fragment = false;

            self.vertices.clear();
            self.indices.clear();

            for n in 0..self.sides + 1 {
                let (x, y) = if self.squircle_factor == 0.0 || angle.sin().abs() < 0.00001 || angle.cos().abs() < 0.00001 {
                    (angle.cos(), angle.sin())
                } else {
                    // https://arxiv.org/vc/arxiv/papers/1604/1604.02174v1.pdf
                    (
                        (angle.cos().signum() * (1.0 - (1.0 - self.squircle_factor.powi(2) * (2.0 * angle).sin().powi(2)).sqrt()).sqrt())
                            / (self.squircle_factor * 2.0f32.sqrt() * angle.sin().abs()),
                        (angle.sin().signum() * (1.0 - (1.0 - self.squircle_factor.powi(2) * (2.0 * angle).sin().powi(2)).sqrt()).sqrt())
                            / (self.squircle_factor * 2.0f32.sqrt() * angle.cos().abs()),
                    )
                };

                let position = Vec2::new(x, y);
                let outer_position = position * radius;
                let inner_position = position * (radius - self.thickness);
                let outer_uv = outer_position / radius * Vec2::new(0.5, 0.5) + Vec2::new(0.5, 0.5);
                let inner_uv = inner_position / radius * Vec2::new(0.5, 0.5) + Vec2::new(0.5, 0.5);
                let outer_position = outer_position + radius;
                let inner_position = inner_position + radius;
                self.vertices.extend_from_slice(&self.get_vertices(outer_position, inner_position, outer_uv, inner_uv, SolidColor::new(1.0, 1.0, 1.0, 1.0)));

                if n > 0 {
                    self.indices.extend_from_slice(&[n * 2 - 2, n * 2 - 1, n * 2, n * 2 - 1, n * 2, n * 2 + 1]);
                }

                angle += step;

                // Ensure that the disc is always drawn to the end
                if angle >= fixed_end_angle - step / 2.0 {
                    if !last_fragment {
                        angle = fixed_end_angle;
                        last_fragment = true;
                        continue;
                    } else if last_fragment {
                        break;
                    }
                }
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

            self.dirty = false;
        }
    }

    #[rustfmt::skip]
    fn get_vertices(&self, outer_position: Vec2, inner_position: Vec2, outer_uv: Vec2, inner_uv: Vec2, color: SolidColor) -> [f32; 18] {
        [
            /* v.x */ outer_position.x,
            /* v.y */ outer_position.y,
            /* v.z */ 0.0,
            /* c.r */ color.r,
            /* c.g */ color.g,
            /* c.b */ color.b,
            /* c.a */ color.a,
            /* t.u */ outer_uv.x,
            /* t.v */ outer_uv.y,

            /* v.x */ inner_position.x,
            /* v.y */ inner_position.y,
            /* v.z */ 0.0,
            /* c.r */ color.r,
            /* c.g */ color.g,
            /* c.b */ color.b,
            /* c.a */ color.a,
            /* t.u */ inner_uv.x,
            /* t.v */ inner_uv.y,
        ]
    }
}

impl Drawable for Circle {
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
        self.dirty = true;
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

    fn draw(&mut self, shader: &Shader) -> Result<(), String> {
        if self.dirty {
            self.update();
        }

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

impl StorageItem for Circle {
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

impl Drop for Circle {
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
