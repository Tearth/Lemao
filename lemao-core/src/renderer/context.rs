use super::batcher::BatchRenderer;
use super::cameras::Camera;
use super::drawable::circle::Circle;
use super::drawable::disc::Disc;
use super::drawable::frame::Frame;
use super::drawable::line::Line;
use super::drawable::rectangle::Rectangle;
use super::drawable::text::Text;
use super::drawable::tilemap::Tilemap;
use super::drawable::Color;
use super::drawable::Drawable;
use super::fonts::Font;
use super::shaders::Shader;
use super::shaders::DEFAULT_VERTEX_SHADER;
use super::shaders::GRADIENT_FRAGMENT_SHADER;
use super::shaders::SOLID_FRAGMENT_SHADER;
use super::shapes::Shape;
use super::textures::RawTexture;
use super::textures::Texture;
use crate::utils::storage::Storage;
use lemao_common_platform::renderer::RendererPlatformSpecific;
use lemao_math::color::SolidColor;
use lemao_math::vec2::Vec2;
use lemao_math::vec3::Vec3;
use lemao_opengl::bindings::opengl;
use lemao_opengl::pointers::OpenGLPointers;
use std::ffi::c_void;
use std::rc::Rc;

pub struct RendererContext {
    pub(crate) gl: Rc<OpenGLPointers>,

    pub viewport_size: Vec2,
    pub default_camera_id: usize,
    pub active_camera_id: usize,
    pub default_solid_shader_id: usize,
    pub default_gradient_shader_id: usize,
    pub active_shader_id: usize,
    pub default_line_shape_id: usize,
    pub default_rectangle_shape_id: usize,
    pub default_sprite_shape_id: usize,
    pub default_texture_id: usize,
    pub swap_interval: u32,

    renderer_platform_specific: Box<dyn RendererPlatformSpecific>,

    pub textures: Storage<Texture>,
    pub fonts: Storage<Font>,
    pub cameras: Storage<Camera>,
    pub shaders: Storage<Shader>,
    pub shapes: Storage<Shape>,

    pub batch_renderer: Option<BatchRenderer>,
}

impl RendererContext {
    pub fn new(renderer_platform_specific: Box<dyn RendererPlatformSpecific>, viewport_size: Vec2) -> Result<Self, String> {
        Ok(RendererContext {
            gl: Default::default(),

            viewport_size,
            default_camera_id: 0,
            active_camera_id: 0,
            default_solid_shader_id: 0,
            default_gradient_shader_id: 0,
            active_shader_id: 0,
            default_line_shape_id: 0,
            default_rectangle_shape_id: 0,
            default_sprite_shape_id: 0,
            default_texture_id: 0,
            swap_interval: 0,

            renderer_platform_specific,

            textures: Default::default(),
            fonts: Default::default(),
            shaders: Default::default(),
            cameras: Default::default(),
            shapes: Default::default(),

            batch_renderer: None,
        })
    }

    pub fn init(&mut self) -> Result<(), String> {
        #[cfg(debug_assertions)]
        unsafe {
            (self.gl.glEnable)(opengl::GL_DEBUG_OUTPUT);
            (self.gl.glDebugMessageCallback)(gl_error, std::ptr::null_mut());

            // Test error handler
            // (self.gl.glEnable)(99999);
        }

        self.init_default_camera()?;
        self.set_viewport_size(self.viewport_size)?;
        self.init_default_shaders()?;
        self.init_default_shapes()?;
        self.init_default_texture()?;
        self.init_batch_renderer();

        Ok(())
    }

    pub fn init_default_camera(&mut self) -> Result<(), String> {
        let camera = Camera::new(Default::default(), Default::default());
        self.default_camera_id = self.cameras.store(camera);
        self.set_camera_as_active(self.default_camera_id)?;

        Ok(())
    }

    pub fn init_default_shaders(&mut self) -> Result<(), String> {
        let solid_shader = Shader::new(self, DEFAULT_VERTEX_SHADER, SOLID_FRAGMENT_SHADER)?;
        self.default_solid_shader_id = self.shaders.store(solid_shader);

        let gradient_shader = Shader::new(self, DEFAULT_VERTEX_SHADER, GRADIENT_FRAGMENT_SHADER)?;
        self.default_gradient_shader_id = self.shaders.store(gradient_shader);

        Ok(())
    }

