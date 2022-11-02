use crate::window::context::WindowContext;

use super::drawable::Drawable;
use super::shaders::Shader;
use super::textures::storage::TextureStorage;
use lemao_math::color::Color;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::{opengl, wgl};
use lemao_opengl::pointers::OpenGLPointers;
use lemao_winapi::bindings::winapi;
use std::rc::Rc;
use std::{mem, ptr};

pub struct RendererContext {
    pub gl: Rc<OpenGLPointers>,
    pub gl_context: winapi::HGLRC,
    pub default_shader: Rc<Shader>,
    pub active_shader: Option<Rc<Shader>>,

    pub textures: TextureStorage,
}

impl RendererContext {
    pub fn new(hdc: winapi::HDC) -> Self {
        unsafe {
            let fake_window = WindowContext::new_fake();
            let fake_window_hdc = fake_window.hdc;

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

            let pixel_format = winapi::ChoosePixelFormat(fake_window_hdc, &pixel_format_descriptor);
            if winapi::SetPixelFormat(fake_window_hdc, pixel_format, &pixel_format_descriptor) == 0 {
                panic!("{}", winapi::GetLastError());
            }

            let fake_gl_context: winapi::HGLRC = winapi::wglCreateContext(fake_window_hdc);
            if winapi::wglMakeCurrent(fake_window_hdc, fake_gl_context) == 0 {
                panic!("{}", winapi::GetLastError());
            }

            let gl: Rc<OpenGLPointers> = Default::default();

            winapi::wglDeleteContext(fake_gl_context);
            fake_window.close();

            #[rustfmt::skip]
            let mut attributes = [
                wgl::WGL_DRAW_TO_WINDOW_ARB, opengl::GL_TRUE,
                wgl::WGL_SUPPORT_OPENGL_ARB, opengl::GL_TRUE,
                wgl::WGL_DOUBLE_BUFFER_ARB, opengl::GL_TRUE,
                wgl::WGL_PIXEL_TYPE_ARB, wgl::WGL_TYPE_RGBA_ARB,
                wgl::WGL_COLOR_BITS_ARB, 32,
                wgl::WGL_DEPTH_BITS_ARB, 24,
                wgl::WGL_STENCIL_BITS_ARB, 8,
                0,
            ];

            let mut pixel_format = 0;
            let mut formats_count = 0;
            let attributes_ptr = attributes.as_mut_ptr() as *const i32;

            if (gl.wglChoosePixelFormatARB)(hdc as wgl::HDC, attributes_ptr, ptr::null_mut(), 1, &mut pixel_format, &mut formats_count) == 0 {
                panic!("{}", winapi::GetLastError());
            }

            if winapi::SetPixelFormat(hdc, pixel_format, &pixel_format_descriptor) == 0 {
                panic!("{}", winapi::GetLastError());
            }

            #[rustfmt::skip]
            let mut attributes = [
                wgl::WGL_CONTEXT_MAJOR_VERSION_ARB, 3, 
                wgl::WGL_CONTEXT_MINOR_VERSION_ARB, 2, 
                0
            ];
            let attributes_ptr = attributes.as_mut_ptr() as *const i32;

            let gl_context = (gl.wglCreateContextAttribsARB)(hdc as wgl::HDC, ptr::null_mut(), attributes_ptr);
            if winapi::wglMakeCurrent(hdc, gl_context as winapi::HGLRC) == 0 {
                panic!("{}", winapi::GetLastError());
            }

            let gl: Rc<OpenGLPointers> = Default::default();
            let default_shader = match Shader::new_default(gl.clone()) {
                Ok(value) => Rc::new(value),
                Err(message) => panic!("Default shader compilation error: {}", message),
            };
            RendererContext { gl, gl_context: gl_context as winapi::HGLRC, default_shader, active_shader: None, textures: TextureStorage::new() }
        }
    }

    pub fn set_viewport(&self, width: i32, height: i32) {
        unsafe {
            (self.gl.glViewport)(0, 0, width, height);
        }
    }

    pub fn set_default_shader(&mut self) {
        self.active_shader = Some(self.default_shader.clone());
        self.default_shader.set_as_active();
    }

    pub fn clear(&self, color: Color) {
        unsafe {
            (self.gl.glClearColor)(color.r, color.g, color.b, color.a);
            (self.gl.glClear)(opengl::GL_COLOR_BUFFER_BIT);
        }
    }

    pub fn draw(&self, drawable: &dyn Drawable) {
        let view = Mat4x4::translate(Vec3::new(0.0, 0.0, -3.0));
        let proj = Mat4x4::ortho(800.0, 600.0, 0.1, 100.0);

        self.active_shader.as_ref().unwrap().set_parameter("view", view.as_ptr());
        self.active_shader.as_ref().unwrap().set_parameter("proj", proj.as_ptr());

        drawable.draw(self.active_shader.as_ref().unwrap());
    }

    pub fn release(&self) {
        unsafe {
            winapi::wglDeleteContext(self.gl_context);
        }
    }
}
