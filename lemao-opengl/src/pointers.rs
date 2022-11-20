use crate::bindings::opengl;
use crate::bindings::wgl;
use lemao_winapi::bindings::winapi;
use std::ffi::CString;
use std::mem;

#[allow(non_snake_case)]
pub struct OpenGLPointers {
    pub glAttachShader: opengl::PFNGLATTACHSHADERPROC,
    pub glBindBuffer: opengl::PFNGLBINDBUFFERPROC,
    pub glBindTexture: opengl::PFNGLBINDTEXTUREPROC,
    pub glBindVertexArray: opengl::PFNGLBINDVERTEXARRAYPROC,
    pub glBlendFunc: opengl::PFNGLBLENDFUNCPROC,
    pub glBufferData: opengl::PFNGLBUFFERDATAPROC,
    pub glClear: opengl::PFNGLCLEARPROC,
    pub glClearColor: opengl::PFNGLCLEARCOLORPROC,
    pub glCompileShader: opengl::PFNGLCOMPILESHADERPROC,
    pub glCreateProgram: opengl::PFNGLCREATEPROGRAMPROC,
    pub glCreateShader: opengl::PFNGLCREATESHADERPROC,
    pub glDebugMessageCallback: opengl::PFNGLDEBUGMESSAGECALLBACKPROC,
    pub glDeleteBuffers: opengl::PFNGLDELETEBUFFERSPROC,
    pub glDeleteProgram: opengl::PFNGLDELETEPROGRAMPROC,
    pub glDeleteTextures: opengl::PFNGLDELETETEXTURESPROC,
    pub glDeleteShader: opengl::PFNGLDELETESHADERPROC,
    pub glDeleteVertexArrays: opengl::PFNGLDELETEVERTEXARRAYSPROC,
    pub glDrawArrays: opengl::PFNGLDRAWARRAYSPROC,
    pub glDrawElements: opengl::PFNGLDRAWELEMENTSPROC,
    pub glEnable: opengl::PFNGLENABLEPROC,
    pub glEnableVertexAttribArray: opengl::PFNGLENABLEVERTEXATTRIBARRAYPROC,
    pub glGenBuffers: opengl::PFNGLGENBUFFERSPROC,
    pub glGenerateMipmap: opengl::PFNGLGENERATEMIPMAPPROC,
    pub glGenTextures: opengl::PFNGLGENTEXTURESPROC,
    pub glGenVertexArrays: opengl::PFNGLGENVERTEXARRAYSPROC,
    pub glGetActiveUniform: opengl::PFNGLGETACTIVEUNIFORMPROC,
    pub glGetError: opengl::PFNGLGETERRORPROC,
    pub glGetProgramiv: opengl::PFNGLGETPROGRAMIVPROC,
    pub glGetProgramInfoLog: opengl::PFNGLGETPROGRAMINFOLOGPROC,
    pub glGetShaderInfoLog: opengl::PFNGLGETSHADERINFOLOGPROC,
    pub glGetShaderiv: opengl::PFNGLGETSHADERIVPROC,
    pub glGetString: opengl::PFNGLGETSTRINGPROC,
    pub glGetUniformLocation: opengl::PFNGLGETUNIFORMLOCATIONPROC,
    pub glLinkProgram: opengl::PFNGLLINKPROGRAMPROC,
    pub glShaderSource: opengl::PFNGLSHADERSOURCEPROC,
    pub glTexImage2D: opengl::PFNGLTEXIMAGE2DPROC,
    pub glTexParameteri: opengl::PFNGLTEXPARAMETERIPROC,
    pub glUniformMatrix4fv: opengl::PFNGLUNIFORMMATRIX4FVPROC,
    pub glUniform4fv: opengl::PFNGLUNIFORM4FVPROC,
    pub glUseProgram: opengl::PFNGLUSEPROGRAMPROC,
    pub glVertexAttribPointer: opengl::PFNGLVERTEXATTRIBPOINTERPROC,
    pub glViewport: opengl::PFNGLVIEWPORTPROC,

    pub wglChoosePixelFormatARB: wgl::PFNWGLCHOOSEPIXELFORMATARBPROC,
    pub wglCreateContextAttribsARB: wgl::PFNWGLCREATECONTEXTATTRIBSARBPROC,
}

