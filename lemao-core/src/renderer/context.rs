use super::cameras::storage::CameraStorage;
use super::cameras::Camera;
use super::drawable::sprite::Sprite;
use super::drawable::storage::DrawableStorage;
use super::drawable::text::Text;
use super::drawable::Drawable;
use super::fonts::storage::FontStorage;
use super::shaders::storage::ShaderStorage;
use super::shaders::Shader;
use super::shapes::storage::ShapeStorage;
use super::shapes::Shape;
use super::textures::storage::TextureStorage;
use crate::window::context::WindowContext;
use lemao_math::color::Color;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::opengl;
use lemao_opengl::bindings::wgl;
use lemao_opengl::pointers::OpenGLPointers;
use lemao_winapi::bindings::winapi;
use std::ffi::c_void;
use std::fs;
use std::mem;
use std::ptr;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::Mutex;

pub struct RendererContext {
    pub(crate) gl: Rc<OpenGLPointers>,
    gl_context: winapi::HGLRC,
    active_camera_id: usize,
    default_shader_id: usize,
    active_shader_id: usize,
    default_shape_id: usize,
    textures: Arc<Mutex<TextureStorage>>,
    fonts: Arc<Mutex<FontStorage>>,
    cameras: Option<CameraStorage>,
    shaders: Option<ShaderStorage>,
    drawables: Option<DrawableStorage>,
    shapes: Option<ShapeStorage>,
}

impl RendererContext {
    pub fn new(hdc: winapi::HDC, textures: Arc<Mutex<TextureStorage>>, fonts: Arc<Mutex<FontStorage>>) -> Result<Self, String> {
        unsafe {
            let fake_window = WindowContext::new_fake()?;
            let fake_window_hdc = fake_window.hdc;

            let pixel_format_descriptor = winapi::PIXELFORMATDESCRIPTOR {
                nSize: mem::size_of::<winapi::PIXELFORMATDESCRIPTOR>() as u16,
                nVersion: 1,
                dwFlags: winapi::PFD_DRAW_TO_WINDOW | winapi::PFD_SUPPORT_OPENGL | winapi::PFD_DOUBLEBUFFER,
                iPixelType: winapi::PFD_TYPE_RGBA as u8,
                cColorBits: 32,
                cRedBits: 0,
                cRedShift: 0,
                cGreenBits: 0,
                cGreenShift: 0,
                cBlueBits: 0,
                cBlueShift: 0,
                cAlphaBits: 0,
                cAlphaShift: 0,
                cAccumBits: 0,
                cAccumRedBits: 0,
                cAccumGreenBits: 0,
                cAccumBlueBits: 0,
                cAccumAlphaBits: 0,
                cDepthBits: 24,
                cStencilBits: 8,
                cAuxBuffers: 0,
                iLayerType: winapi::PFD_MAIN_PLANE as u8,
                bReserved: 0,
                dwLayerMask: 0,
                dwVisibleMask: 0,
                dwDamageMask: 0,
            };

            let pixel_format = winapi::ChoosePixelFormat(fake_window_hdc, &pixel_format_descriptor);
            if winapi::SetPixelFormat(fake_window_hdc, pixel_format, &pixel_format_descriptor) == 0 {
                return Err(format!("Error while setting pixel format for fake window, GetLastError()={}", winapi::GetLastError()));
            }

            let fake_gl_context: winapi::HGLRC = winapi::wglCreateContext(fake_window_hdc);
            if winapi::wglMakeCurrent(fake_window_hdc, fake_gl_context) == 0 {
                return Err(format!("Error while creating fake OpenGL context, GetLastError()={}", winapi::GetLastError()));
            }

            let gl: Rc<OpenGLPointers> = Default::default();

            winapi::wglDeleteContext(fake_gl_context);
            fake_window.close();

            let mut attributes = [
                wgl::WGL_DRAW_TO_WINDOW_ARB,
                opengl::GL_TRUE,
                wgl::WGL_SUPPORT_OPENGL_ARB,
                opengl::GL_TRUE,
                wgl::WGL_DOUBLE_BUFFER_ARB,
                opengl::GL_TRUE,
                wgl::WGL_PIXEL_TYPE_ARB,
                wgl::WGL_TYPE_RGBA_ARB,
                wgl::WGL_COLOR_BITS_ARB,
                32,
                wgl::WGL_DEPTH_BITS_ARB,
                24,
                wgl::WGL_STENCIL_BITS_ARB,
                8,
                0,
            ];

            let mut pixel_format = 0;
            let mut formats_count = 0;
            let attributes_ptr = attributes.as_mut_ptr() as *const i32;

            if (gl.wglChoosePixelFormatARB)(hdc as wgl::HDC, attributes_ptr, ptr::null_mut(), 1, &mut pixel_format, &mut formats_count) == 0 {
                return Err(format!("Error while loading available pixel formats for desired window, GetLastError()={}", winapi::GetLastError()));
            }

            if winapi::SetPixelFormat(hdc, pixel_format, &pixel_format_descriptor) == 0 {
                return Err(format!("Error while setting pixel format for desired window, GetLastError()={}", winapi::GetLastError()));
            }

            let mut attributes = [wgl::WGL_CONTEXT_MAJOR_VERSION_ARB, 3, wgl::WGL_CONTEXT_MINOR_VERSION_ARB, 2, 0];
            let attributes_ptr = attributes.as_mut_ptr() as *const i32;

            let gl_context = (gl.wglCreateContextAttribsARB)(hdc as wgl::HDC, ptr::null_mut(), attributes_ptr);
            if winapi::wglMakeCurrent(hdc, gl_context as winapi::HGLRC) == 0 {
                return Err(format!("Error while creating OpenGL context for desired window, GetLastError()={}", winapi::GetLastError()));
            }

            Ok(RendererContext {
                gl: Default::default(),
                gl_context: gl_context as winapi::HGLRC,
                active_camera_id: 0,
                default_shader_id: 0,
                active_shader_id: 0,
                default_shape_id: 0,

                textures,
                fonts,
                shaders: None,
                drawables: None,
                cameras: None,
                shapes: None,
            })
        }
    }

