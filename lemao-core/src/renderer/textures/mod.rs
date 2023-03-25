use crate::utils::storage::StorageItem;

use super::context::RendererContext;
use lemao_math::vec2::Vec2;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::ffi::c_void;
use std::rc::Rc;

pub mod bmp;

pub struct RawTexture {
    pub size: Vec2,
    pub data: Vec<u8>,
}

pub struct Texture {
    pub id: usize,
    pub name: Option<String>,
    pub(crate) texture_gl_id: u32,
    gl: Rc<OpenGLPointers>,

    pub size: Vec2,
}

impl RawTexture {
    pub fn new(size: Vec2, data: Vec<u8>) -> Self {
        Self { size, data }
    }
}

impl Texture {
    pub fn new(renderer: &RendererContext, raw: &RawTexture) -> Result<Self, String> {
        unsafe {
            let gl = renderer.gl.clone();
            let mut texture_gl_id = 0;

            (gl.glGenTextures)(1, &mut texture_gl_id);
            (gl.glBindTexture)(opengl::GL_TEXTURE_2D, texture_gl_id);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_WRAP_S, opengl::GL_MIRRORED_REPEAT as i32);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_WRAP_T, opengl::GL_MIRRORED_REPEAT as i32);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_MIN_FILTER, opengl::GL_NEAREST_MIPMAP_NEAREST as i32);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_MAG_FILTER, opengl::GL_NEAREST as i32);

            let mut texture = Self { id: 0, name: None, texture_gl_id, gl, size: raw.size };
            texture.set_data(raw);

            Ok(texture)
        }
    }

    pub fn set_data(&mut self, raw: &RawTexture) {
        unsafe {
            (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_gl_id);

            let format = opengl::GL_RGBA;
            let texture_width = raw.size.x as i32;
            let texture_height = raw.size.y as i32;
            let texture_ptr = raw.data.as_ptr() as *const c_void;

            (self.gl.glTexImage2D)(opengl::GL_TEXTURE_2D, 0, format as i32, texture_width, texture_height, 0, format, opengl::GL_UNSIGNED_BYTE, texture_ptr);
            (self.gl.glGenerateMipmap)(opengl::GL_TEXTURE_2D);

            self.size = raw.size;
        }
    }
}

impl StorageItem for Texture {
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

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            if self.texture_gl_id != 0 {
                (self.gl.glDeleteTextures)(1, &self.texture_gl_id);
            }
        }
    }
}
