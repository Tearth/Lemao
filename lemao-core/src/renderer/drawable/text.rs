use super::*;
use crate::renderer::context::RendererContext;
use crate::renderer::fonts::Font;
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
    size: Vec2,
    anchor: Vec2,
    color: Color,
    text: String,
    line_height: u32,
    vao_gl_id: u32,
    vbo_gl_id: u32,
    ebo_gl_id: u32,
    texture_gl_id: u32,
    elements_count: u32,
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
    pub fn new(renderer: &RendererContext, font: &Font) -> Self {
        let mut text = Text {
            id: 0,
            position: Default::default(),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
            size: Default::default(),
            anchor: Default::default(),
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            text: Default::default(),
            line_height: font.cell_height,
            vao_gl_id: 0,
            vbo_gl_id: 0,
            ebo_gl_id: 0,
            texture_gl_id: 0,
            elements_count: 0,
            font_id: 0,
            font_width: font.width,
            font_height: font.height,
            font_cell_width: font.cell_width,
            font_cell_height: font.cell_height,
            font_base_character_offset: font.base_character_offset,
            font_character_widths: font.character_widths.clone(),
            gl: renderer.gl.clone(),
        };

        text.set_font(font);
        text
    }

    pub fn get_font(&self) -> usize {
        self.font_id
    }

    pub fn set_font(&mut self, font: &Font) {
        unsafe {
            if self.vao_gl_id == 0 {
                (self.gl.glGenVertexArrays)(1, &mut self.vao_gl_id);
            }
            (self.gl.glBindVertexArray)(self.vao_gl_id);

            if self.vbo_gl_id == 0 {
                (self.gl.glGenBuffers)(1, &mut self.vbo_gl_id);
            }
            (self.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, self.vbo_gl_id);

            if self.ebo_gl_id == 0 {
                (self.gl.glGenBuffers)(1, &mut self.ebo_gl_id);
            }
            (self.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, self.ebo_gl_id);

            let attrib_size = (9 * mem::size_of::<f32>()) as i32;
            (self.gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, ptr::null_mut());
            (self.gl.glVertexAttribPointer)(1, 4, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (3 * mem::size_of::<f32>()) as *const c_void);
            (self.gl.glVertexAttribPointer)(2, 2, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (7 * mem::size_of::<f32>()) as *const c_void);

            (self.gl.glEnableVertexAttribArray)(0);
            (self.gl.glEnableVertexAttribArray)(1);
            (self.gl.glEnableVertexAttribArray)(2);

            self.font_id = font.id;
            self.texture_gl_id = font.texture_gl_id;
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        unsafe {
            let mut vertices = Vec::new();
            let mut indices = Vec::new();
            let mut offset: Vec2 = Default::default();
            let mut size: Vec2 = Vec2::new(0.0, self.line_height as f32);

            let characters_per_row = (self.font_width / self.font_cell_width) as u8;
            let uv_width = self.font_cell_width as f32 / self.font_width as f32;
            let uv_height = self.font_cell_height as f32 / self.font_height as f32;
            let uv_size = Vec2::new(uv_width, uv_height);
            let mut index = 0;

            for char in text.chars() {
                if char == '\n' {
                    offset.x = 0.0;
                    offset.y -= self.line_height as f32;
                    size.y += self.line_height as f32;
                    continue;
                }

                let row = (char as u8 - self.font_base_character_offset) % characters_per_row;
                let col = (char as u8 - self.font_base_character_offset) / characters_per_row;

                let character_width = self.font_character_widths[char as usize];
                let uv = Vec2::new(row as f32 * uv_width, 1.0 - col as f32 * uv_height - uv_height);

                vertices.extend_from_slice(&self.get_vertices(self.font_cell_width, self.font_cell_height, offset, uv, uv_size, self.color));

                let indices_offset = (index * 4) as u32;
                indices.extend_from_slice(&[
                    0 + indices_offset,
                    1 + indices_offset,
                    2 + indices_offset,
                    0 + indices_offset,
                    2 + indices_offset,
                    3 + indices_offset,
                ]);

                offset += Vec2::new(character_width as f32, 0.0);
                size.x = size.x.max(offset.x);
                index += 1;
            }

            let anchor_offset = self.anchor * size;
            for index in 0..(vertices.len() / 9) {
                vertices[index * 9 + 0] -= anchor_offset.x;
                vertices[index * 9 + 1] += size.y - self.font_cell_height as f32 - anchor_offset.y;
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
            self.size = size;
            self.elements_count = indices.len() as u32;
        }
    }

    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    pub fn get_line_height(&self) -> u32 {
        self.line_height
    }

    pub fn set_line_height(&mut self, line_height: u32) {
        self.line_height = line_height;
        self.set_text(&self.text.clone());
    }

    fn get_vertices(&self, width: u32, height: u32, offset: Vec2, uv: Vec2, uv_size: Vec2, color: Color) -> [f32; 36] {
        [
            // Left-bottom
            /* v.x */ 0.0 + offset.x,
            /* v.y */ 0.0 + offset.y,
            /* v.z */ 0.0,
            /* c.r */ color.r,
            /* c.g */ color.g,
            /* c.b */ color.b,
            /* c.a */ color.a,
            /* t.u */ uv.x,
            /* t.v */ uv.y,
            // Right-bottom
            /* v.x */ (width as f32) + offset.x,
            /* v.y */ 0.0 + offset.y,
            /* v.z */ 0.0,
            /* c.r */ color.r,
            /* c.g */ color.g,
            /* c.b */ color.b,
            /* c.a */ color.a,
            /* t.u */ uv.x + uv_size.x,
            /* t.v */ uv.y,
            // Right-top
            /* v.x */ (width as f32) + offset.x,
            /* v.y */ (height as f32) + offset.y,
            /* v.z */ 0.0,
            /* c.r */ color.r,
            /* c.g */ color.g,
            /* c.b */ color.b,
            /* c.a */ color.a,
            /* t.u */ uv.x + uv_size.x,
            /* t.v */ uv.y + uv_size.y,
            // Left-top
            /* v.x */ 0.0 + offset.x,
            /* v.y */ (height as f32) + offset.y,
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

impl Drawable for Text {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

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

    fn get_anchor(&self) -> Vec2 {
        self.anchor
    }

    fn set_anchor(&mut self, anchor: Vec2) -> Result<(), String> {
        if self.vbo_gl_id == 0 {
            return Err("Sprite not initialized".to_string());
        }

        self.anchor = anchor;
        self.set_text(&self.text.clone());

        Ok(())
    }

    fn get_color(&self) -> Color {
        self.color
    }

    fn set_color(&mut self, color: Color) -> Result<(), String> {
        if self.vbo_gl_id == 0 {
            return Err("Sprite not initialized".to_string());
        }

        self.color = color;
        self.set_text(&self.text.clone());

        Ok(())
    }

    fn draw(&self, shader: &Shader) -> Result<(), String> {
        unsafe {
            let translation = Mat4x4::translate(Vec3::new(self.position.x, self.position.y, 0.0));
            let scale = Mat4x4::scale(Vec3::new(self.scale.x, self.scale.y, 1.0));
            let rotation = Mat4x4::rotate(self.rotation);
            let model = translation * rotation * scale;

            shader.set_parameter("model", model.as_ptr())?;

            (self.gl.glBindVertexArray)(self.vao_gl_id);
            (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_gl_id);
            (self.gl.glDrawElements)(opengl::GL_TRIANGLES, self.elements_count as i32, opengl::GL_UNSIGNED_INT, ptr::null());

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

impl Drop for Text {
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
