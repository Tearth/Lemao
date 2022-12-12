use super::context::RendererContext;
use lemao_math::vec2::Vec2;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::ffi::c_void;
use std::rc::Rc;

pub mod bmp;
pub mod storage;

pub struct Texture {
    pub(crate) id: usize,
    pub(crate) texture_gl_id: u32,
    gl: Rc<OpenGLPointers>,

    size: Vec2,
}

impl Texture {
    pub fn new(renderer: &RendererContext, size: Vec2, data: Vec<u8>) -> Self {
        unsafe {
            let gl = renderer.gl.clone();
            let mut texture_gl_id = 0;

            (gl.glGenTextures)(1, &mut texture_gl_id);
            (gl.glBindTexture)(opengl::GL_TEXTURE_2D, texture_gl_id);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_WRAP_S, opengl::GL_MIRRORED_REPEAT as i32);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_WRAP_T, opengl::GL_MIRRORED_REPEAT as i32);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_MIN_FILTER, opengl::GL_LINEAR_MIPMAP_LINEAR as i32);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_MAG_FILTER, opengl::GL_LINEAR as i32);

            let texture = Self { id: 0, texture_gl_id, gl, size };
            texture.set_data(size, data);

            texture
        }
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    pub fn set_data(&self, size: Vec2, data: Vec<u8>) {
        unsafe {
            (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_gl_id);

            let format = opengl::GL_RGBA;
            let texture_width = size.x as i32;
            let texture_height = size.y as i32;
            let texture_ptr = data.as_ptr() as *const c_void;

            (self.gl.glTexImage2D)(opengl::GL_TEXTURE_2D, 0, format as i32, texture_width, texture_height, 0, format, opengl::GL_UNSIGNED_BYTE, texture_ptr);
            (self.gl.glGenerateMipmap)(opengl::GL_TEXTURE_2D);
        }
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
