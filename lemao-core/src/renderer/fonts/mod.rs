use super::context::RendererContext;
use lemao_opengl::bindings::opengl;
use std::ffi::c_void;

pub mod bff;
pub mod storage;

pub struct Font {
    pub id: usize,
    pub width: u32,
    pub height: u32,
    pub cell_width: u32,
    pub cell_height: u32,
    pub base_character_offset: u8,
    pub character_widths: Vec<u8>,
    pub data: Vec<u8>,
    pub texture_gl_id: u32,
}

impl Font {
    pub fn new(
        renderer: &RendererContext,
        width: u32,
        height: u32,
        cell_width: u32,
        cell_height: u32,
        base_character_offset: u8,
        character_widths: Vec<u8>,
        data: Vec<u8>,
    ) -> Self {
        unsafe {
            let gl = renderer.gl.clone();
            let mut texture_gl_id = 0;

            (gl.glGenTextures)(1, &mut texture_gl_id);
            (gl.glBindTexture)(opengl::GL_TEXTURE_2D, texture_gl_id);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_WRAP_S, opengl::GL_MIRRORED_REPEAT as i32);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_WRAP_T, opengl::GL_MIRRORED_REPEAT as i32);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_MIN_FILTER, opengl::GL_NEAREST_MIPMAP_LINEAR as i32);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_MAG_FILTER, opengl::GL_NEAREST as i32);

            let format = opengl::GL_RGBA;
            let texture_width = width as i32;
            let texture_height = height as i32;
            let texture_ptr = data.as_ptr() as *const c_void;

            (gl.glTexImage2D)(opengl::GL_TEXTURE_2D, 0, format as i32, texture_width, texture_height, 0, format, opengl::GL_UNSIGNED_BYTE, texture_ptr);
            (gl.glGenerateMipmap)(opengl::GL_TEXTURE_2D);

            Self { id: 0, width, height, cell_width, cell_height, base_character_offset, character_widths, data, texture_gl_id }
        }
    }
}
