use super::context::RendererContext;
use lemao_math::vec2::Vec2;
use lemao_opengl::bindings::opengl;
use std::ffi::c_void;

pub mod bmp;
pub mod storage;

pub struct Texture {
    pub id: usize,
    pub size: Vec2,
    pub data: Vec<u8>,
    pub(crate) texture_gl_id: u32,
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

            let format = opengl::GL_RGBA;
            let texture_width = size.x as i32;
            let texture_height = size.y as i32;
            let texture_ptr = data.as_ptr() as *const c_void;

            (gl.glTexImage2D)(opengl::GL_TEXTURE_2D, 0, format as i32, texture_width, texture_height, 0, format, opengl::GL_UNSIGNED_BYTE, texture_ptr);
            (gl.glGenerateMipmap)(opengl::GL_TEXTURE_2D);

            Self { id: 0, size, data, texture_gl_id }
        }
    }
}
