use lemao_opengl::context::OpenGLContext;
use lemao_winapi::bindings::winapi;
use std::mem;

#[derive(Default)]
pub struct RendererContext {
    pub gl: Option<OpenGLContext>,
    pub gl_context: Option<winapi::HGLRC>,
}

impl RendererContext {
    pub fn init(&mut self, hdc: winapi::HDC) {
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

            self.gl = Some(Default::default());
            self.gl_context = Some(gl_context);
        }
    }

    pub fn release(&self) {
        unsafe {
            winapi::wglDeleteContext(self.gl_context.unwrap());
        }
    }
}