    pub fn init_default_shapes(&mut self) -> Result<(), String> {
        let sprite_shape = Shape::new(
            self,
            vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
            vec![0, 1, 2, 0, 2, 3],
            vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0)],
            vec![
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ],
        );
        self.default_sprite_shape_id = self.shapes.store(sprite_shape);

        let line_shape = Shape::new(
            self,
            vec![Vec3::new(-0.5, 0.0, 0.0), Vec3::new(0.5, 0.0, 0.0), Vec3::new(0.5, 1.0, 0.0), Vec3::new(-0.5, 1.0, 0.0)],
            vec![0, 1, 2, 0, 2, 3],
            vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0)],
            vec![
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ],
        );
        self.default_line_shape_id = self.shapes.store(line_shape);

        let rectangle_shape = Shape::new(
            self,
            vec![Vec3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0)],
            vec![0, 1, 2, 0, 2, 3],
            vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0)],
            vec![
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
                SolidColor::new(1.0, 1.0, 1.0, 1.0),
            ],
        );
        self.default_rectangle_shape_id = self.shapes.store(rectangle_shape);

        Ok(())
    }

    pub fn init_default_texture(&mut self) -> Result<(), String> {
        let texture = Texture::new(self, &RawTexture::new(Vec2::new(1.0, 1.0), vec![255, 255, 255, 255]))?;
        self.default_texture_id = self.textures.store(texture);

        Ok(())
    }

    pub fn init_batch_renderer(&mut self) {
        self.batch_renderer = Some(BatchRenderer::new(self, 1024 * 1024, 1024 * 1024));
    }

    pub fn set_viewport_size(&mut self, size: Vec2) -> Result<(), String> {
        unsafe {
            (self.gl.glViewport)(0, 0, size.x as i32, size.y as i32);
            self.cameras.get_mut(self.active_camera_id)?.size = size;
            self.cameras.get_mut(self.active_camera_id)?.dirty = true;

            Ok(())
        }
    }

    pub fn set_shader_as_active(&mut self, shader_id: usize) -> Result<(), String> {
        let shader = self.shaders.get_mut(shader_id)?;

        self.active_shader_id = shader_id;
        shader.set_as_active();
        Ok(())
    }

    pub fn set_camera_as_active(&mut self, camera_id: usize) -> Result<(), String> {
        let camera = self.cameras.get_mut(camera_id)?;

        self.active_camera_id = camera_id;
        camera.dirty = true;

        Ok(())
    }

    pub fn create_circle(&mut self) -> Result<Circle, String> {
        let texture = self.textures.get(self.default_texture_id)?;
        let circle = Circle::new(self, texture);

        Ok(circle)
    }

    pub fn create_disc(&mut self) -> Result<Disc, String> {
        let texture = self.textures.get(self.default_texture_id)?;
        let disc = Disc::new(self, texture);

        Ok(disc)
    }

    pub fn create_frame(&mut self) -> Result<Frame, String> {
        let texture = self.textures.get(self.default_texture_id)?;
        let frame = Frame::new(self, texture);

        Ok(frame)
    }

    pub fn create_line(&mut self) -> Result<Line, String> {
        let shape = self.shapes.get(self.default_line_shape_id)?;
        let texture = self.textures.get(self.default_texture_id)?;
        let line = Line::new(self, shape, texture);

        Ok(line)
    }

    pub fn create_rectangle(&mut self) -> Result<Rectangle, String> {
        let shape = self.shapes.get(self.default_rectangle_shape_id)?;
        let texture = self.textures.get(self.default_texture_id)?;
        let rectangle = Rectangle::new(self, shape, texture);

        Ok(rectangle)
    }

    pub fn create_text(&mut self, font_id: usize) -> Result<Text, String> {
        let font = self.fonts.get(font_id)?;
        let text = Text::new(self, font);

        Ok(text)
    }

    pub fn create_tilemap(&mut self, texture_id: usize) -> Result<Tilemap, String> {
        let texture = self.textures.get(texture_id)?;
        let tilemap = Tilemap::new(self, texture);

        Ok(tilemap)
    }

    pub fn enable_scissor(&self, position: Vec2, size: Vec2) {
        unsafe {
            (self.gl.glEnable)(opengl::GL_SCISSOR_TEST);
            (self.gl.glScissor)(position.x as i32, position.y as i32, size.x as i32, size.y as i32);
        }
    }

    pub fn disable_scissor(&self) {
        unsafe {
            (self.gl.glDisable)(opengl::GL_SCISSOR_TEST);
        }
    }

    pub fn batcher_add_drawable<T: Drawable>(&mut self, drawable: &T) -> Result<(), String> {
        let transformation_matrix = drawable.get_transformation_matrix();
        let mut batch = drawable.get_batch();

        if let Some(shape_id) = batch.shape_id {
            let shape = self.shapes.get(shape_id)?;
            batch.vertices = Some(&shape.vertices);
            batch.indices = Some(&shape.indices);
        }

        self.batch_renderer.as_mut().unwrap().add(transformation_matrix, &batch)?;
        Ok(())
    }

    pub fn batcher_draw(&mut self) -> Result<(), String> {
        let shader_id = match self.batch_renderer.as_ref().unwrap().get_color() {
            Color::SolidColor(_) => self.default_solid_shader_id,
            Color::Gradient(_) => self.default_gradient_shader_id,
        };

        if shader_id != self.active_shader_id || self.cameras.get(self.active_camera_id)?.dirty {
            self.set_shader_as_active(shader_id)?;

            let camera = self.cameras.get_mut(self.active_camera_id)?;
            let shader = self.shaders.get(shader_id)?;
            shader.set_parameter("proj", camera.get_projection_matrix().as_ptr())?;
            shader.set_parameter("view", camera.get_view_matrix().as_ptr())?;
            camera.dirty = false;
        }

        self.batch_renderer.as_mut().unwrap().draw(self.shaders.get(shader_id)?)
    }

    pub fn draw<T: Drawable>(&mut self, drawable: &mut T) -> Result<(), String> {
        let color = drawable.get_color();

        let shader_id = match color {
            Color::SolidColor(_) => self.default_solid_shader_id,
            Color::Gradient(_) => self.default_gradient_shader_id,
        };

        if shader_id != self.active_shader_id || self.cameras.get(self.active_camera_id)?.dirty {
            self.set_shader_as_active(shader_id)?;

            let camera = self.cameras.get_mut(self.active_camera_id)?;
            let shader = self.shaders.get(shader_id)?;
            shader.set_parameter("proj", camera.get_projection_matrix().as_ptr())?;
            shader.set_parameter("view", camera.get_view_matrix().as_ptr())?;
            camera.dirty = false;
        }

        let shader = self.shaders.get(shader_id)?;
        drawable.draw(shader)?;

        Ok(())
    }

    pub fn clear(&self, color: SolidColor) {
        unsafe {
            (self.gl.glClearColor)(color.r, color.g, color.b, color.a);
            (self.gl.glClear)(opengl::GL_COLOR_BUFFER_BIT);
        }
    }

    pub fn set_swap_interval(&mut self, interval: u32) {
        self.renderer_platform_specific.set_swap_interval(interval);
        self.swap_interval = interval;
    }

    pub fn close(&self) {
        self.renderer_platform_specific.close();
    }
}

impl Drop for RendererContext {
    fn drop(&mut self) {
        self.close();
    }
}

#[allow(dead_code)]
unsafe extern "C" fn gl_error(
    source: opengl::GLenum,
    r#type: opengl::GLenum,
    id: opengl::GLuint,
    severity: opengl::GLenum,
    length: opengl::GLsizei,
    message: *const i8,
    user_param: *const c_void,
) {
    println!(
        "OpenGL error: source={}, type={}, id={}, severity={}, user_param={:?}, message={}",
        source,
        r#type,
        id,
        severity,
        user_param,
        &String::from_raw_parts(message as *mut u8, length as usize, length as usize)
    );
}
