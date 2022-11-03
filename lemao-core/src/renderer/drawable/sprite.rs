use super::*;
use crate::renderer::textures::Texture;
use crate::renderer::textures::TextureFormat;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::ffi::c_void;
use std::mem;
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

    vao_gl_id: u32,
    vbo_gl_id: u32,
    ebo_gl_id: u32,
    texture_gl_id: u32,

    texture_id: usize,
    gl: Rc<OpenGLPointers>,
}

impl Sprite {
    pub fn new(gl: Rc<OpenGLPointers>, texture: &Texture) -> Self {
        let mut sprite = Sprite {
            id: 0,

            position: Default::default(),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,

            width: 0,
            height: 0,
            anchor: Default::default(),
            texture_id: texture.id,

            vao_gl_id: 0,
            vbo_gl_id: 0,
            ebo_gl_id: 0,
            texture_gl_id: 0,

            gl,
        };

        sprite.set_texture(texture);
        sprite
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        unsafe {
            if self.vao_gl_id == 0 {
                (self.gl.glGenVertexArrays)(1, &mut self.vao_gl_id);
            }
            (self.gl.glBindVertexArray)(self.vao_gl_id);

            if self.vbo_gl_id == 0 {
                (self.gl.glGenBuffers)(1, &mut self.vbo_gl_id);
            }

            let vertices: [f32; 20] = self.get_vertices(Vec2::new(0.5, 0.5), texture.width, texture.height);
            let vertices_size = (mem::size_of::<f32>() * vertices.len()) as i64;
            let vertices_ptr = vertices.as_ptr() as *const c_void;

            (self.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, self.vbo_gl_id);
            (self.gl.glBufferData)(opengl::GL_ARRAY_BUFFER, vertices_size, vertices_ptr, opengl::GL_STATIC_DRAW);

            if self.ebo_gl_id == 0 {
                (self.gl.glGenBuffers)(1, &mut self.ebo_gl_id);
            }

            let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];
            let indices_size = (mem::size_of::<u32>() * indices.len()) as i64;
            let indices_ptr = indices.as_ptr() as *const c_void;

            (self.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, self.ebo_gl_id);
            (self.gl.glBufferData)(opengl::GL_ELEMENT_ARRAY_BUFFER, indices_size, indices_ptr, opengl::GL_STATIC_DRAW);

            let attrib_size = (5 * mem::size_of::<f32>()) as i32;
            (self.gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size as i32, ptr::null_mut());
            (self.gl.glVertexAttribPointer)(1, 2, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (3 * mem::size_of::<f32>()) as *const c_void);

            (self.gl.glEnableVertexAttribArray)(0);
            (self.gl.glEnableVertexAttribArray)(1);

            if self.texture_gl_id != 0 {
                (self.gl.glDeleteTextures)(1, &self.texture_gl_id);
            }

            (self.gl.glGenTextures)(1, &mut self.texture_gl_id);
            (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_gl_id);
            (self.gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_WRAP_S, opengl::GL_MIRRORED_REPEAT as i32);
            (self.gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_WRAP_T, opengl::GL_MIRRORED_REPEAT as i32);
            (self.gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_MIN_FILTER, opengl::GL_NEAREST as i32);
            (self.gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_MAG_FILTER, opengl::GL_LINEAR as i32);

            let format = if texture.format == TextureFormat::RGB { opengl::GL_RGB } else { opengl::GL_RGBA };
            let texture_width = texture.width as i32;
            let texture_height = texture.height as i32;
            let texture_ptr = texture.data.as_ptr() as *const c_void;

            (self.gl.glTexImage2D)(opengl::GL_TEXTURE_2D, 0, format as i32, texture_width, texture_height, 0, format, opengl::GL_UNSIGNED_BYTE, texture_ptr);
            (self.gl.glGenerateMipmap)(opengl::GL_TEXTURE_2D);

            self.width = texture_width as u32;
            self.height = texture_height as u32;
            self.texture_id = texture.id;
        }
    }

    pub fn get_anchor(&self) -> Vec2 {
        self.anchor
    }

    pub fn set_anchor(&mut self, anchor: Vec2) {
        unsafe {
            if self.vbo_gl_id == 0 {
                return;
            }

            let vertices = self.get_vertices(anchor, self.width, self.height);
            let vertices_size = (mem::size_of::<f32>() * vertices.len()) as i64;

            (self.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, self.vbo_gl_id);
            (self.gl.glBufferData)(opengl::GL_ARRAY_BUFFER, vertices_size, vertices.as_ptr() as *const c_void, opengl::GL_STATIC_DRAW);

            self.anchor = anchor;
        }
    }

    fn get_vertices(&self, anchor: Vec2, width: u32, height: u32) -> [f32; 20] {
        let offset = anchor * Vec2::new(width as f32, height as f32);
        [
            // Left-bottom
            /* v.x */ 0.0 - offset.x,
            /* v.y */ 0.0 - offset.y,
            /* v.z */ 0.0,
            /* t.u */ 0.0,
            /* t.v */ 0.0,
            // Right-bottom
            /* v.x */ (width as f32) - offset.x,
            /* v.y */ 0.0 - offset.y,
            /* v.z */ 0.0,
            /* t.u */ 1.0,
            /* t.v */ 0.0,
            // Right-top
            /* v.x */ (width as f32) - offset.x,
            /* v.y */ (height as f32) - offset.y,
            /* v.z */ 0.0,
            /* t.u */ 1.0,
            /* t.v */ 1.0,
            // Left-top
            /* v.x */ 0.0 - offset.x,
            /* v.y */ (height as f32) - offset.y,
            /* v.z */ 0.0,
            /* t.u */ 0.0,
            /* t.v */ 1.0,
        ]
    }
}

impl Drawable for Sprite {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn draw(&self, shader: &Shader) {
        unsafe {
            let translation = Mat4x4::translate(Vec3::new(self.position.x, self.position.y, 0.0));
            let scale = Mat4x4::scale(Vec3::new(self.scale.x, self.scale.y, 1.0));
            let rotation = Mat4x4::rotate(self.rotation);
            let model = translation * rotation * scale;

            shader.set_parameter("model", model.as_ptr());

            (self.gl.glBindVertexArray)(self.vao_gl_id);
            (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_gl_id);
            (self.gl.glDrawElements)(opengl::GL_TRIANGLES, 6, opengl::GL_UNSIGNED_INT, ptr::null());
        }
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    fn move_toward(&mut self, delta: Vec2) {
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
}

impl Drop for Sprite {
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

            if self.texture_gl_id != 0 {
                (self.gl.glDeleteTextures)(1, &self.texture_gl_id);
            }
        }
    }
}
