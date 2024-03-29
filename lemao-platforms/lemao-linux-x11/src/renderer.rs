use crate::bindings::x11;
use lemao_common_platform::renderer::RendererPlatformSpecific;
use lemao_opengl::bindings::glx::{self, __GLXcontextRec};
use lemao_opengl::pointers::OpenGLPointers;
use std::{mem, ptr};

pub struct LinuxX11Renderer {
    display: *mut x11::_XDisplay,
    window: u64,
    gl: OpenGLPointers,
    gl_context: *mut __GLXcontextRec,
}

impl LinuxX11Renderer {
    pub unsafe fn new(display: *mut x11::_XDisplay, frame_buffer_config: *mut glx::__GLXFBConfigRec, window: u64) -> Self {
        unsafe {
            let context_attributes = [
                glx::GLX_CONTEXT_MAJOR_VERSION_ARB,
                3,
                glx::GLX_CONTEXT_MINOR_VERSION_ARB,
                3,
                glx::GLX_CONTEXT_FLAGS_ARB,
                glx::GLX_CONTEXT_FORWARD_COMPATIBLE_BIT_ARB,
                0,
            ];
            let context_attributes_ptr = context_attributes.as_ptr() as *const i32;

            let gl: OpenGLPointers = Default::default();
            let gl_context = (gl.glXCreateContextAttribsARB)(mem::transmute(display), frame_buffer_config, ptr::null_mut(), 1, context_attributes_ptr);

            x11::XSync(display, 0);
            glx::glXMakeCurrent(mem::transmute(display), window, gl_context);

            Self { display, window, gl, gl_context }
        }
    }
}

impl RendererPlatformSpecific for LinuxX11Renderer {
    fn set_swap_interval(&self, interval: u32) {
        unsafe { (self.gl.glXSwapIntervalEXT)(mem::transmute(self.display), self.window, interval as i32) }
    }

    fn close(&self) {
        unsafe { glx::glXDestroyContext(mem::transmute(self.display), self.gl_context) }
    }
}
