use crate::bindings::opengl;
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
    pub glBufferSubData: opengl::PFNGLBUFFERSUBDATAPROC,
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
    pub glDisable: opengl::PFNGLDISABLEPROC,
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
    pub glScissor: opengl::PFNGLSCISSORPROC,
    pub glShaderSource: opengl::PFNGLSHADERSOURCEPROC,
    pub glTexImage2D: opengl::PFNGLTEXIMAGE2DPROC,
    pub glTexParameteri: opengl::PFNGLTEXPARAMETERIPROC,
    pub glUniformMatrix4fv: opengl::PFNGLUNIFORMMATRIX4FVPROC,
    pub glUniform1f: opengl::PFNGLUNIFORM1FPROC,
    pub glUniform1i: opengl::PFNGLUNIFORM1IPROC,
    pub glUniform2fv: opengl::PFNGLUNIFORM2FVPROC,
    pub glUniform4fv: opengl::PFNGLUNIFORM4FVPROC,
    pub glUseProgram: opengl::PFNGLUSEPROGRAMPROC,
    pub glVertexAttribPointer: opengl::PFNGLVERTEXATTRIBPOINTERPROC,
    pub glViewport: opengl::PFNGLVIEWPORTPROC,

    #[cfg(windows)]
    pub wglChoosePixelFormatARB: crate::bindings::wgl::PFNWGLCHOOSEPIXELFORMATARBPROC,

    #[cfg(windows)]
    pub wglCreateContextAttribsARB: crate::bindings::wgl::PFNWGLCREATECONTEXTATTRIBSARBPROC,

    #[cfg(windows)]
    pub wglSwapIntervalEXT: crate::bindings::wgl::PFNWGLSWAPINTERVALEXTPROC,

    #[cfg(unix)]
    pub glXCreateContextAttribsARB: crate::bindings::glx::PFNGLXCREATECONTEXTATTRIBSARBPROC,

    #[cfg(unix)]
    pub glXSwapIntervalEXT: crate::bindings::glx::PFNGLXSWAPINTERVALEXTPROC,
}

