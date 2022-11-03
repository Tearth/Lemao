use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::collections::HashMap;
use std::ffi::CString;
use std::ptr;
use std::rc::Rc;

pub mod storage;

pub const MAX_UNIFORM_NAME_LENGTH: usize = 32;
pub const ERROR_LENGTH: usize = 1024;
pub const DEFAULT_VERTEX_SHADER: &str = include_str!("./default.vert");
pub const DEFAULT_FRAGMENT_SHADER: &str = include_str!("./default.frag");

pub struct Shader {
    pub id: usize,
    program_id: u32,
    uniforms: HashMap<String, u32>,

    gl: Rc<OpenGLPointers>,
}

impl Shader {
    pub fn new_default(gl: Rc<OpenGLPointers>) -> Result<Self, String> {
        Shader::new_from_string(gl, DEFAULT_VERTEX_SHADER, DEFAULT_FRAGMENT_SHADER)
    }

    pub fn new_from_string(gl: Rc<OpenGLPointers>, vertex_shader: &str, fragment_shader: &str) -> Result<Self, String> {
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
                let log_ptr = log.as_mut_ptr() as *mut i8;
                (gl.glGetShaderInfoLog)(vertex_shader_id, ERROR_LENGTH as i32, ptr::null_mut(), log_ptr);

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
                let log_ptr = log.as_mut_ptr() as *mut i8;
                (gl.glGetShaderInfoLog)(fragment_shader_id, ERROR_LENGTH as i32, ptr::null_mut(), log_ptr);

                return Err(String::from_utf8(log).unwrap());
            }

            let program_id = (gl.glCreateProgram)();
            (gl.glAttachShader)(program_id, vertex_shader_id);
            (gl.glAttachShader)(program_id, fragment_shader_id);
            (gl.glLinkProgram)(program_id);
            (gl.glGetProgramiv)(program_id, opengl::GL_LINK_STATUS, &mut success);

            if success == 0 {
                let mut log = vec![0; ERROR_LENGTH];
                let log_ptr = log.as_mut_ptr() as *mut i8;
                (gl.glGetProgramInfoLog)(program_id, 1024, ptr::null_mut(), log_ptr);

                return Err(String::from_utf8(log).unwrap());
            }

            (gl.glDeleteShader)(vertex_shader_id);
            (gl.glDeleteShader)(fragment_shader_id);

            let mut active_uniforms = 0;
            let mut uniforms: HashMap<String, u32> = Default::default();

            (gl.glGetProgramiv)(program_id, opengl::GL_ACTIVE_UNIFORMS, &mut active_uniforms);

            for index in 0..active_uniforms {
                let mut r#type = 0;
                let mut length = 0;
                let mut size = 0;
                let mut name = vec![0; MAX_UNIFORM_NAME_LENGTH];
                let name_ptr = name.as_mut_ptr() as *mut i8;

                (gl.glGetActiveUniform)(program_id, index as u32, MAX_UNIFORM_NAME_LENGTH as i32, &mut length, &mut size, &mut r#type, name_ptr);

                let name = String::from_utf8(name).unwrap();
                let name = name.trim_end_matches(char::from_u32(0).unwrap()).to_string();
                let name_cstr = CString::new(name.clone()).unwrap();
                let location = (gl.glGetUniformLocation)(3, name_cstr.as_ptr());

                uniforms.insert(name, location as u32);
            }

            Ok(Shader { id: 0, program_id, uniforms, gl })
        }
    }

    pub fn set_parameter(&self, name: &str, data: *const f32) {
        unsafe {
            let id = self.uniforms.get(name).unwrap();
            (self.gl.glUniformMatrix4fv)(*id as i32, 1, opengl::GL_TRUE as u8, data);
        }
    }

    pub fn set_as_active(&self) {
        unsafe {
            (self.gl.glUseProgram)(self.program_id);
            (self.gl.glEnable)(opengl::GL_BLEND);
            (self.gl.glBlendFunc)(opengl::GL_SRC_ALPHA, opengl::GL_ONE_MINUS_SRC_ALPHA);
        }
    }
}
