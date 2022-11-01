use super::*;
use crate::renderer::context::RendererContext;
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
    pub position: Vec2<f32>,
    pub scale: Vec2<f32>,
    pub rotation: f32,

    anchor: Vec2<f32>,

    vao_index: u32,
    vbo_index: u32,
    ebo_index: u32,
    texture_index: u32,

    texture: Rc<Texture>,
    gl: Rc<OpenGLPointers>,
}

impl Sprite {
    pub fn new(renderer: &RendererContext, texture: Rc<Texture>) -> Self {
        let mut sprite = Sprite {
            position: Default::default(),
            scale: Default::default(),
            rotation: 0.0,

            anchor: Default::default(),

            vao_index: 0,
            vbo_index: 0,
            ebo_index: 0,
            texture_index: 0,

            texture: texture.clone(),
            gl: renderer.gl.clone(),
        };
        sprite.set_texture(texture);

        sprite
    }

    pub fn get_texture(&self) -> &Texture {
        &self.texture
    }

    pub fn set_texture(&mut self, texture: Rc<Texture>) {
        unsafe {
            if self.vao_index == 0 {
                (self.gl.glGenVertexArrays)(1, &mut self.vao_index);
            }
            (self.gl.glBindVertexArray)(self.vao_index);

            if self.vbo_index == 0 {
                (self.gl.glGenBuffers)(1, &mut self.vbo_index);
            }

            let vertices: [f32; 20] = self.get_vertices(Vec2::new(0.5, 0.5));
            let vertices_size = (mem::size_of::<f32>() * vertices.len()) as i64;
            let vertices_ptr = vertices.as_ptr() as *const c_void;

            (self.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, self.vbo_index);
            (self.gl.glBufferData)(opengl::GL_ARRAY_BUFFER, vertices_size, vertices_ptr, opengl::GL_STATIC_DRAW);

            if self.ebo_index == 0 {
                (self.gl.glGenBuffers)(1, &mut self.ebo_index);
            }

            let indices: [u32; 6] = [0, 1, 2, 0, 2, 3];
            let indices_size = (mem::size_of::<u32>() * indices.len()) as i64;
            let indices_ptr = indices.as_ptr() as *const c_void;

            (self.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, self.ebo_index);
            (self.gl.glBufferData)(opengl::GL_ELEMENT_ARRAY_BUFFER, indices_size, indices_ptr, opengl::GL_STATIC_DRAW);

            let attrib_size = (5 * mem::size_of::<f32>()) as i32;
            (self.gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size as i32, ptr::null_mut());
            (self.gl.glVertexAttribPointer)(1, 2, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (3 * mem::size_of::<f32>()) as *const c_void);

            (self.gl.glEnableVertexAttribArray)(0);
            (self.gl.glEnableVertexAttribArray)(1);

            if self.texture_index != 0 {
                (self.gl.glDeleteTextures)(1, &self.texture_index);
            }

            (self.gl.glGenTextures)(1, &mut self.texture_index);
            (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_index);
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
        }
    }

    pub fn get_anchor(&self) -> Vec2<f32> {
        self.anchor
    }

    pub fn set_anchor(&mut self, anchor: Vec2<f32>) {
        unsafe {
            if self.vbo_index == 0 {
                return;
            }

            let vertices = self.get_vertices(anchor);
            let vertices_size = (mem::size_of::<f32>() * vertices.len()) as i64;

            (self.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, self.vbo_index);
            (self.gl.glBufferData)(opengl::GL_ARRAY_BUFFER, vertices_size, vertices.as_ptr() as *const c_void, opengl::GL_STATIC_DRAW);

            self.anchor = anchor;
        }
    }

    fn get_vertices(&self, anchor: Vec2<f32>) -> [f32; 20] {
        let offset = anchor * Vec2::new(self.texture.width as f32, self.texture.height as f32);
        [
            0.0 - offset.x,
            0.0 - offset.y,
            0.0,
            0.0,
            0.0,
            (self.texture.width as f32) - offset.x,
            0.0 - offset.y,
            0.0,
            1.0,
            0.0,
            (self.texture.width as f32) - offset.x,
            (self.texture.height as f32) - offset.y,
            0.0,
            1.0,
            1.0,
            0.0 - offset.x,
            (self.texture.height as f32) - offset.y,
            0.0,
            0.0,
            1.0,
        ]
    }
}

impl Drawable for Sprite {
    fn draw(&self, shader: &Rc<Shader>) {
        unsafe {
            shader.as_ref().set_parameter("model", Mat4x4::translate(Vec3::new(self.position.x, self.position.y, 0.0)).as_ptr());

            (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_index);
            (self.gl.glBindVertexArray)(self.vao_index);
            (self.gl.glDrawElements)(opengl::GL_TRIANGLES, 6, opengl::GL_UNSIGNED_INT, ptr::null());
        }
    }
}

impl Drop for Sprite {
    fn drop(&mut self) {
        unsafe {
            if self.vbo_index != 0 {
                (self.gl.glDeleteBuffers)(1, &mut self.vbo_index);
            }

            if self.ebo_index != 0 {
                (self.gl.glDeleteBuffers)(1, &mut self.ebo_index);
            }

            if self.vao_index != 0 {
                (self.gl.glDeleteVertexArrays)(1, &mut self.vao_index);
            }

            if self.texture_index != 0 {
                (self.gl.glDeleteTextures)(1, &self.texture_index);
            }
        }
    }
}
