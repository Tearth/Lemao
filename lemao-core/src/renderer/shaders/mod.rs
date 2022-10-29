use lemao_opengl::bindings::opengl;
use lemao_opengl::context::OpenGLContext;
use std::ffi::CString;
use std::ptr;

pub const ERROR_LENGTH: usize = 1024;
pub const DEFAULT_VERTEX_SHADER: &str = include_str!("./default.vert");
pub const DEFAULT_FRAGMENT_SHADER: &str = include_str!("./default.frag");

pub fn load(gl: &OpenGLContext, vertex_shader: &str, fragment_shader: &str) -> Result<u32, String> {
    unsafe {
        let mut success = 0;

        let vertex_shader_cstr = CString::new(vertex_shader).unwrap();
        let vertex_shader_array = [vertex_shader_cstr.as_ptr()];
        let vertex_shader_id = (gl.glCreateShader)(opengl::GL_VERTEX_SHADER);
        (gl.glShaderSource)(vertex_shader_id, 1, vertex_shader_array.as_ptr() as *const *const i8, ptr::null());
        (gl.glCompileShader)(vertex_shader_id);

        (gl.glGetShaderiv)(vertex_shader_id, opengl::GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log = vec![0; ERROR_LENGTH];
            (gl.glGetShaderInfoLog)(vertex_shader_id, ERROR_LENGTH as i32, ptr::null_mut(), log.as_mut_ptr() as *mut i8);

            return Err(String::from_utf8(log).unwrap());
        }

        let fragment_shader_cstr = CString::new(fragment_shader).unwrap();
        let fragment_shader_array = [fragment_shader_cstr.as_ptr()];
        let fragment_shader_id = (gl.glCreateShader)(opengl::GL_FRAGMENT_SHADER);
        (gl.glShaderSource)(fragment_shader_id, 1, fragment_shader_array.as_ptr() as *const *const i8, ptr::null());
        (gl.glCompileShader)(fragment_shader_id);

        (gl.glGetShaderiv)(fragment_shader_id, opengl::GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            let mut log = vec![0; ERROR_LENGTH];
            (gl.glGetShaderInfoLog)(fragment_shader_id, ERROR_LENGTH as i32, ptr::null_mut(), log.as_mut_ptr() as *mut i8);

            return Err(String::from_utf8(log).unwrap());
        }

        let shader_program = (gl.glCreateProgram)();
        (gl.glAttachShader)(shader_program, vertex_shader_id);
        (gl.glAttachShader)(shader_program, fragment_shader_id);
        (gl.glLinkProgram)(shader_program);

        (gl.glGetProgramiv)(shader_program, opengl::GL_LINK_STATUS, &mut success);
        if success == 0 {
            let mut log = vec![0; ERROR_LENGTH];
            (gl.glGetProgramInfoLog)(shader_program, 1024, ptr::null_mut(), log.as_mut_ptr() as *mut i8);

            return Err(String::from_utf8(log).unwrap());
        }

        (gl.glDeleteShader)(vertex_shader_id);
        (gl.glDeleteShader)(fragment_shader_id);

        Ok(shader_program)
    }
}

pub fn load_default(gl: &OpenGLContext) -> Result<u32, String> {
    load(gl, DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER)
}