impl Default for OpenGLPointers {
    fn default() -> Self {
        Self {
            glAttachShader: get_proc_address::<opengl::PFNGLATTACHSHADERPROC>("glAttachShader"),
            glBindBuffer: get_proc_address::<opengl::PFNGLBINDBUFFERPROC>("glBindBuffer"),
            glBindTexture: get_proc_address::<opengl::PFNGLBINDTEXTUREPROC>("glBindTexture"),
            glBindVertexArray: get_proc_address::<opengl::PFNGLBINDVERTEXARRAYPROC>("glBindVertexArray"),
            glBlendFunc: get_proc_address::<opengl::PFNGLBLENDFUNCPROC>("glBlendFunc"),
            glBufferData: get_proc_address::<opengl::PFNGLBUFFERDATAPROC>("glBufferData"),
            glBufferSubData: get_proc_address::<opengl::PFNGLBUFFERSUBDATAPROC>("glBufferSubData"),
            glClear: get_proc_address::<opengl::PFNGLCLEARPROC>("glClear"),
            glClearColor: get_proc_address::<opengl::PFNGLCLEARCOLORPROC>("glClearColor"),
            glCreateProgram: get_proc_address::<opengl::PFNGLCREATEPROGRAMPROC>("glCreateProgram"),
            glCompileShader: get_proc_address::<opengl::PFNGLCOMPILESHADERPROC>("glCompileShader"),
            glCreateShader: get_proc_address::<opengl::PFNGLCREATESHADERPROC>("glCreateShader"),
            glDebugMessageCallback: get_proc_address::<opengl::PFNGLDEBUGMESSAGECALLBACKPROC>("glDebugMessageCallback"),
            glDeleteBuffers: get_proc_address::<opengl::PFNGLDELETEBUFFERSPROC>("glDeleteBuffers"),
            glDeleteProgram: get_proc_address::<opengl::PFNGLDELETEPROGRAMPROC>("glDeleteProgram"),
            glDeleteTextures: get_proc_address::<opengl::PFNGLDELETETEXTURESPROC>("glDeleteTextures"),
            glDeleteShader: get_proc_address::<opengl::PFNGLDELETESHADERPROC>("glDeleteShader"),
            glDeleteVertexArrays: get_proc_address::<opengl::PFNGLDELETEVERTEXARRAYSPROC>("glDeleteVertexArrays"),
            glDisable: get_proc_address::<opengl::PFNGLDISABLEPROC>("glDisable"),
            glDrawArrays: get_proc_address::<opengl::PFNGLDRAWARRAYSPROC>("glDrawArrays"),
            glDrawElements: get_proc_address::<opengl::PFNGLDRAWELEMENTSPROC>("glDrawElements"),
            glEnable: get_proc_address::<opengl::PFNGLENABLEPROC>("glEnable"),
            glEnableVertexAttribArray: get_proc_address::<opengl::PFNGLENABLEVERTEXATTRIBARRAYPROC>("glEnableVertexAttribArray"),
            glGenBuffers: get_proc_address::<opengl::PFNGLGENBUFFERSPROC>("glGenBuffers"),
            glGenerateMipmap: get_proc_address::<opengl::PFNGLGENERATEMIPMAPPROC>("glGenerateMipmap"),
            glGenTextures: get_proc_address::<opengl::PFNGLGENTEXTURESPROC>("glGenTextures"),
            glGenVertexArrays: get_proc_address::<opengl::PFNGLGENBUFFERSPROC>("glGenVertexArrays"),
            glGetActiveUniform: get_proc_address::<opengl::PFNGLGETACTIVEUNIFORMPROC>("glGetActiveUniform"),
            glGetError: get_proc_address::<opengl::PFNGLGETERRORPROC>("glGetError"),
            glGetProgramiv: get_proc_address::<opengl::PFNGLGETPROGRAMIVPROC>("glGetProgramiv"),
            glGetProgramInfoLog: get_proc_address::<opengl::PFNGLGETPROGRAMINFOLOGPROC>("glGetProgramInfoLog"),
            glGetShaderInfoLog: get_proc_address::<opengl::PFNGLGETSHADERINFOLOGPROC>("glGetShaderInfoLog"),
            glGetShaderiv: get_proc_address::<opengl::PFNGLGETSHADERIVPROC>("glGetShaderiv"),
            glGetString: get_proc_address::<opengl::PFNGLGETSTRINGPROC>("glGetString"),
            glGetUniformLocation: get_proc_address::<opengl::PFNGLGETUNIFORMLOCATIONPROC>("glGetUniformLocation"),
            glLinkProgram: get_proc_address::<opengl::PFNGLLINKPROGRAMPROC>("glLinkProgram"),
            glScissor: get_proc_address::<opengl::PFNGLSCISSORPROC>("glScissor"),
            glShaderSource: get_proc_address::<opengl::PFNGLSHADERSOURCEPROC>("glShaderSource"),
            glTexImage2D: get_proc_address::<opengl::PFNGLTEXIMAGE2DPROC>("glTexImage2D"),
            glTexParameteri: get_proc_address::<opengl::PFNGLTEXPARAMETERIPROC>("glTexParameteri"),
            glUniformMatrix4fv: get_proc_address::<opengl::PFNGLUNIFORMMATRIX4FVPROC>("glUniformMatrix4fv"),
            glUniform1f: get_proc_address::<opengl::PFNGLUNIFORM1FPROC>("glUniform1f"),
            glUniform1i: get_proc_address::<opengl::PFNGLUNIFORM1IPROC>("glUniform1i"),
            glUniform2fv: get_proc_address::<opengl::PFNGLUNIFORM2FVPROC>("glUniform2fv"),
            glUniform4fv: get_proc_address::<opengl::PFNGLUNIFORM4FVPROC>("glUniform4fv"),
            glUseProgram: get_proc_address::<opengl::PFNGLUSEPROGRAMPROC>("glUseProgram"),
            glVertexAttribPointer: get_proc_address::<opengl::PFNGLVERTEXATTRIBPOINTERPROC>("glVertexAttribPointer"),
            glViewport: get_proc_address::<opengl::PFNGLVIEWPORTPROC>("glViewport"),

            #[cfg(windows)]
            wglChoosePixelFormatARB: get_proc_address::<crate::bindings::wgl::PFNWGLCHOOSEPIXELFORMATARBPROC>("wglChoosePixelFormatARB"),

            #[cfg(windows)]
            wglCreateContextAttribsARB: get_proc_address::<crate::bindings::wgl::PFNWGLCREATECONTEXTATTRIBSARBPROC>("wglCreateContextAttribsARB"),

            #[cfg(windows)]
            wglSwapIntervalEXT: get_proc_address::<crate::bindings::wgl::PFNWGLSWAPINTERVALEXTPROC>("wglSwapIntervalEXT"),

            #[cfg(unix)]
            glXCreateContextAttribsARB: get_proc_address::<crate::bindings::glx::PFNGLXCREATECONTEXTATTRIBSARBPROC>("glXCreateContextAttribsARB"),

            #[cfg(unix)]
            glXSwapIntervalEXT: get_proc_address::<crate::bindings::glx::PFNGLXSWAPINTERVALEXTPROC>("glXSwapIntervalEXT"),
        }
    }
}

#[cfg(windows)]
fn get_proc_address<T>(name: &str) -> T {
    use crate::bindings::winapi;

    unsafe {
        let opengl32_dll_functions = [
            "glBindTexture",
            "glBlendFunc",
            "glClear",
            "glClearColor",
            "glDeleteTextures",
            "glDisable",
            "glDrawArrays",
            "glDrawElements",
            "glEnable",
            "glGenTextures",
            "glGetError",
            "glScissor",
            "glTexImage2D",
            "glTexParameteri",
            "glViewport",
        ];

        if opengl32_dll_functions.contains(&name) {
            let opengl32_dll_cstr = CString::new("opengl32.dll").unwrap();
            let opengl32_dll_handle = winapi::LoadLibraryA(opengl32_dll_cstr.as_ptr());

            let function_cstr = CString::new(name).unwrap();
            let function_handle = winapi::GetProcAddress(opengl32_dll_handle, function_cstr.as_ptr());

            mem::transmute_copy::<winapi::FARPROC, T>(&function_handle)
        } else {
            let function_cstr = CString::new(name).unwrap();
            let function_handle = winapi::wglGetProcAddress(function_cstr.as_ptr());

            mem::transmute_copy::<winapi::PROC, T>(&function_handle)
        }
    }
}

#[cfg(unix)]
fn get_proc_address<T>(name: &str) -> T {
    unsafe {
        let function_cstr = CString::new(name).unwrap();
        let function_handle = crate::bindings::glx::glXGetProcAddressARB(function_cstr.as_ptr() as *const u8);
        mem::transmute_copy::<crate::bindings::glx::__GLXextFuncPtr, T>(&function_handle)
    }
}
