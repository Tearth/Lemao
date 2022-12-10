use super::context::RendererContext;
use super::drawable::Drawable;
use super::shaders::Shader;
use lemao_math::color::Color;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec4::Vec4;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::rc::Rc;

pub struct BatchRenderer {
    pub(crate) vao_gl_id: u32,
    pub(crate) vbo_gl_id: u32,
    pub(crate) ebo_gl_id: u32,
    gl: Rc<OpenGLPointers>,

    max_vertices_count: usize,
    max_indices_count: usize,
    first_batch_added: bool,
    vertices: Vec<f32>,
    indices: Vec<u32>,
    texture_gl_id: u32,
    color: Color,
    max_indice: u32,
}

pub struct Batch<'a> {
    pub(crate) shape_id: Option<usize>,
    pub(crate) vertices: Option<&'a Vec<f32>>,
    pub(crate) indices: Option<&'a Vec<u32>>,
    pub(crate) texture_gl_id: Option<u32>,
    pub(crate) color: Option<Color>,
}

impl BatchRenderer {
    pub fn new(renderer: &RendererContext, max_vertices_count: usize, max_indices_count: usize) -> Self {
        unsafe {
            let gl = renderer.gl.clone();

            let mut vao_gl_id = 0;
            (gl.glGenVertexArrays)(1, &mut vao_gl_id);
            (gl.glBindVertexArray)(vao_gl_id);

            let data_size = (mem::size_of::<f32>() * max_vertices_count) as i64;

            let mut vbo_gl_id = 0;
            (gl.glGenBuffers)(1, &mut vbo_gl_id);
            (gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, vbo_gl_id);
            (gl.glBufferData)(opengl::GL_ARRAY_BUFFER, data_size, ptr::null(), opengl::GL_STATIC_DRAW);

            let indices_size = (mem::size_of::<u32>() * max_indices_count) as i64;

            let mut ebo_gl_id = 0;
            (gl.glGenBuffers)(1, &mut ebo_gl_id);
            (gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, ebo_gl_id);
            (gl.glBufferData)(opengl::GL_ELEMENT_ARRAY_BUFFER, indices_size, ptr::null(), opengl::GL_STATIC_DRAW);

            let attrib_size = (9 * mem::size_of::<f32>()) as i32;
            (gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, ptr::null_mut());
            (gl.glVertexAttribPointer)(1, 4, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (3 * mem::size_of::<f32>()) as *const c_void);
            (gl.glVertexAttribPointer)(2, 2, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (7 * mem::size_of::<f32>()) as *const c_void);

            (gl.glEnableVertexAttribArray)(0);
            (gl.glEnableVertexAttribArray)(1);
            (gl.glEnableVertexAttribArray)(2);

            Self {
                vao_gl_id,
                vbo_gl_id,
                ebo_gl_id,
                gl,
                max_vertices_count,
                max_indices_count,
                first_batch_added: false,
                vertices: Vec::new(),
                indices: Vec::new(),
                texture_gl_id: 0,
                color: Color::new(0.0, 0.0, 0.0, 1.0),
                max_indice: 0,
            }
        }
    }

    pub fn add(&mut self, drawable: &dyn Drawable, batch: &Batch) -> Result<(), String> {
        if self.first_batch_added {
            if self.texture_gl_id != batch.texture_gl_id.unwrap() {
                return Err("Invalid texture".to_string());
            }

            if self.color != batch.color.unwrap() {
                return Err("Invalid color".to_string());
            }
        }

        let vertices = batch.vertices.unwrap();
        if self.vertices.len() + vertices.len() > self.max_vertices_count {
            return Err("Too many vertices".to_string());
        }

        let indices = batch.indices.unwrap();
        if self.indices.len() + indices.len() > self.max_indices_count {
            return Err("Too many indices".to_string());
        }

        let transformation_matrix = drawable.get_transformation_matrix();
        for index in 0..(vertices.len() / 9) {
            let position = Vec4::new(vertices[index * 9 + 0], vertices[index * 9 + 1], vertices[index * 9 + 2], 1.0);
            let transformed_position = transformation_matrix * position;

            self.vertices.push(transformed_position.x);
            self.vertices.push(transformed_position.y);
            self.vertices.push(transformed_position.z);

            self.vertices.push(vertices[index * 9 + 3]);
            self.vertices.push(vertices[index * 9 + 4]);
            self.vertices.push(vertices[index * 9 + 5]);
            self.vertices.push(vertices[index * 9 + 6]);
            self.vertices.push(vertices[index * 9 + 7]);
            self.vertices.push(vertices[index * 9 + 8]);
        }

        let base_indice = self.max_indice;
        for index in 0..indices.len() {
            self.indices.push(indices[index] + base_indice);
            self.max_indice = self.max_indice.max(indices[index] + base_indice + 1);
        }

        self.first_batch_added = true;
        self.texture_gl_id = batch.texture_gl_id.unwrap();
        self.color = batch.color.unwrap();

        Ok(())
    }

    pub fn draw(&mut self, shader: &Shader) -> Result<(), String> {
        unsafe {
            let data_size = (mem::size_of::<f32>() * self.vertices.len()) as i64;
            let data_ptr = self.vertices.as_ptr() as *const c_void;

            (self.gl.glBindVertexArray)(self.vao_gl_id);
            (self.gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, self.vbo_gl_id);
            (self.gl.glBufferData)(opengl::GL_ARRAY_BUFFER, data_size, data_ptr, opengl::GL_STATIC_DRAW);

            let indices_size = (mem::size_of::<u32>() * self.indices.len()) as i64;
            let indices_ptr = self.indices.as_ptr() as *const c_void;

            (self.gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, self.ebo_gl_id);
            (self.gl.glBufferData)(opengl::GL_ELEMENT_ARRAY_BUFFER, indices_size, indices_ptr, opengl::GL_STATIC_DRAW);

            shader.set_parameter("model", Mat4x4::identity().as_ptr())?;
            shader.set_parameter("color", Color::new(1.0, 1.0, 1.0, 1.0).as_ptr())?;

            (self.gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_gl_id);
            (self.gl.glDrawElements)(opengl::GL_TRIANGLES, self.indices.len() as i32, opengl::GL_UNSIGNED_INT, ptr::null());

            self.first_batch_added = false;
            self.vertices.clear();
            self.indices.clear();
            self.texture_gl_id = 0;
            self.max_indice = 0;

            Ok(())
        }
    }
}

impl<'a> Batch<'a> {
    pub fn new(
        shape_id: Option<usize>,
        vertices: Option<&'a Vec<f32>>,
        indices: Option<&'a Vec<u32>>,
        texture_gl_id: Option<u32>,
        color: Option<Color>,
    ) -> Self {
        Self { shape_id, vertices, indices, texture_gl_id, color }
    }
}
