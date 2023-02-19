use crate::utils::storage::StorageItem;

use super::context::RendererContext;
use lemao_math::color::SolidColor;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::any::Any;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use std::rc::Rc;

pub struct Shape {
    pub(crate) id: usize,
    pub(crate) vao_gl_id: u32,
    pub(crate) vbo_gl_id: u32,
    pub(crate) ebo_gl_id: u32,
    gl: Rc<OpenGLPointers>,

    vertices: Vec<f32>,
    indices: Vec<u32>,
}

impl Shape {
    pub fn new(renderer: &RendererContext, vertices: Vec<Vec3>, indices: Vec<u32>, uv: Vec<Vec2>, colors: Vec<SolidColor>) -> Self {
        unsafe {
            let gl = renderer.gl.clone();

            let mut vao_gl_id = 0;
            (gl.glGenVertexArrays)(1, &mut vao_gl_id);
            (gl.glBindVertexArray)(vao_gl_id);

            let mut data = Vec::new();
            for i in 0..vertices.len() {
                data.push(vertices[i].x);
                data.push(vertices[i].y);
                data.push(vertices[i].z);

                data.push(colors[i].r);
                data.push(colors[i].g);
                data.push(colors[i].b);
                data.push(colors[i].a);

                data.push(uv[i].x);
                data.push(uv[i].y);
            }

            let data_size = (mem::size_of::<f32>() * data.len()) as i64;
            let data_ptr = data.as_ptr() as *const c_void;

            let mut vbo_gl_id = 0;
            (gl.glGenBuffers)(1, &mut vbo_gl_id);
            (gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, vbo_gl_id);
            (gl.glBufferData)(opengl::GL_ARRAY_BUFFER, data_size, data_ptr, opengl::GL_STATIC_DRAW);

            let indices_size = (mem::size_of::<u32>() * indices.len()) as i64;
            let indices_ptr = indices.as_ptr() as *const c_void;

            let mut ebo_gl_id = 0;
            (gl.glGenBuffers)(1, &mut ebo_gl_id);
            (gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, ebo_gl_id);
            (gl.glBufferData)(opengl::GL_ELEMENT_ARRAY_BUFFER, indices_size, indices_ptr, opengl::GL_STATIC_DRAW);

            let attrib_size = (9 * mem::size_of::<f32>()) as i32;
            (gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, ptr::null_mut());
            (gl.glVertexAttribPointer)(1, 4, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (3 * mem::size_of::<f32>()) as *const c_void);
            (gl.glVertexAttribPointer)(2, 2, opengl::GL_FLOAT, opengl::GL_FALSE as u8, attrib_size, (7 * mem::size_of::<f32>()) as *const c_void);

            (gl.glEnableVertexAttribArray)(0);
            (gl.glEnableVertexAttribArray)(1);
            (gl.glEnableVertexAttribArray)(2);

            Self { id: 0, vao_gl_id, vbo_gl_id, ebo_gl_id, gl, vertices: data, indices }
        }
    }

    pub fn get_vertices(&self) -> &Vec<f32> {
        &self.vertices
    }

    pub fn get_indices(&self) -> &Vec<u32> {
        &self.indices
    }
}

impl StorageItem for Shape {
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
}

impl Drop for Shape {
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
