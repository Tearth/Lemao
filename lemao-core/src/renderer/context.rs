use super::shaders;
use super::sprite::Sprite;
use super::textures::Texture;
use lemao_math::color::Color;
use lemao_opengl::bindings::opengl;
use lemao_opengl::context::OpenGLContext;
use lemao_winapi::bindings::winapi;
use std::mem;

pub struct RendererContext {
    pub gl: OpenGLContext,
    pub gl_context: winapi::HGLRC,
    pub default_shader_program: u32,
}

impl RendererContext {
    pub fn new(hdc: winapi::HDC) -> Self {
        unsafe {
            let pixel_format_descriptor = winapi::PIXELFORMATDESCRIPTOR {
                nSize: mem::size_of::<winapi::PIXELFORMATDESCRIPTOR>() as u16,
                nVersion: 1,
                dwFlags: winapi::PFD_DRAW_TO_WINDOW | winapi::PFD_SUPPORT_OPENGL | winapi::PFD_DOUBLEBUFFER,
                iPixelType: winapi::PFD_TYPE_RGBA as u8,
                cColorBits: 32,
                cRedBits: 0,
                cRedShift: 0,
                cGreenBits: 0,
                cGreenShift: 0,
                cBlueBits: 0,
                cBlueShift: 0,
                cAlphaBits: 0,
                cAlphaShift: 0,
                cAccumBits: 0,
                cAccumRedBits: 0,
                cAccumGreenBits: 0,
                cAccumBlueBits: 0,
                cAccumAlphaBits: 0,
                cDepthBits: 24,
                cStencilBits: 8,
                cAuxBuffers: 0,
                iLayerType: winapi::PFD_MAIN_PLANE as u8,
                bReserved: 0,
                dwLayerMask: 0,
                dwVisibleMask: 0,
                dwDamageMask: 0,
            };

            let pixel_format = winapi::ChoosePixelFormat(hdc, &pixel_format_descriptor);
            if winapi::SetPixelFormat(hdc, pixel_format, &pixel_format_descriptor) == 0 {
                panic!("{}", winapi::GetLastError());
            }

            let gl_context: winapi::HGLRC = winapi::wglCreateContext(hdc);
            if winapi::wglMakeCurrent(hdc, gl_context) == 0 {
                panic!("{}", winapi::GetLastError());
            }

            let gl = Default::default();
            let default_shader_program = match shaders::load_default(&gl) {
                Ok(value) => value,
                Err(message) => panic!("Default shader compilation error: {}", message),
            };
            (gl.glUseProgram)(default_shader_program);

            RendererContext { gl, gl_context, default_shader_program }
        }
    }

    pub fn set_viewport(&self, width: i32, height: i32) {
        unsafe {
            (self.gl.glViewport)(0, 0, width, height);
        }
    }

    pub fn clear(&self, color: Color) {
        unsafe {
            (self.gl.glClearColor)(color.r, color.g, color.b, color.a);
            (self.gl.glClear)(opengl::GL_COLOR_BUFFER_BIT);
        }
    }

    pub fn create_sprite(&self, loaded_texture: &Texture) -> Sprite {
        Sprite::new(&self.gl, loaded_texture)
    }

    pub fn draw(&self, sprite: &Sprite) {
        sprite.draw(&self.gl);
    }

    pub fn release(&self) {
        unsafe {
            winapi::wglDeleteContext(self.gl_context);
        }
    }
}
