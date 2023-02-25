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

pub struct Tilemap {
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
    texture_size: Vec2,
    frames_count: Vec2,
    total_frames_count: u32,
    frame: u32,
    vertices: Vec<f32>,
    indices: Vec<u32>,
    dirty: bool,
}

impl Tilemap {
    pub fn new(renderer: &RendererContext, texture: &Texture, tile_size: Vec2) -> Self {
        let texture_size = texture.get_size();
        let mut tilemap = Tilemap {
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
            size: tile_size,
            anchor: Default::default(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            texture_size,
            frames_count: Vec2::new(texture_size.x / tile_size.x, texture_size.y / tile_size.y),
            total_frames_count: ((texture_size.x / tile_size.x) * (texture_size.y / tile_size.y)) as u32,
            frame: 0,
            vertices: Vec::new(),
            indices: Vec::new(),
            dirty: true,
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

    pub fn get_texture_id(&self) -> usize {
        self.texture_id
    }

    pub fn set_texture(&mut self, texture: &Texture, tile_size: Vec2) {
        let texture_size = texture.get_size();

        self.texture_id = texture.id;
        self.texture_gl_id = texture.texture_gl_id;
        self.size = texture.get_size();
        self.frames_count = Vec2::new(texture_size.x / tile_size.x, texture_size.y / tile_size.y);
        self.total_frames_count = ((texture_size.x / tile_size.x) * (texture_size.y / tile_size.y)) as u32;
        self.frame = 0;
    }

    pub fn get_frame(&self) -> u32 {
        self.frame
    }

    pub fn set_frame(&mut self, frame: u32) {
        self.frame = frame;
    }

    pub fn set_next_frame(&mut self) {
        self.frame = if self.frame + 1 >= self.total_frames_count { 0 } else { self.frame + 1 };
        self.dirty = true;
    }

    pub fn set_previous_frame(&mut self) {
        self.frame = if self.frame == 0 { self.total_frames_count - 1 } else { self.frame - 1 };
        self.dirty = true;
    }

    pub fn update(&mut self) {
        unsafe {
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

            self.dirty = false;
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

impl Drawable for Tilemap {
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
        let anchor_offset = Mat4x4::translate(-Vec3::from(self.anchor));
        let scale = Mat4x4::scale(Vec3::from(self.scale * self.size).floor());
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

impl StorageItem for Tilemap {
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
