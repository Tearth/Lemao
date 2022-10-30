use lemao_opengl::bindings::opengl;
use lemao_opengl::context::OpenGLContext;
use std::ffi::c_void;
use std::mem;
use std::ptr;
use crate::renderer::textures::Texture;
use crate::renderer::textures::TextureFormat;

pub struct Sprite {
    pub vao_index: u32,
    pub texture_index: u32
}

impl Sprite {
    pub fn new(gl: &OpenGLContext, texture_data: &Texture) -> Self {
        unsafe {
            let mut vao_index = 0;
            (gl.glGenVertexArrays)(1, &mut vao_index);
            (gl.glBindVertexArray)(vao_index);
            
            #[rustfmt::skip]
            let vertices: [f32; 20] = [
                120.0, 120.0, 0.0,      1.0, 1.0, 
                120.0, -120.0, 0.0,     1.0, 0.0,
                -120.0, -120.0, 0.0,    0.0, 0.0,
                -120.0, 120.0, 0.0,     0.0, 1.0,
            ];
            let indices: [u32; 6] = [0, 1, 3, 1, 2, 3];

            let mut vbo_index = 0u32;
            (gl.glGenBuffers)(1, &mut vbo_index);
            (gl.glBindBuffer)(opengl::GL_ARRAY_BUFFER, vbo_index);
            (gl.glBufferData)(
                opengl::GL_ARRAY_BUFFER,
                (mem::size_of::<f32>() * vertices.len()) as i64,
                vertices.as_ptr() as *const c_void,
                opengl::GL_STATIC_DRAW,
            );

            let mut ebo_index = 0u32;
            (gl.glGenBuffers)(1, &mut ebo_index);
            (gl.glBindBuffer)(opengl::GL_ELEMENT_ARRAY_BUFFER, ebo_index);
            (gl.glBufferData)(
                opengl::GL_ELEMENT_ARRAY_BUFFER,
                (mem::size_of::<u32>() * indices.len()) as i64,
                indices.as_ptr() as *const c_void,
                opengl::GL_STATIC_DRAW,
            );

            (gl.glVertexAttribPointer)(0, 3, opengl::GL_FLOAT, opengl::GL_FALSE as u8, (5 * mem::size_of::<f32>()) as i32, ptr::null_mut());
            (gl.glEnableVertexAttribArray)(0);

            (gl.glVertexAttribPointer)(
                1,
                2,
                opengl::GL_FLOAT,
                opengl::GL_FALSE as u8,
                (5 * mem::size_of::<f32>()) as i32,
                (3 * mem::size_of::<f32>()) as *const c_void,
            );
            (gl.glEnableVertexAttribArray)(1);

            // Texture
            let mut texture_index = 0;
            let format = if texture_data.format == TextureFormat::RGB { opengl::GL_RGB } else { opengl::GL_RGBA};

            (gl.glGenTextures)(1, &mut texture_index);
            (gl.glBindTexture)(opengl::GL_TEXTURE_2D, texture_index);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_WRAP_S, opengl::GL_MIRRORED_REPEAT as i32);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_WRAP_T, opengl::GL_MIRRORED_REPEAT as i32);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_MIN_FILTER, opengl::GL_NEAREST as i32);
            (gl.glTexParameteri)(opengl::GL_TEXTURE_2D, opengl::GL_TEXTURE_MAG_FILTER, opengl::GL_LINEAR as i32);
            (gl.glTexImage2D)(
                opengl::GL_TEXTURE_2D,
                0,
                format as i32,
                240,
                240,
                0,
                format,
                opengl::GL_UNSIGNED_BYTE,
                texture_data.data.as_ptr() as *const c_void,
            );
            (gl.glGenerateMipmap)(opengl::GL_TEXTURE_2D);

            Sprite { vao_index, texture_index }
        }
    }

    pub fn draw(&self, gl: &OpenGLContext) {
        unsafe {
            (gl.glBindTexture)(opengl::GL_TEXTURE_2D, self.texture_index);
            (gl.glBindVertexArray)(self.vao_index);
            (gl.glDrawElements)(opengl::GL_TRIANGLES, 6, opengl::GL_UNSIGNED_INT, ptr::null());
        }
    }
}
