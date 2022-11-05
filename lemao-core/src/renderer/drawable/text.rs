use super::*;
use crate::renderer::fonts::Font;
use crate::utils::log;
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

pub struct Text {
    id: usize,

    position: Vec2,
    scale: Vec2,
    rotation: f32,

    width: u32,
    height: u32,
    anchor: Vec2,
    text: String,

    vao_gl_id: u32,
    vbo_gl_id: u32,
    ebo_gl_id: u32,
    texture_gl_id: u32,

    font_id: usize,
    font_width: u32,
    font_height: u32,
    font_cell_width: u32,
    font_cell_height: u32,
    font_base_character_offset: u8,
    font_character_widths: Vec<u8>,

    gl: Rc<OpenGLPointers>,
}

impl Text {
    pub fn new(gl: Rc<OpenGLPointers>, font: &Font) -> Self {
        let mut text = Text {
            id: 0,

            position: Default::default(),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,

            width: 0,
            height: 0,
            anchor: Default::default(),
            text: Default::default(),

            vao_gl_id: 0,
            vbo_gl_id: 0,
            ebo_gl_id: 0,
            texture_gl_id: 0,

            font_id: 0,
            font_width: font.width,
            font_height: font.height,
            font_cell_width: font.cell_width,
            font_cell_height: font.cell_height,
            font_base_character_offset: font.base_character_offset,
            font_character_widths: font.character_widths.clone(),

            gl,
        };

        text.set_texture(font);
        text
    }

    pub fn set_texture(&mut self, font: &Font) {
        unsafe {
            log::debug(&format!("Setting a new texture with gl_id {} for the sprite with id {}", font.id, self.id));

            if self.vao_gl_id == 0 {
                log::debug("Creating a new VAO buffer");
                (self.gl.glGenVertexArrays)(1, &mut self.vao_gl_id);
                log::debug(&format!("Created a new VAO buffer with gl_id {}", self.vao_gl_id));
            }
            (self.gl.glBindVertexArray)(self.vao_gl_id);

            if self.vbo_gl_id == 0 {
                log::debug("Creating a new VBO buffer");
                (self.gl.glGenBuffers)(1, &mut self.vbo_gl_id);
                log::debug(&format!("Created a new VBO buffer with gl_id {}", self.vbo_gl_id));
            }
            (self.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, self.vbo_gl_id);

            if self.ebo_gl_id == 0 {
                log::debug("Creating a new EBO buffer");
                (self.gl.glGenBuffers)(1, &mut self.ebo_gl_id);
                log::debug(&format!("Created a new EBO buffer with gl_id {}", self.ebo_gl_id));
            }
            (self.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, self.ebo_gl_id);

            let attrib_size = (5 * mem::size_of::<f32>()) as i32;
            (self.gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, ptr::null_mut());
            (self.gl.glVertexAttribPointer)(1, 2, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (3 * mem::size_of::<f32>()) as *const c_void);

            (self.gl.glEnableVertexAttribArray)(0);
            (self.gl.glEnableVertexAttribArray)(1);

            if self.texture_gl_id != 0 {
                log::debug("Deleting old texture");
                (self.gl.glDeleteTextures)(1, &self.texture_gl_id);
                log::debug(&format!("Texture with gl_id {} deleted", self.ebo_gl_id));
            }

            (self.gl.glGenTextures)(1, &mut self.texture_gl_id);
            (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_gl_id);
            (self.gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_WRAP_S, opengl::GL_MIRRORED_REPEAT as i32);
            (self.gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_WRAP_T, opengl::GL_MIRRORED_REPEAT as i32);
            (self.gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_MIN_FILTER, opengl::GL_NEAREST as i32);
            (self.gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_MAG_FILTER, opengl::GL_LINEAR as i32);

            let format = opengl::GL_RGBA;
            let texture_width = font.width as i32;
            let texture_height = font.height as i32;
            let texture_ptr = font.data.as_ptr() as *const c_void;

            (self.gl.glTexImage2D)(opengl::GL_TEXTURE_2D, 0, format as i32, texture_width, texture_height, 0, format, opengl::GL_UNSIGNED_BYTE, texture_ptr);
            (self.gl.glGenerateMipmap)(opengl::GL_TEXTURE_2D);

            self.width = texture_width as u32;
            self.height = texture_height as u32;
            self.font_id = font.id;

            log::debug(&format!("Texture setting for text with id {} done", self.id));
        }
    }

