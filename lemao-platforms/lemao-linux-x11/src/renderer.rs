use crate::bindings::x11;
use lemao_common_platform::renderer::RendererPlatformSpecific;
use lemao_opengl::bindings::glx;
use lemao_opengl::pointers::OpenGLPointers;
use std::rc::Rc;
use std::{mem, ptr};

pub struct LinuxX11Renderer {}

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

            let gl: Rc<OpenGLPointers> = Default::default();
            let context = (gl.glXCreateContextAttribsARB)(mem::transmute(display), frame_buffer_config, ptr::null_mut(), 1, context_attributes_ptr);

            x11::XSync(display, 0);
            glx::glXMakeCurrent(mem::transmute(display), window, context);

            Self {}
        }
    }
}

impl RendererPlatformSpecific for LinuxX11Renderer {
    fn close(&self) {
        // unsafe {}
    }
}