    pub fn init(&self) {
        unsafe {
            (self.gl.glEnable)(opengl::GL_DEBUG_OUTPUT);
            (self.gl.glDebugMessageCallback)(gl_error, ptr::null_mut());
        }
    }

    pub fn init_storages(&mut self) {
        self.cameras = Some(Default::default());
        self.shaders = Some(Default::default());
        self.drawables = Some(Default::default());
        self.shapes = Some(Default::default());
    }

    pub fn init_default_camera(&mut self) {
        let camera = Camera::new(Default::default(), Default::default());
        self.active_camera_id = self.cameras.as_mut().unwrap().store(camera);
    }

    pub fn init_default_shader(&mut self) -> Result<(), String> {
        let shader = Shader::new_default(self)?;
        self.default_shader_id = self.shaders.as_mut().unwrap().store(shader);

        Ok(())
    }

    pub fn init_default_shape(&mut self) {
        let shape = Shape::new(
            self,
            vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
            vec![0, 1, 2, 0, 2, 3],
            vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0)],
            vec![Color::new(1.0, 1.0, 1.0, 1.0), Color::new(1.0, 1.0, 1.0, 1.0), Color::new(1.0, 1.0, 1.0, 1.0), Color::new(1.0, 1.0, 1.0, 1.0)],
        );
        self.default_shape_id = self.shapes.as_mut().unwrap().store(shape);
    }

    pub fn set_viewport(&mut self, width: u32, height: u32) {
        unsafe {
            (self.gl.glViewport)(0, 0, width as i32, height as i32);
        }
    }

    pub fn create_shader(&mut self, vertex_shader_path: &str, fragment_shader_path: &str) -> Result<usize, String> {
        let vertex_shader = match fs::read_to_string(vertex_shader_path) {
            Ok(content) => content,
            Err(message) => return Err(format!("Error while loading vertex shader: {}", message)),
        };

        let fragment_shader = match fs::read_to_string(fragment_shader_path) {
            Ok(content) => content,
            Err(message) => return Err(format!("Error while loading fragment shader: {}", message)),
        };

        let shader = Shader::new_from_string(self, &vertex_shader, &fragment_shader)?;
        Ok(self.shaders.as_mut().unwrap().store(shader))
    }

    pub fn get_shader(&self, shader_id: usize) -> Result<&Shader, String> {
        self.shaders.as_ref().unwrap().get(shader_id)
    }

    pub fn get_shader_mut(&mut self, shader_id: usize) -> Result<&mut Shader, String> {
        self.shaders.as_mut().unwrap().get_mut(shader_id)
    }

    pub fn set_shader_as_active(&mut self, shader_id: usize) -> Result<(), String> {
        let shader = self.shaders.as_mut().unwrap().get_mut(shader_id)?;

        self.active_shader_id = shader_id;
        shader.set_as_active();
        Ok(())
    }

    pub fn set_default_shader(&mut self) -> Result<(), String> {
        self.set_shader_as_active(self.default_shader_id)
    }

    pub fn create_camera(&mut self, position: Vec2, size: Vec2) -> Result<usize, String> {
        let camera = Camera::new(position, size);
        Ok(self.cameras.as_mut().unwrap().store(camera))
    }

    pub fn get_camera(&self, camera_id: usize) -> Result<&Camera, String> {
        self.cameras.as_ref().unwrap().get(camera_id)
    }

    pub fn get_camera_mut(&mut self, camera_id: usize) -> Result<&mut Camera, String> {
        self.cameras.as_mut().unwrap().get_mut(camera_id)
    }

    pub fn get_active_camera(&self) -> Result<&Camera, String> {
        self.cameras.as_ref().unwrap().get(self.active_camera_id)
    }

    pub fn get_active_camera_mut(&mut self) -> Result<&mut Camera, String> {
        self.cameras.as_mut().unwrap().get_mut(self.active_camera_id)
    }

    pub fn set_camera_as_active(&mut self, camera_id: usize) -> Result<(), String> {
        let camera = self.cameras.as_mut().unwrap().get_mut(camera_id)?;

        self.active_camera_id = camera_id;
        camera.set_dirty_flag(true);

        Ok(())
    }

    pub fn create_sprite(&mut self, texture_id: usize) -> Result<usize, String> {
        let shape = self.shapes.as_ref().unwrap().get(self.default_shape_id)?;
        let texture_storage = self.textures.lock().unwrap();
        let texture = texture_storage.get(texture_id)?;
        let sprite = Box::new(Sprite::new(self, shape, texture));

        Ok(self.drawables.as_mut().unwrap().store(sprite))
    }

    pub fn create_text(&mut self, font_id: usize) -> Result<usize, String> {
        let font_storage = self.fonts.lock().unwrap();
        let font = font_storage.get(font_id)?;
        let text = Box::new(Text::new(self, font));

        Ok(self.drawables.as_mut().unwrap().store(text))
    }

    pub fn get_drawable(&self, drawable_id: usize) -> Result<&dyn Drawable, String> {
        self.drawables.as_ref().unwrap().get(drawable_id)
    }

    pub fn get_drawable_with_type<T: 'static>(&self, drawable_id: usize) -> Result<&T, String> {
        self.get_drawable(drawable_id)?.as_any().downcast_ref::<T>().ok_or(format!("Drawable object with id {} cannot be downcasted", drawable_id))
    }

    pub fn get_drawable_mut(&mut self, drawable_id: usize) -> Result<&mut dyn Drawable, String> {
        self.drawables.as_mut().unwrap().get_mut(drawable_id)
    }

    pub fn get_drawable_with_type_mut<T: 'static>(&mut self, drawable_id: usize) -> Result<&mut T, String> {
        self.get_drawable_mut(drawable_id)?.as_any_mut().downcast_mut::<T>().ok_or(format!("Drawable object with id {} cannot be downcasted", drawable_id))
    }

    pub fn clear(&self, color: Color) {
        unsafe {
            (self.gl.glClearColor)(color.r, color.g, color.b, color.a);
            (self.gl.glClear)(opengl::GL_COLOR_BUFFER_BIT);
        }
    }

    pub fn draw(&mut self, drawable_id: usize) -> Result<(), String> {
        let camera = self.cameras.as_mut().unwrap().get_mut(self.active_camera_id)?;
        let shader = self.shaders.as_ref().unwrap().get(self.active_shader_id)?;

        if camera.get_dirty_flag() {
            shader.set_parameter("proj", camera.get_projection_matrix().as_ptr())?;
            shader.set_parameter("view", camera.get_view_matrix().as_ptr())?;
            camera.set_dirty_flag(false);
        }

        self.get_drawable(drawable_id)?.draw(shader)?;
        Ok(())
    }

    pub fn release(&self) -> Result<(), String> {
        unsafe {
            if winapi::wglDeleteContext(self.gl_context) == 0 {
                return Err(format!("Error while releasing OpenGL context, GetLastError()={}", winapi::GetLastError()));
            }

            Ok(())
        }
    }
}

unsafe extern "C" fn gl_error(
    source: opengl::GLenum,
    r#type: opengl::GLenum,
    id: opengl::GLuint,
    severity: opengl::GLenum,
    length: opengl::GLsizei,
    message: *const i8,
    user_param: *const c_void,
) {
    panic!(
        "OpenGL error: source={}, type={}, id={}, severity={}, user_param={:?}, message={}",
        source,
        r#type,
        id,
        severity,
        user_param,
        &String::from_raw_parts(message as *mut u8, length as usize, length as usize)
    );
}
