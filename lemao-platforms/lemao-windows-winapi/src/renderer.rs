use crate::bindings::winapi;
use crate::window::WindowWinAPI;
use lemao_common_platform::renderer::RendererPlatformSpecific;
use lemao_common_platform::window::WindowPlatformSpecific;
use lemao_opengl::bindings::opengl;
use lemao_opengl::bindings::wgl;
use lemao_opengl::pointers::OpenGLPointers;
use std::mem;
use std::ptr;

pub struct WindowsWinAPIRenderer {
    gl_context: winapi::HGLRC,
    gl: OpenGLPointers,
}

impl WindowsWinAPIRenderer {
    pub unsafe fn new(hdc: winapi::HDC) -> Result<WindowsWinAPIRenderer, String> {
        unsafe {
            let fake_window = WindowWinAPI::new_fake()?;
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
                return Err(format!("Error while setting pixel format for fake window, GetLastError()={}", winapi::GetLastError()));
            }

            let fake_gl_context: winapi::HGLRC = winapi::wglCreateContext(fake_window_hdc);
            if winapi::wglMakeCurrent(fake_window_hdc, fake_gl_context) == 0 {
                return Err(format!("Error while creating fake OpenGL context, GetLastError()={}", winapi::GetLastError()));
            }

            let gl: OpenGLPointers = Default::default();

            winapi::wglDeleteContext(fake_gl_context);
            fake_window.close();

            let mut attributes = [
                wgl::WGL_DRAW_TO_WINDOW_ARB,
                opengl::GL_TRUE,
                wgl::WGL_SUPPORT_OPENGL_ARB,
                opengl::GL_TRUE,
                wgl::WGL_DOUBLE_BUFFER_ARB,
                opengl::GL_TRUE,
                wgl::WGL_PIXEL_TYPE_ARB,
                wgl::WGL_TYPE_RGBA_ARB,
                wgl::WGL_COLOR_BITS_ARB,
                32,
                wgl::WGL_DEPTH_BITS_ARB,
                24,
                wgl::WGL_STENCIL_BITS_ARB,
                8,
                wgl::WGL_SAMPLE_BUFFERS_ARB,
                opengl::GL_TRUE,
                wgl::WGL_SAMPLES_ARB,
                16,
                0,
            ];

            let mut pixel_format = 0;
            let mut formats_count = 0;
            let attributes_ptr = attributes.as_mut_ptr() as *const i32;

            if (gl.wglChoosePixelFormatARB)(hdc as wgl::HDC, attributes_ptr, ptr::null_mut(), 1, &mut pixel_format, &mut formats_count) == 0 {
                return Err(format!("Error while loading available pixel formats for desired window, GetLastError()={}", winapi::GetLastError()));
            }

            if winapi::SetPixelFormat(hdc, pixel_format, &pixel_format_descriptor) == 0 {
                return Err(format!("Error while setting pixel format for desired window, GetLastError()={}", winapi::GetLastError()));
            }

            let mut attributes = [wgl::WGL_CONTEXT_MAJOR_VERSION_ARB, 3, wgl::WGL_CONTEXT_MINOR_VERSION_ARB, 3, 0];
            let attributes_ptr = attributes.as_mut_ptr() as *const i32;

            let gl_context = (gl.wglCreateContextAttribsARB)(hdc as wgl::HDC, ptr::null_mut(), attributes_ptr) as winapi::HGLRC;
            if winapi::wglMakeCurrent(hdc, gl_context) == 0 {
                return Err(format!("Error while creating OpenGL context for desired window, GetLastError()={}", winapi::GetLastError()));
            }

            Ok(Self { gl_context, gl })
        }
    }
}

impl RendererPlatformSpecific for WindowsWinAPIRenderer {
    fn set_swap_interval(&self, interval: u32) {
        unsafe {
            (self.gl.wglSwapIntervalEXT)(interval as i32);
        }
    }

    fn close(&self) {
        unsafe {
            winapi::wglDeleteContext(self.gl_context);
        }
    }
}
