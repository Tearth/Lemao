use super::*;
use crate::renderer::context::RendererContext;
use crate::renderer::fonts::Font;
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

pub struct Text {
    pub(crate) id: usize,
    pub(crate) vao_gl_id: u32,
    pub(crate) vbo_gl_id: u32,
    pub(crate) ebo_gl_id: u32,
    pub(crate) font_id: usize,
    pub(crate) texture_gl_id: u32,
    gl: Rc<OpenGLPointers>,

    position: Vec2,
    scale: Vec2,
    rotation: f32,
    size: Vec2,
    anchor: Vec2,
    color: Color,
    text: String,
    line_height: u32,
    elements_count: u32,
    vertices: Vec<f32>,
    indices: Vec<u32>,

    font_size: Vec2,
    font_cell_size: Vec2,
    font_base_character_offset: u8,
    font_character_widths: Vec<u8>,
}

impl Text {
    pub fn new(renderer: &RendererContext, font: &Font) -> Self {
        let mut text = Text {
            id: 0,
            vao_gl_id: 0,
            vbo_gl_id: 0,
            ebo_gl_id: 0,
            font_id: font.id,
            texture_gl_id: font.texture_gl_id,
            gl: renderer.gl.clone(),

            position: Default::default(),
            scale: Vec2::new(1.0, 1.0),
            rotation: 0.0,
            size: Default::default(),
            anchor: Default::default(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            text: Default::default(),
            line_height: font.get_cell_size().y as u32,
            elements_count: 0,
            vertices: Vec::new(),
            indices: Vec::new(),

            font_size: font.get_size(),
            font_cell_size: font.get_cell_size(),
            font_base_character_offset: font.get_base_character_offset(),
            font_character_widths: font.get_character_widths(),
        };

        unsafe {
            (text.gl.glGenVertexArrays)(1, &mut text.vao_gl_id);
            (text.gl.glBindVertexArray)(text.vao_gl_id);

            (text.gl.glGenBuffers)(1, &mut text.vbo_gl_id);
            (text.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, text.vbo_gl_id);

            (text.gl.glGenBuffers)(1, &mut text.ebo_gl_id);
            (text.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, text.ebo_gl_id);

            let attrib_size = (9 * mem::size_of::<f32>()) as i32;
            (text.gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, ptr::null_mut());
            (text.gl.glVertexAttribPointer)(1, 4, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (3 * mem::size_of::<f32>()) as *const c_void);
            (text.gl.glVertexAttribPointer)(2, 2, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (7 * mem::size_of::<f32>()) as *const c_void);

            (text.gl.glEnableVertexAttribArray)(0);
            (text.gl.glEnableVertexAttribArray)(1);
            (text.gl.glEnableVertexAttribArray)(2);
        }

        text
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_font_id(&self) -> usize {
        self.font_id
    }

    pub fn set_font(&mut self, font: &Font) {
        self.font_id = font.id;
        self.texture_gl_id = font.texture_gl_id;

        // We must regenerate mesh, since the font sizes could change
        self.set_text(&self.text.clone());
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }

    pub fn set_text(&mut self, text: &str) {
        unsafe {
            let mut offset: Vec2 = Default::default();
            let mut size: Vec2 = Vec2::new(0.0, self.line_height as f32);

            let characters_per_row = (self.font_size.x / self.font_cell_size.x) as u8;
            let uv_width = self.font_cell_size.x / self.font_size.x;
            let uv_height = self.font_cell_size.y / self.font_size.y;
            let uv_size = Vec2::new(uv_width, uv_height);
            let mut index = 0;
            let mut color = SolidColor::new(1.0, 1.0, 1.0, 1.0);
            let mut color_section = false;
            let mut color_definition = String::new();

            self.vertices.clear();
            self.indices.clear();

            for char in text.chars() {
                if char == '\n' {
                    offset.x = 0.0;
                    offset.y -= self.line_height as f32;
                    size.y += self.line_height as f32;
                    continue;
                } else if char == '°' {
                    if color_section {
                        let tokens = color_definition.split(',').collect::<Vec<&str>>();
                        let r = tokens[0].parse::<u8>().unwrap() as f32;
                        let g = tokens[1].parse::<u8>().unwrap() as f32;
                        let b = tokens[2].parse::<u8>().unwrap() as f32;
                        let a = tokens[3].parse::<u8>().unwrap() as f32;

                        color = SolidColor::new(r / 255.0, g / 255.0, b / 255.0, a / 255.0);
                        color_section = false;
                    } else {
                        color_section = true;
                        color_definition.clear();
                    }

                    continue;
                }

                if color_section {
                    color_definition += &char.to_string();
                    continue;
                }

                let row = (char as u8 - self.font_base_character_offset) % characters_per_row;
                let col = (char as u8 - self.font_base_character_offset) / characters_per_row;

                let character_width = self.font_character_widths[char as usize];
                let uv = Vec2::new(row as f32 * uv_width, 1.0 - col as f32 * uv_height - uv_height);

                self.vertices.extend_from_slice(&self.get_vertices(self.font_cell_size.x as u32, self.font_cell_size.y as u32, offset, uv, uv_size, color));

                let indices_offset = (index * 4) as u32;
                self.indices.extend_from_slice(&[
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

            // Adjust vertices, so the default anchor is in the left-bottom corner
            for index in 0..(self.vertices.len() / 9) {
                self.vertices[index * 9 + 1] += size.y - self.font_cell_size.y;
            }

            let vertices_size = (mem::size_of::<f32>() * self.vertices.len()) as i64;
            let vertices_ptr = self.vertices.as_ptr() as *const c_void;

            (self.gl.glBindVertexArray)(self.vao_gl_id);
            (self.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, self.vbo_gl_id);
            (self.gl.glBufferData)(opengl::GL_ARRAY_BUFFER, vertices_size, vertices_ptr, opengl::GL_STATIC_DRAW);

            let indices_size = (mem::size_of::<u32>() * self.indices.len()) as i64;
            let indices_ptr = self.indices.as_ptr() as *const c_void;

            (self.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, self.ebo_gl_id);
            (self.gl.glBufferData)(opengl::GL_ELEMENT_ARRAY_BUFFER, indices_size, indices_ptr, opengl::GL_STATIC_DRAW);

            self.text = text.to_string();
            self.size = size;
            self.elements_count = self.indices.len() as u32;
        }
    }

    pub fn get_line_height(&self) -> u32 {
        self.line_height
    }

    pub fn set_line_height(&mut self, line_height: u32) {
        self.line_height = line_height;
        self.set_text(&self.text.clone());
    }

    pub fn calculate_text_size(&self, text: String) -> Vec2 {
        let mut text_size = Vec2::new(0.0, self.line_height as f32);
        let mut line_width = 0.0;
        let mut color_section = false;

        for char in text.chars() {
            if char == '\n' {
                line_width = 0.0;
                text_size.y += self.line_height as f32;
                continue;
            } else if char == '°' {
                color_section = !color_section;
                continue;
            }

            if color_section {
                continue;
            }

            line_width += self.font_character_widths[char as usize] as f32;
            text_size.x = f32::max(text_size.x, line_width);
        }

        text_size
    }

    fn get_vertices(&self, width: u32, height: u32, offset: Vec2, uv: Vec2, uv_size: Vec2, color: SolidColor) -> [f32; 36] {
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
        let anchor_offset = Mat4x4::translate(-Vec3::from(self.anchor * self.size).floor());
        let scale = Mat4x4::scale(Vec3::from(self.scale));
        let rotation = Mat4x4::rotate(self.rotation);
        translation * rotation * scale * anchor_offset
    }

    fn get_batch(&self) -> Batch {
        Batch::new(None, Some(&self.vertices), Some(&self.indices), Some(self.texture_gl_id), Some(&self.color))
    }

    fn draw(&self, shader: &Shader) -> Result<(), String> {
        unsafe {
            let model = self.get_transformation_matrix();

            shader.set_parameter("model", model.as_ptr())?;
            shader.set_color(&self.color)?;

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

impl StorageItem for Text {
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
