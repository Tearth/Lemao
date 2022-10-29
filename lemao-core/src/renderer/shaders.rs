use lemao_opengl::bindings::opengl;
use lemao_opengl::context::OpenGLContext;
use std::ptr;

#[rustfmt::skip]
pub const DEFAULT_VERTEX_SHADER: &str = 
"#version 330 core

layout (location = 0) in vec3 aPos;

void main()
{
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}\0";

#[rustfmt::skip]
pub const DEFAULT_FRAGMENT_SHADER: &str = 
"#version 330 core

out vec4 FragColor;

void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
}\0";

pub fn load_and_compile(gl: &OpenGLContext, vertex_shader: &str, fragment_shader: &str) -> Result<u32, String> {
    unsafe {
        let source_array = [vertex_shader.as_ptr()];
        let vertex_shader = (gl.glCreateShader)(opengl::GL_VERTEX_SHADER);
        (gl.glShaderSource)(vertex_shader, 1, source_array.as_ptr() as *const *const i8, ptr::null());
        (gl.glCompileShader)(vertex_shader);

        let mut success = 0;
        (gl.glGetShaderiv)(vertex_shader, opengl::GL_COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut log: Vec<u8> = Vec::new();
            log.resize(1024, 0);

            (gl.glGetShaderInfoLog)(vertex_shader, 1024, ptr::null_mut(), log.as_mut_ptr() as *mut i8);
            let message = String::from_utf8(log).unwrap();

            return Err(message);
        }

        let source_array = [fragment_shader.as_ptr()];
        let fragment_shader = (gl.glCreateShader)(opengl::GL_FRAGMENT_SHADER);
        (gl.glShaderSource)(fragment_shader, 1, source_array.as_ptr() as *const *const i8, ptr::null());
        (gl.glCompileShader)(fragment_shader);

        let mut success = 0;
        (gl.glGetShaderiv)(fragment_shader, opengl::GL_COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut log: Vec<u8> = Vec::new();
            log.resize(1024, 0);

            (gl.glGetShaderInfoLog)(fragment_shader, 1024, ptr::null_mut(), log.as_mut_ptr() as *mut i8);
            let message = String::from_utf8(log).unwrap();

            return Err(message);
        }

        let shader_program = (gl.glCreateProgram)();
        (gl.glAttachShader)(shader_program, vertex_shader);
        (gl.glAttachShader)(shader_program, fragment_shader);
        (gl.glLinkProgram)(shader_program);

        let mut success = 0;
        (gl.glGetProgramiv)(shader_program, opengl::GL_LINK_STATUS, &mut success);

        if success == 0 {
            let mut log: Vec<u8> = Vec::new();
            log.resize(1024, 0);

            (gl.glGetProgramInfoLog)(shader_program, 1024, ptr::null_mut(), log.as_mut_ptr() as *mut i8);
            let message = String::from_utf8(log).unwrap();

            return Err(message);
        }

        (gl.glDeleteShader)(vertex_shader);
        (gl.glDeleteShader)(fragment_shader);

        Ok(shader_program)
    }
}
