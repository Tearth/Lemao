use lemao_opengl::context::OpenGLContext;
use lemao_winapi::bindings::winapi;
use std::mem;

#[derive(Default)]
pub struct RendererContext {
    pub gl: Option<OpenGLContext>,
}

impl RendererContext {
    pub fn init(&mut self, hwnd: winapi::HWND) {
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

            let device_context: winapi::HDC = winapi::GetDC(hwnd);
            let pixel_format = winapi::ChoosePixelFormat(device_context, &pixel_format_descriptor);
            if winapi::SetPixelFormat(device_context, pixel_format, &pixel_format_descriptor) == 0 {
                panic!("{}", winapi::GetLastError());
            }

            let opengl_context: winapi::HGLRC = winapi::wglCreateContext(device_context);
            if winapi::wglMakeCurrent(device_context, opengl_context) == 0 {
                panic!("{}", winapi::GetLastError());
            }

            self.gl = Some(Default::default());
        }
    }
}