    pub fn set_text(&mut self, text: &str) {
        unsafe {
            let mut vertices = Vec::new();
            let mut indices = Vec::new();
            let mut offset = 0.0;

            let characters_per_row = (self.font_width / self.font_cell_width) as u8;
            let uv_width = self.font_cell_width as f32 / self.font_width as f32;
            let uv_height = self.font_cell_height as f32 / self.font_height as f32;

            for (index, char) in text.chars().enumerate() {
                let row = (char as u8 - self.font_base_character_offset) % characters_per_row;
                let col = (char as u8 - self.font_base_character_offset) / characters_per_row;

                let character_width = self.font_character_widths[char as usize];
                let uv = Vec2::new(row as f32 * uv_width, 1.0 - col as f32 * uv_height - uv_height);
                let uv_size = Vec2::new(uv_width, uv_height);

                vertices.extend_from_slice(&self.get_vertices(self.font_cell_width, self.font_cell_height, offset, uv, uv_size));

                let indices_offset = (index * 4) as u32;
                indices.extend_from_slice(&[
                    0 + indices_offset,
                    1 + indices_offset,
                    2 + indices_offset,
                    0 + indices_offset,
                    2 + indices_offset,
                    3 + indices_offset,
                ]);

                offset += character_width as f32;
            }

            let text_width = offset;
            let text_height = self.font_cell_height as f32;
            let anchor_offset = self.anchor * Vec2::new(text_width, text_height);

            for index in 0..(vertices.len() / 5) {
                vertices[index * 5 + 0] -= anchor_offset.x;
                vertices[index * 5 + 1] -= anchor_offset.y;
            }

            let vertices_size = (mem::size_of::<f32>() * vertices.len()) as i64;
            let vertices_ptr = vertices.as_ptr() as *const c_void;

            (self.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, self.vbo_gl_id);
            (self.gl.glBufferData)(opengl::GL_ARRAY_BUFFER, vertices_size, vertices_ptr, opengl::GL_STATIC_DRAW);

            let indices_size = (mem::size_of::<u32>() * indices.len()) as i64;
            let indices_ptr = indices.as_ptr() as *const c_void;

            (self.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, self.ebo_gl_id);
            (self.gl.glBufferData)(opengl::GL_ELEMENT_ARRAY_BUFFER, indices_size, indices_ptr, opengl::GL_STATIC_DRAW);

            self.text = text.to_string();
        }
    }

    pub fn get_anchor(&self) -> Vec2 {
        self.anchor
    }

    pub fn set_anchor(&mut self, anchor: Vec2) {
        self.anchor = anchor;
        self.set_text(&self.text.clone());
    }

    fn get_vertices(&self, width: u32, height: u32, offset: f32, uv: Vec2, uv_size: Vec2) -> [f32; 20] {
        [
            // Left-bottom
            /* v.x */ 0.0 + offset,
            /* v.y */ 0.0,
            /* v.z */ 0.0,
            /* t.u */ uv.x,
            /* t.v */ uv.y,
            // Right-bottom
            /* v.x */ (width as f32) + offset,
            /* v.y */ 0.0,
            /* v.z */ 0.0,
            /* t.u */ uv.x + uv_size.x,
            /* t.v */ uv.y,
            // Right-top
            /* v.x */ (width as f32) + offset,
            /* v.y */ (height as f32),
            /* v.z */ 0.0,
            /* t.u */ uv.x + uv_size.x,
            /* t.v */ uv.y + uv_size.y,
            // Left-top
            /* v.x */ 0.0 + offset,
            /* v.y */ (height as f32),
            /* v.z */ 0.0,
            /* t.u */ uv.x,
            /* t.v */ uv.y + uv_size.y,
        ]
    }
}

impl Drawable for Text {
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
            (self.gl.glDrawElements)(opengl::GL_TRIANGLES, (self.text.len() * 6) as i32, opengl::GL_UNSIGNED_INT, ptr::null());
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Drop for Text {
    fn drop(&mut self) {
        unsafe {
            if self.vbo_gl_id != 0 {
                log::debug(&format!("Deleting VBO buffer with gl_id {}", self.vbo_gl_id));
                (self.gl.glDeleteBuffers)(1, &mut self.vbo_gl_id);
                log::debug("Deleting VBO buffer done");
            }

            if self.ebo_gl_id != 0 {
                log::debug(&format!("Deleting EBO buffer with gl_id {}", self.ebo_gl_id));
                (self.gl.glDeleteBuffers)(1, &mut self.ebo_gl_id);
                log::debug("Deleting EBO buffer done");
            }

            if self.vao_gl_id != 0 {
                log::debug(&format!("Deleting VAO buffer with gl_id {}", self.vao_gl_id));
                (self.gl.glDeleteVertexArrays)(1, &mut self.vao_gl_id);
                log::debug("Deleting VAO buffer done");
            }

            if self.texture_gl_id != 0 {
                log::debug(&format!("Deleting texture with gl_id {}", self.texture_gl_id));
                (self.gl.glDeleteTextures)(1, &self.texture_gl_id);
                log::debug("Deleting texture done");
            }
        }
    }
}
