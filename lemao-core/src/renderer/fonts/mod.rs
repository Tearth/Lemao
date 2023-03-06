use crate::utils::storage::StorageItem;

use super::context::RendererContext;
use super::textures::RawTexture;
use lemao_math::vec2::Vec2;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::ffi::c_void;
use std::rc::Rc;

pub mod bff;

pub struct RawFont {
    pub size: Vec2,
    pub cell_size: Vec2,
    pub base_character_offset: u8,
    pub character_widths: Vec<u8>,
    pub data: Vec<u8>,
}

pub struct Font {
    pub id: usize,
    pub name: Option<String>,
    pub(crate) texture_gl_id: u32,
    gl: Rc<OpenGLPointers>,

    pub size: Vec2,
    pub cell_size: Vec2,
    pub base_character_offset: u8,
    pub character_widths: Vec<u8>,
}

impl RawFont {
    pub fn new(size: Vec2, cell_size: Vec2, base_character_offset: u8, character_widths: Vec<u8>, data: Vec<u8>) -> Self {
        Self { size, cell_size, base_character_offset, character_widths, data }
    }

    pub fn set_character(&mut self, char: u8, offset: Vec2, texture: &RawTexture) {
        let texture_size = texture.size;
        let texture_data = &texture.data;

        let characters_per_row = (self.size.x / self.cell_size.x) as u8;
        let row = (char - self.base_character_offset) % characters_per_row;
        let col = (char - self.base_character_offset) / characters_per_row;
        let initial_x = (row as f32 * self.cell_size.x) as usize;
        let initial_y = (self.size.y - (col as f32 * self.cell_size.y) - self.cell_size.y) as usize;

        let mut texture_data_index = 0;
        for y in initial_y..initial_y + self.cell_size.y as usize {
            for x in initial_x..initial_x + self.cell_size.x as usize {
                for p in 0..4 {
                    self.data[x * 4 + y * self.size.x as usize * 4 + p] = 0;
                    texture_data_index += 1;
                }
            }
        }

        let initial_x = initial_x + offset.x as usize;
        let initial_y = initial_y + offset.y as usize;
        texture_data_index = 0;

        for y in initial_y..initial_y + texture_size.y as usize {
            for x in initial_x..initial_x + texture_size.x as usize {
                for p in 0..4 {
                    self.data[x * 4 + y * self.size.x as usize * 4 + p] = texture_data[texture_data_index];
                    texture_data_index += 1;
                }
            }
        }

        self.character_widths[char as usize] = texture_size.x as u8;
    }
}

impl Font {
    pub fn new(renderer: &RendererContext, raw: &RawFont) -> Result<Self, String> {
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
            let texture_ptr = raw.data.as_ptr() as *const c_void;

            (gl.glTexImage2D)(opengl::GL_TEXTURE_2D, 0, format as i32, raw.size.x as i32, raw.size.y as i32, 0, format, opengl::GL_UNSIGNED_BYTE, texture_ptr);
            (gl.glGenerateMipmap)(opengl::GL_TEXTURE_2D);

            Ok(Self {
                id: 0,
                name: None,
                texture_gl_id,
                gl,
                size: raw.size,
                cell_size: raw.cell_size,
                base_character_offset: raw.base_character_offset,
                character_widths: raw.character_widths.clone(),
            })
        }
    }
}

impl StorageItem for Font {
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

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            if self.texture_gl_id != 0 {
                (self.gl.glDeleteTextures)(1, &self.texture_gl_id);
            }
        }
    }
}