impl Default for OpenGLPointers {
    fn default() -> Self {
        unsafe {
            let opengl32_dll_cstr = CString::new("opengl32.dll").unwrap();
            let opengl32_dll_handle = winapi::LoadLibraryA(opengl32_dll_cstr.as_ptr());

            Self {
                glAttachShader: get_wgl_proc_address::<opengl::PFNGLATTACHSHADERPROC>("glAttachShader"),
                glBindBuffer: get_wgl_proc_address::<opengl::PFNGLBINDBUFFERPROC>("glBindBuffer"),
                glBindTexture: get_proc_address::<opengl::PFNGLBINDTEXTUREPROC>("glBindTexture", opengl32_dll_handle),
                glBindVertexArray: get_wgl_proc_address::<opengl::PFNGLBINDVERTEXARRAYPROC>("glBindVertexArray"),
                glBlendFunc: get_proc_address::<opengl::PFNGLBLENDFUNCPROC>("glBlendFunc", opengl32_dll_handle),
                glBufferData: get_wgl_proc_address::<opengl::PFNGLBUFFERDATAPROC>("glBufferData"),
                glClear: get_proc_address::<opengl::PFNGLCLEARPROC>("glClear", opengl32_dll_handle),
                glClearColor: get_proc_address::<opengl::PFNGLCLEARCOLORPROC>("glClearColor", opengl32_dll_handle),
                glCreateProgram: get_wgl_proc_address::<opengl::PFNGLCREATEPROGRAMPROC>("glCreateProgram"),
                glCompileShader: get_wgl_proc_address::<opengl::PFNGLCOMPILESHADERPROC>("glCompileShader"),
                glCreateShader: get_wgl_proc_address::<opengl::PFNGLCREATESHADERPROC>("glCreateShader"),
                glDebugMessageCallback: get_wgl_proc_address::<opengl::PFNGLDEBUGMESSAGECALLBACKPROC>("glDebugMessageCallback"),
                glDeleteBuffers: get_wgl_proc_address::<opengl::PFNGLDELETEBUFFERSPROC>("glDeleteBuffers"),
                glDeleteProgram: get_wgl_proc_address::<opengl::PFNGLDELETEPROGRAMPROC>("glDeleteProgram"),
                glDeleteTextures: get_proc_address::<opengl::PFNGLDELETETEXTURESPROC>("glDeleteTextures", opengl32_dll_handle),
                glDeleteShader: get_wgl_proc_address::<opengl::PFNGLDELETESHADERPROC>("glDeleteShader"),
                glDeleteVertexArrays: get_wgl_proc_address::<opengl::PFNGLDELETEVERTEXARRAYSPROC>("glDeleteVertexArrays"),
                glDrawArrays: get_proc_address::<opengl::PFNGLDRAWARRAYSPROC>("glDrawArrays", opengl32_dll_handle),
                glDrawElements: get_proc_address::<opengl::PFNGLDRAWELEMENTSPROC>("glDrawElements", opengl32_dll_handle),
                glEnable: get_proc_address::<opengl::PFNGLENABLEPROC>("glEnable", opengl32_dll_handle),
                glEnableVertexAttribArray: get_wgl_proc_address::<opengl::PFNGLENABLEVERTEXATTRIBARRAYPROC>("glEnableVertexAttribArray"),
                glGenBuffers: get_wgl_proc_address::<opengl::PFNGLGENBUFFERSPROC>("glGenBuffers"),
                glGenerateMipmap: get_wgl_proc_address::<opengl::PFNGLGENERATEMIPMAPPROC>("glGenerateMipmap"),
                glGenTextures: get_proc_address::<opengl::PFNGLGENTEXTURESPROC>("glGenTextures", opengl32_dll_handle),
                glGenVertexArrays: get_wgl_proc_address::<opengl::PFNGLGENBUFFERSPROC>("glGenVertexArrays"),
                glGetActiveUniform: get_wgl_proc_address::<opengl::PFNGLGETACTIVEUNIFORMPROC>("glGetActiveUniform"),
                glGetError: get_proc_address::<opengl::PFNGLGETERRORPROC>("glGetError", opengl32_dll_handle),
                glGetProgramiv: get_wgl_proc_address::<opengl::PFNGLGETPROGRAMIVPROC>("glGetProgramiv"),
                glGetProgramInfoLog: get_wgl_proc_address::<opengl::PFNGLGETPROGRAMINFOLOGPROC>("glGetProgramInfoLog"),
                glGetShaderInfoLog: get_wgl_proc_address::<opengl::PFNGLGETSHADERINFOLOGPROC>("glGetShaderInfoLog"),
                glGetShaderiv: get_wgl_proc_address::<opengl::PFNGLGETSHADERIVPROC>("glGetShaderiv"),
                glGetString: get_proc_address::<opengl::PFNGLGETSTRINGPROC>("glGetString", opengl32_dll_handle),
                glGetUniformLocation: get_wgl_proc_address::<opengl::PFNGLGETUNIFORMLOCATIONPROC>("glGetUniformLocation"),
                glLinkProgram: get_wgl_proc_address::<opengl::PFNGLLINKPROGRAMPROC>("glLinkProgram"),
                glShaderSource: get_wgl_proc_address::<opengl::PFNGLSHADERSOURCEPROC>("glShaderSource"),
                glTexImage2D: get_proc_address::<opengl::PFNGLTEXIMAGE2DPROC>("glTexImage2D", opengl32_dll_handle),
                glTexParameteri: get_proc_address::<opengl::PFNGLTEXPARAMETERIPROC>("glTexParameteri", opengl32_dll_handle),
                glUniformMatrix4fv: get_wgl_proc_address::<opengl::PFNGLUNIFORMMATRIX4FVPROC>("glUniformMatrix4fv"),
                glUniform4fv: get_wgl_proc_address::<opengl::PFNGLUNIFORM4FVPROC>("glUniform4fv"),
                glUseProgram: get_wgl_proc_address::<opengl::PFNGLUSEPROGRAMPROC>("glUseProgram"),
                glVertexAttribPointer: get_wgl_proc_address::<opengl::PFNGLVERTEXATTRIBPOINTERPROC>("glVertexAttribPointer"),
                glViewport: get_proc_address::<opengl::PFNGLVIEWPORTPROC>("glViewport", opengl32_dll_handle),

                wglChoosePixelFormatARB: get_wgl_proc_address::<wgl::PFNWGLCHOOSEPIXELFORMATARBPROC>("wglChoosePixelFormatARB"),
                wglCreateContextAttribsARB: get_wgl_proc_address::<wgl::PFNWGLCREATECONTEXTATTRIBSARBPROC>("wglCreateContextAttribsARB"),
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
