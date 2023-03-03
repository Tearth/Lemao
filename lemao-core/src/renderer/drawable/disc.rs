use super::*;
use crate::renderer::context::RendererContext;
use crate::renderer::textures::Texture;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::rc::Rc;

pub struct Disc {
    pub id: usize,
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
    pub sides: u32,
    pub start_angle: f32,
    pub end_angle: f32,
    pub squircle_factor: f32,
    elements_count: u32,
    vertices: Vec<f32>,
    indices: Vec<u32>,
}

impl Disc {
    pub fn new(renderer: &RendererContext, texture: &Texture) -> Self {
        let mut disc = Disc {
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
            size: Vec2::new(100.0, 100.0),
            anchor: Default::default(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            sides: 256,
            start_angle: 0.0,
            end_angle: 2.0 * std::f32::consts::PI,
            squircle_factor: 0.0,
            elements_count: 0,
            vertices: Vec::new(),
            indices: Vec::new(),
        };

        unsafe {
            (disc.gl.glGenVertexArrays)(1, &mut disc.vao_gl_id);
            (disc.gl.glBindVertexArray)(disc.vao_gl_id);

            (disc.gl.glGenBuffers)(1, &mut disc.vbo_gl_id);
            (disc.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, disc.vbo_gl_id);

            (disc.gl.glGenBuffers)(1, &mut disc.ebo_gl_id);
            (disc.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, disc.ebo_gl_id);

            let attrib_size = (9 * mem::size_of::<f32>()) as i32;
            (disc.gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, ptr::null_mut());
            (disc.gl.glVertexAttribPointer)(1, 4, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (3 * mem::size_of::<f32>()) as *const c_void);
            (disc.gl.glVertexAttribPointer)(2, 2, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (7 * mem::size_of::<f32>()) as *const c_void);

            (disc.gl.glEnableVertexAttribArray)(0);
            (disc.gl.glEnableVertexAttribArray)(1);
            (disc.gl.glEnableVertexAttribArray)(2);
        }

        disc
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.texture_id = texture.id;
        self.texture_gl_id = texture.texture_gl_id;
    }

    pub fn get_transformation_matrix(&self) -> Mat4x4 {
        let translation = Mat4x4::translate(Vec3::from(self.position));
        let anchor_offset = Mat4x4::translate(-Vec3::from(self.anchor * self.size).floor());
        let scale = Mat4x4::scale(Vec3::from(self.scale));
        let rotation = Mat4x4::rotate(self.rotation);
        translation * rotation * scale * anchor_offset
    }

    pub fn get_batch(&self) -> Batch {
        Batch::new(None, Some(&self.vertices), Some(&self.indices), Some(self.texture_gl_id), Some(&self.color))
    }

    pub fn update(&mut self) {
        unsafe {
            let fixed_start_angle = self.start_angle + std::f32::consts::PI / 2.0;
            let fixed_end_angle = self.end_angle + std::f32::consts::PI / 2.0;

            let mut angle = fixed_start_angle;
            let step = 2.0 * std::f32::consts::PI / (self.sides as f32);
            let radius = self.size / 2.0;
            let mut last_fragment = false;

            self.vertices.clear();
            self.indices.clear();

            self.vertices.extend_from_slice(&self.get_vertices(radius, Vec2::new(0.5, 0.5), SolidColor::new(1.0, 1.0, 1.0, 1.0)));

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
                let scaled_position = position * radius + radius;
                let uv = position * Vec2::new(0.5, 0.5) + Vec2::new(0.5, 0.5);
                self.vertices.extend_from_slice(&self.get_vertices(scaled_position, uv, SolidColor::new(1.0, 1.0, 1.0, 1.0)));

                if n > 0 {
                    self.indices.extend_from_slice(&[0, n, n + 1]);
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
        }
    }

    pub fn draw(&mut self, shader: &Shader) -> Result<(), String> {
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
}

impl Drop for Disc {
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
