use crate::bindings::winapi;
use lemao_common_platform::renderer::RendererPlatformSpecific;

pub struct WindowsWinAPIRenderer {
    gl_context: winapi::HGLRC,
}

impl WindowsWinAPIRenderer {
    pub fn new(gl_context: winapi::HGLRC) -> Self {
        Self { gl_context }
    }
}

impl RendererPlatformSpecific for WindowsWinAPIRenderer {
    fn close(&self) {
        unsafe {
            winapi::wglDeleteContext(self.gl_context);
        }
    }
}
