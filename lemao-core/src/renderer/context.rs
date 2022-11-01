use super::drawable::Drawable;
use super::shaders::Shader;
use lemao_math::color::Color;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use lemao_winapi::bindings::winapi;
use std::mem;
use std::rc::Rc;

pub struct RendererContext {
    pub gl: Rc<OpenGLPointers>,
    pub gl_context: winapi::HGLRC,
    pub default_shader: Rc<Shader>,
    pub active_shader: Option<Rc<Shader>>,
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

            let gl: Rc<OpenGLPointers> = Default::default();
            let default_shader = match Shader::new_default(gl.clone()) {
                Ok(value) => Rc::new(value),
                Err(message) => panic!("Default shader compilation error: {}", message),
            };

            RendererContext { gl, gl_context, default_shader, active_shader: None }
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
