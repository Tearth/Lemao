use crate::bindings::opengl;
use lemao_winapi::bindings::winapi;
use std::ffi::CString;
use std::mem;

#[allow(non_snake_case)]
pub struct OpenGLContext {
    pub glAttachShader: opengl::PFNGLATTACHSHADERPROC,
    pub glBindBuffer: opengl::PFNGLBINDBUFFERPROC,
    pub glBindVertexArray: opengl::PFNGLBINDVERTEXARRAYPROC,
    pub glBufferData: opengl::PFNGLBUFFERDATAPROC,
    pub glClear: opengl::PFNGLCLEARPROC,
    pub glClearColor: opengl::PFNGLCLEARCOLORPROC,
    pub glCompileShader: opengl::PFNGLCOMPILESHADERPROC,
    pub glCreateProgram: opengl::PFNGLCREATEPROGRAMPROC,
    pub glCreateShader: opengl::PFNGLCREATESHADERPROC,
    pub glDeleteShader: opengl::PFNGLDELETESHADERPROC,
    pub glDrawArrays: opengl::PFNGLDRAWARRAYSPROC,
    pub glDrawElements: opengl::PFNGLDRAWELEMENTSPROC,
    pub glEnableVertexAttribArray: opengl::PFNGLENABLEVERTEXATTRIBARRAYPROC,
    pub glGenBuffers: opengl::PFNGLGENBUFFERSPROC,
    pub glGenVertexArrays: opengl::PFNGLGENVERTEXARRAYSPROC,
    pub glGetProgramiv: opengl::PFNGLGETPROGRAMIVPROC,
    pub glGetProgramInfoLog: opengl::PFNGLGETPROGRAMINFOLOGPROC,
    pub glGetShaderInfoLog: opengl::PFNGLGETSHADERINFOLOGPROC,
    pub glGetShaderiv: opengl::PFNGLGETSHADERIVPROC,
    pub glGetString: opengl::PFNGLGETSTRINGPROC,
    pub glLinkProgram: opengl::PFNGLLINKPROGRAMPROC,
    pub glShaderSource: opengl::PFNGLSHADERSOURCEPROC,
    pub glUseProgram: opengl::PFNGLUSEPROGRAMPROC,
    pub glVertexAttribPointer: opengl::PFNGLVERTEXATTRIBPOINTERPROC,
    pub glViewport: opengl::PFNGLVIEWPORTPROC,
}

impl Default for OpenGLContext {
    fn default() -> Self {
        unsafe {
            let opengl32_dll_cstr = CString::new("opengl32.dll").unwrap();
            let opengl32_dll_handle = winapi::LoadLibraryA(opengl32_dll_cstr.as_ptr());

            Self {
                glAttachShader: get_wgl_proc_address::<opengl::PFNGLATTACHSHADERPROC>("glAttachShader"),
                glBindBuffer: get_wgl_proc_address::<opengl::PFNGLBINDBUFFERPROC>("glBindBuffer"),
                glBindVertexArray: get_wgl_proc_address::<opengl::PFNGLBINDVERTEXARRAYPROC>("glBindVertexArray"),
                glBufferData: get_wgl_proc_address::<opengl::PFNGLBUFFERDATAPROC>("glBufferData"),
                glClear: get_proc_address::<opengl::PFNGLCLEARPROC>("glClear", opengl32_dll_handle),
                glClearColor: get_proc_address::<opengl::PFNGLCLEARCOLORPROC>("glClearColor", opengl32_dll_handle),
                glCreateProgram: get_wgl_proc_address::<opengl::PFNGLCREATEPROGRAMPROC>("glCreateProgram"),
                glCompileShader: get_wgl_proc_address::<opengl::PFNGLCOMPILESHADERPROC>("glCompileShader"),
                glCreateShader: get_wgl_proc_address::<opengl::PFNGLCREATESHADERPROC>("glCreateShader"),
                glDeleteShader: get_wgl_proc_address::<opengl::PFNGLDELETESHADERPROC>("glDeleteShader"),
                glDrawArrays: get_proc_address::<opengl::PFNGLDRAWARRAYSPROC>("glDrawArrays", opengl32_dll_handle),
                glDrawElements: get_proc_address::<opengl::PFNGLDRAWELEMENTSPROC>("glDrawElements", opengl32_dll_handle),
                glEnableVertexAttribArray: get_wgl_proc_address::<opengl::PFNGLENABLEVERTEXATTRIBARRAYPROC>("glEnableVertexAttribArray"),
                glGenBuffers: get_wgl_proc_address::<opengl::PFNGLGENBUFFERSPROC>("glGenBuffers"),
                glGenVertexArrays: get_wgl_proc_address::<opengl::PFNGLGENBUFFERSPROC>("glGenVertexArrays"),
                glGetProgramiv: get_wgl_proc_address::<opengl::PFNGLGETPROGRAMIVPROC>("glGetProgramiv"),
                glGetProgramInfoLog: get_wgl_proc_address::<opengl::PFNGLGETPROGRAMINFOLOGPROC>("glGetProgramInfoLog"),
                glGetShaderInfoLog: get_wgl_proc_address::<opengl::PFNGLGETSHADERINFOLOGPROC>("glGetShaderInfoLog"),
                glGetShaderiv: get_wgl_proc_address::<opengl::PFNGLGETSHADERIVPROC>("glGetShaderiv"),
                glGetString: get_proc_address::<opengl::PFNGLGETSTRINGPROC>("glGetString", opengl32_dll_handle),
                glLinkProgram: get_wgl_proc_address::<opengl::PFNGLLINKPROGRAMPROC>("glLinkProgram"),
                glShaderSource: get_wgl_proc_address::<opengl::PFNGLSHADERSOURCEPROC>("glShaderSource"),
                glUseProgram: get_wgl_proc_address::<opengl::PFNGLUSEPROGRAMPROC>("glUseProgram"),
                glVertexAttribPointer: get_wgl_proc_address::<opengl::PFNGLVERTEXATTRIBPOINTERPROC>("glVertexAttribPointer"),
                glViewport: get_proc_address::<opengl::PFNGLVIEWPORTPROC>("glViewport", opengl32_dll_handle),
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
