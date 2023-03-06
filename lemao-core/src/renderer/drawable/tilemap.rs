use super::*;
use crate::renderer::context::RendererContext;
use crate::renderer::textures::Texture;
use crate::utils::storage::StorageItem;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::rc::Rc;

pub struct Tilemap {
    pub id: usize,
    pub name: Option<String>,
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
    pub texture_size: Vec2,
    pub frames_count: Vec2,
    pub total_frames_count: u32,
    pub frame: u32,
    vertices: Vec<f32>,
    indices: Vec<u32>,
}

impl Tilemap {
    pub fn new(renderer: &RendererContext, texture: &Texture) -> Self {
        let texture_size = texture.size;
        let mut tilemap = Tilemap {
            id: 0,
            name: None,
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
            texture_size,
            frames_count: Default::default(),
            total_frames_count: 0,
            frame: 0,
            vertices: Vec::new(),
            indices: Vec::new(),
        };

        unsafe {
            (tilemap.gl.glGenVertexArrays)(1, &mut tilemap.vao_gl_id);
            (tilemap.gl.glBindVertexArray)(tilemap.vao_gl_id);

            (tilemap.gl.glGenBuffers)(1, &mut tilemap.vbo_gl_id);
            (tilemap.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, tilemap.vbo_gl_id);

            (tilemap.gl.glGenBuffers)(1, &mut tilemap.ebo_gl_id);
            (tilemap.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, tilemap.ebo_gl_id);

            let attrib_size = (9 * mem::size_of::<f32>()) as i32;
            (tilemap.gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, ptr::null_mut());
            (tilemap.gl.glVertexAttribPointer)(1, 4, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (3 * mem::size_of::<f32>()) as *const c_void);
            (tilemap.gl.glVertexAttribPointer)(2, 2, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (7 * mem::size_of::<f32>()) as *const c_void);

            (tilemap.gl.glEnableVertexAttribArray)(0);
            (tilemap.gl.glEnableVertexAttribArray)(1);
            (tilemap.gl.glEnableVertexAttribArray)(2);
        }

        tilemap
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.texture_id = texture.id;
        self.texture_gl_id = texture.texture_gl_id;
    }

    pub fn get_transformation_matrix(&self) -> Mat4x4 {
        let translation = Mat4x4::translate(Vec3::from(self.position));
        let anchor_offset = Mat4x4::translate(-Vec3::from(self.anchor));
        let scale = Mat4x4::scale(Vec3::from(self.scale * self.size).floor());
        let rotation = Mat4x4::rotate(self.rotation);
        translation * rotation * scale * anchor_offset
    }

    pub fn set_next_frame(&mut self) {
        self.frame = if self.frame + 1 >= self.total_frames_count { 0 } else { self.frame + 1 };
    }

    pub fn set_previous_frame(&mut self) {
        self.frame = if self.frame == 0 { self.total_frames_count - 1 } else { self.frame - 1 };
    }

    pub fn get_batch(&self) -> Batch {
        Batch::new(None, Some(&self.vertices), Some(&self.indices), Some(self.texture_gl_id), Some(&self.color))
    }

    pub fn update(&mut self) {
        unsafe {
            self.frames_count = Vec2::new(self.texture_size.x / self.size.x, self.texture_size.y / self.size.y);
            self.total_frames_count = (self.frames_count.x * self.frames_count.y) as u32;
            self.frame = self.frame.clamp(0, self.total_frames_count - 1);

            self.vertices.clear();
            self.indices.clear();

            let uv_width = self.size.x / self.texture_size.x;
            let uv_height = self.size.y / self.texture_size.y;
            let uv_size = Vec2::new(uv_width, uv_height);

            let row = self.frame % (self.frames_count.x as u32);
            let col = self.frame / (self.frames_count.y as u32);
            let uv = Vec2::new(row as f32 * uv_width, 1.0 - col as f32 * uv_height - uv_size.y);

            self.vertices.extend_from_slice(&self.get_vertices(uv, uv_size, SolidColor::new(1.0, 1.0, 1.0, 1.0)));
            self.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);

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
            (self.gl.glDrawElements)(opengl::GL_TRIANGLES, 6, opengl::GL_UNSIGNED_INT, ptr::null());

            Ok(())
        }
    }

    #[rustfmt::skip]
    fn get_vertices(&self, uv: Vec2, uv_size: Vec2, color: SolidColor) -> [f32; 36] {
        [
            // Left-bottom
            /* v.x */ 0.0,
            /* v.y */ 0.0,
            /* v.z */ 0.0,
            /* c.r */ color.r,
            /* c.g */ color.g,
            /* c.b */ color.b,
            /* c.a */ color.a,
            /* t.u */ uv.x,
            /* t.v */ uv.y,
            // Right-bottom
            /* v.x */ 1.0,
            /* v.y */ 0.0,
            /* v.z */ 0.0,
            /* c.r */ color.r,
            /* c.g */ color.g,
            /* c.b */ color.b,
            /* c.a */ color.a,
            /* t.u */ uv.x + uv_size.x,
            /* t.v */ uv.y,
            // Right-top
            /* v.x */ 1.0,
            /* v.y */ 1.0,
            /* v.z */ 0.0,
            /* c.r */ color.r,
            /* c.g */ color.g,
            /* c.b */ color.b,
            /* c.a */ color.a,
            /* t.u */ uv.x + uv_size.x,
            /* t.v */ uv.y + uv_size.y,
            // Left-top
            /* v.x */ 0.0,
            /* v.y */ 1.0,
            /* v.z */ 0.0,
            /* c.r */ color.r,
            /* c.g */ color.g,
            /* c.b */ color.b,
            /* c.a */ color.a,
            /* t.u */ uv.x,
            /* t.v */ uv.y + uv_size.y,
        ]
    }
}

impl StorageItem for Tilemap {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn get_name(&self) -> Option<String> {
        self.name.clone()
    }

    fn set_name(&mut self, name: Option<String>) {
        self.name = name;
    }
}

impl Drop for Tilemap {
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
