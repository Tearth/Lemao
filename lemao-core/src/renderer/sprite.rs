use lemao_opengl::bindings::opengl;
use lemao_opengl::context::OpenGLContext;
use std::ffi::c_void;
use std::mem;
use std::ptr;

pub struct Sprite {
    pub vao: u32,
}

impl Sprite {
    pub fn new(gl: &OpenGLContext) -> Self {
        unsafe {
            let mut vao = 0;
            (gl.glGenVertexArrays)(1, &mut vao);
            (gl.glBindVertexArray)(vao);

            let vertices: [f32; 12] = [0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0, -0.5, 0.5, 0.0];
            let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

            let mut vbo = 0u32;
            (gl.glGenBuffers)(1, &mut vbo);
            (gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, vbo);
            (gl.glBufferData)(
                opengl::GL_ARRAY_BUFFER,
                (mem::size_of::<f32>() * vertices.len()) as i64,
                vertices.as_ptr() as *const c_void,
                opengl::GL_STATIC_DRAW,
            );

            let mut ebo = 0u32;
            (gl.glGenBuffers)(1, &mut ebo);
            (gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, ebo);
            (gl.glBufferData)(
                opengl::GL_ELEMENT_ARRAY_BUFFER,
                (mem::size_of::<u32>() * indices.len()) as i64,
                indices.as_ptr() as *const c_void,
                opengl::GL_STATIC_DRAW,
            );

            (gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, (3 * mem::size_of::<f32>()) as i32, ptr::null_mut());
            (gl.glEnableVertexAttribArray)(0);

            Sprite { vao }
        }
    }

    pub fn draw(&self, gl: &OpenGLContext) {
        unsafe {
            (gl.glBindVertexArray)(self.vao);
            (gl.glDrawElements)(opengl::GL_TRIANGLES, 6, opengl::GL_UNSIGNED_INT, ptr::null());
        }
    }
}
