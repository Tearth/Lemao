use crate::bindings::opengl;
use lemao_winapi::bindings::winapi;
use std::ffi::CString;
use std::mem;

#[allow(non_snake_case)]
pub struct OpenGLContext {
    pub glClear: opengl::PFNGLCLEARPROC,
    pub glClearColor: opengl::PFNGLCLEARCOLORPROC,
    pub glGetString: opengl::PFNGLGETSTRINGPROC,
    pub glViewport: opengl::PFNGLVIEWPORTPROC,
    pub glCreateProgram: opengl::PFNGLCREATEPROGRAMPROC,
}

impl Default for OpenGLContext {
    fn default() -> Self {
        unsafe {
            let opengl32_dll_cstr = CString::new("opengl32.dll").unwrap();
            let opengl32_dll_handle = winapi::LoadLibraryA(opengl32_dll_cstr.as_ptr());

            Self {
                glClear: get_proc_address::<opengl::PFNGLCLEARPROC>("glClear", opengl32_dll_handle),
                glClearColor: get_proc_address::<opengl::PFNGLCLEARCOLORPROC>("glClearColor", opengl32_dll_handle),
                glGetString: get_proc_address::<opengl::PFNGLGETSTRINGPROC>("glGetString", opengl32_dll_handle),
                glViewport: get_proc_address::<opengl::PFNGLVIEWPORTPROC>("glViewport", opengl32_dll_handle),
                glCreateProgram: get_wgl_proc_address::<opengl::PFNGLCREATEPROGRAMPROC>("glCreateProgram"),
            }
        }
    }
}

fn get_proc_address<T>(name: &str, dll_handle: *mut winapi::HINSTANCE__) -> T {
    unsafe {
        let function_cstr = CString::new(name).unwrap();
        let function_handle = winapi::GetProcAddress(dll_handle, function_cstr.as_ptr());
        mem::transmute_copy::<winapi::FARPROC, T>(&function_handle)
    }
}

fn get_wgl_proc_address<T>(name: &str) -> T {
    unsafe {
        let function_cstr = CString::new(name).unwrap();
        let function_handle = winapi::wglGetProcAddress(function_cstr.as_ptr());
        mem::transmute_copy::<winapi::PROC, T>(&function_handle)
    }
}
