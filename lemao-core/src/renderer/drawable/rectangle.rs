use super::*;
use crate::renderer::context::RendererContext;
use crate::renderer::shapes::Shape;
use crate::renderer::textures::Texture;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::ffi::c_void;
use std::rc::Rc;
use std::{mem, ptr};

pub struct Rectangle {
    pub id: usize,
    pub(crate) shape_id: usize,
    pub(crate) shape_vao_gl_id: u32,
    pub(crate) vao_gl_id: u32,
    pub(crate) vbo_gl_id: u32,
    pub(crate) ebo_gl_id: u32,
    pub(crate) texture_id: usize,
    pub(crate) texture_gl_id: u32,
    gl: Rc<OpenGLPointers>,

    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
    pub size: Vec2,
    pub anchor: Vec2,
    pub color: Color,
    pub corner_rounding: CornerRounding,
    pub custom_shape: bool,
    custom_shape_initialized: bool,
    elements_count: u32,
    vertices: Vec<f32>,
    indices: Vec<u32>,
}

impl Rectangle {
    pub fn new(renderer: &RendererContext, shape: &Shape, texture: &Texture) -> Self {
        Rectangle {
            id: 0,
            shape_id: shape.id,
            shape_vao_gl_id: shape.vao_gl_id,
            vao_gl_id: 0,
            vbo_gl_id: 0,
            ebo_gl_id: 0,
            texture_id: texture.id,
            texture_gl_id: texture.texture_gl_id,
            gl: renderer.gl.clone(),

            position: Default::default(),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
            size: Default::default(),
            anchor: Default::default(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            corner_rounding: Default::default(),
            custom_shape: false,
            custom_shape_initialized: false,
            elements_count: 0,
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.texture_id = texture.id;
        self.texture_gl_id = texture.texture_gl_id;
    }

    pub fn get_transformation_matrix(&self) -> Mat4x4 {
        let translation = Mat4x4::translate(Vec3::from(self.position));
        let rotation = Mat4x4::rotate(self.rotation);

        if self.custom_shape {
            let anchor_offset = Mat4x4::translate(-Vec3::from(self.anchor * self.size));
            let scale = Mat4x4::scale(Vec3::from(self.scale).floor());
            translation * rotation * scale * anchor_offset
        } else {
            let anchor_offset = Mat4x4::translate(-Vec3::from(self.anchor));
            let scale = Mat4x4::scale(Vec3::from(self.scale * self.size).floor());
            translation * rotation * scale * anchor_offset
        }
    }

    pub fn get_batch(&self) -> Batch {
        if self.custom_shape {
            Batch::new(None, Some(&self.vertices), Some(&self.indices), Some(self.texture_gl_id), Some(&self.color))
        } else {
            Batch::new(Some(self.shape_id), None, None, Some(self.texture_gl_id), Some(&self.color))
        }
    }

    pub fn update(&mut self) {
        unsafe {
            self.custom_shape = self.corner_rounding != Default::default();

            if self.custom_shape && !self.custom_shape_initialized {
                (self.gl.glGenVertexArrays)(1, &mut self.vao_gl_id);
                (self.gl.glBindVertexArray)(self.vao_gl_id);

                (self.gl.glGenBuffers)(1, &mut self.vbo_gl_id);
                (self.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, self.vbo_gl_id);

                (self.gl.glGenBuffers)(1, &mut self.ebo_gl_id);
                (self.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, self.ebo_gl_id);

                let attrib_size = (9 * mem::size_of::<f32>()) as i32;
                (self.gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, ptr::null_mut());
                (self.gl.glVertexAttribPointer)(1, 4, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (3 * mem::size_of::<f32>()) as *const c_void);
                (self.gl.glVertexAttribPointer)(2, 2, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (7 * mem::size_of::<f32>()) as *const c_void);

                (self.gl.glEnableVertexAttribArray)(0);
                (self.gl.glEnableVertexAttribArray)(1);
                (self.gl.glEnableVertexAttribArray)(2);

                self.custom_shape_initialized = true;
            }

            if self.custom_shape {
                self.vertices.clear();
                self.indices.clear();

                // Center
                self.vertices.extend_from_slice(&self.get_vertices(self.size / 2.0, Vec2::new(0.5, 0.5), SolidColor::new(1.0, 1.0, 1.0, 1.0)));

                // Left-bottom
                self.vertices.extend_from_slice(&self.get_vertices(
                    Vec2::new(self.corner_rounding.left_bottom, 0.0),
                    Vec2::new(self.corner_rounding.left_bottom, 0.0) / self.size,
                    SolidColor::new(1.0, 1.0, 1.0, 1.0),
                ));

                // Right-bottom
                self.vertices.extend_from_slice(&self.get_vertices(
                    Vec2::new(self.size.x - self.corner_rounding.right_bottom, 0.0),
                    Vec2::new(self.size.x - self.corner_rounding.right_bottom, 0.0) / self.size,
                    SolidColor::new(1.0, 1.0, 1.0, 1.0),
                ));

                // Right-bottom
                self.vertices.extend_from_slice(&self.get_vertices(
                    Vec2::new(self.size.x, self.corner_rounding.right_bottom),
                    Vec2::new(self.size.x, self.corner_rounding.right_bottom) / self.size,
                    SolidColor::new(1.0, 1.0, 1.0, 1.0),
                ));

                // Right-top
                self.vertices.extend_from_slice(&self.get_vertices(
                    Vec2::new(self.size.x, self.size.y - self.corner_rounding.right_top),
                    Vec2::new(self.size.x, self.size.y - self.corner_rounding.right_top) / self.size,
                    SolidColor::new(1.0, 1.0, 1.0, 1.0),
                ));

                // Right-top
                self.vertices.extend_from_slice(&self.get_vertices(
                    Vec2::new(self.size.x - self.corner_rounding.right_top, self.size.y),
                    Vec2::new(self.size.x - self.corner_rounding.right_top, self.size.y) / self.size,
                    SolidColor::new(1.0, 1.0, 1.0, 1.0),
                ));

                // Left-top
                self.vertices.extend_from_slice(&self.get_vertices(
                    Vec2::new(self.corner_rounding.left_top, self.size.y),
                    Vec2::new(self.corner_rounding.left_top, self.size.y) / self.size,
                    SolidColor::new(1.0, 1.0, 1.0, 1.0),
                ));

                // Left-top
                self.vertices.extend_from_slice(&self.get_vertices(
                    Vec2::new(0.0, self.size.y - self.corner_rounding.left_top),
                    Vec2::new(0.0, self.size.y - self.corner_rounding.left_top) / self.size,
                    SolidColor::new(1.0, 1.0, 1.0, 1.0),
                ));

                // Left-bottom
                self.vertices.extend_from_slice(&self.get_vertices(
                    Vec2::new(0.0, self.corner_rounding.left_bottom),
                    Vec2::new(0.0, self.corner_rounding.left_bottom) / self.size,
                    SolidColor::new(1.0, 1.0, 1.0, 1.0),
                ));

                self.indices.extend_from_slice(&[0, 1, 2, 0, 3, 4, 0, 5, 6, 0, 7, 8]);

                if self.corner_rounding.left_bottom > 0.0 {
                    self.get_corner(
                        Vec2::new(self.corner_rounding.left_bottom, self.corner_rounding.left_bottom),
                        self.corner_rounding.left_bottom,
                        std::f32::consts::PI * 1.0,
                        std::f32::consts::PI * 1.5,
                    );
                }

                if self.corner_rounding.right_bottom > 0.0 {
                    self.get_corner(
                        Vec2::new(self.size.x - self.corner_rounding.right_bottom, self.corner_rounding.right_bottom),
                        self.corner_rounding.right_bottom,
                        std::f32::consts::PI * 1.5,
                        std::f32::consts::PI * 2.0,
                    );
                }

                if self.corner_rounding.right_top > 0.0 {
                    self.get_corner(
                        Vec2::new(self.size.x - self.corner_rounding.right_top, self.size.y - self.corner_rounding.right_top),
                        self.corner_rounding.right_top,
                        std::f32::consts::PI * 0.0,
                        std::f32::consts::PI * 0.5,
                    );
                }

                if self.corner_rounding.left_top > 0.0 {
                    self.get_corner(
                        Vec2::new(self.corner_rounding.left_top, self.size.y - self.corner_rounding.left_top),
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
    }

    pub fn draw(&mut self, shader: &Shader) -> Result<(), String> {
        unsafe {
            let model = self.get_transformation_matrix();

            shader.set_parameter("model", model.as_ptr())?;
            shader.set_color(&self.color)?;

            if self.custom_shape {
                (self.gl.glBindVertexArray)(self.vao_gl_id);
                (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_gl_id);
                (self.gl.glDrawElements)(opengl::GL_TRIANGLES, self.elements_count as i32, opengl::GL_UNSIGNED_INT, ptr::null());
            } else {
                (self.gl.glBindVertexArray)(self.shape_vao_gl_id);
                (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_gl_id);
                (self.gl.glDrawElements)(opengl::GL_TRIANGLES, 6, opengl::GL_UNSIGNED_INT, ptr::null());
            }

            Ok(())
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

    fn get_corner(&mut self, center: Vec2, corner_rounding: f32, from_angle: f32, to_angle: f32) {
        let mut angle = from_angle;
        let step = (to_angle - from_angle) / corner_rounding;
        let base_indice = self.indices[self.indices.len() - 1] + 1;

        for n in 0..=(corner_rounding as u32) {
            let (x, y) = (angle.cos(), angle.sin());

            let position = Vec2::new(x, y);
            let scaled_position = position * corner_rounding + center;
            let uv = scaled_position / self.size;

            self.vertices.extend_from_slice(&self.get_vertices(scaled_position, uv, SolidColor::new(1.0, 1.0, 1.0, 1.0)));

            if n > 0 {
                self.indices.extend_from_slice(&[0, base_indice + n - 1, base_indice + n]);
            }

            angle += step;

            if angle > to_angle + step / 2.0 {
                break;
            }
        }
    }
}

impl Drop for Rectangle {
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
