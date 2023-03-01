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
use super::drawable::DrawableEnum;
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
use std::fs;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;

pub struct RendererContext {
    pub(crate) gl: Rc<OpenGLPointers>,

    viewport_size: Vec2,
    default_camera_id: usize,
    active_camera_id: usize,
    default_solid_shader_id: usize,
    default_gradient_shader_id: usize,
    active_shader_id: usize,
    default_line_shape_id: usize,
    default_rectangle_shape_id: usize,
    default_sprite_shape_id: usize,
    default_texture_id: usize,
    swap_interval: u32,

    renderer_platform_specific: Box<dyn RendererPlatformSpecific>,
    pub textures: Arc<RwLock<Storage<Texture>>>,
    pub fonts: Arc<RwLock<Storage<Font>>>,
    pub cameras: Storage<Camera>,
    pub shaders: Storage<Shader>,

    pub circles: Storage<Circle>,
    pub discs: Storage<Disc>,
    pub frames: Storage<Frame>,
    pub lines: Storage<Line>,
    pub rectangles: Storage<Rectangle>,
    pub texts: Storage<Text>,
    pub tilemaps: Storage<Tilemap>,

    shapes: Storage<Shape>,
    batch_renderer: Option<BatchRenderer>,
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

            circles: Default::default(),
            discs: Default::default(),
            frames: Default::default(),
            lines: Default::default(),
            rectangles: Default::default(),
            texts: Default::default(),
            tilemaps: Default::default(),

            shapes: Default::default(),
            batch_renderer: None,
        })
    }

    pub fn init(&mut self) -> Result<(), String> {
        // #[cfg(debug_assertions)]
        // unsafe {
        //     (self.gl.glEnable)(opengl::GL_DEBUG_OUTPUT);
        //     (self.gl.glDebugMessageCallback)(gl_error, std::ptr::null_mut());
        // }

        self.set_viewport_size(self.viewport_size);
        self.init_default_camera()?;
        self.init_default_shaders()?;
        self.init_default_shapes()?;
        self.init_default_texture()?;
        self.init_batch_renderer();

        Ok(())
    }

    pub fn init_default_camera(&mut self) -> Result<(), String> {
        let camera = Camera::new(Default::default(), Default::default());
        self.default_camera_id = self.cameras.store(camera);
        self.cameras.get_mut(self.default_camera_id)?.id = self.default_camera_id;
        self.set_default_camera()?;

        Ok(())
    }

    pub fn init_default_shaders(&mut self) -> Result<(), String> {
        let solid_shader = Shader::new(self, DEFAULT_VERTEX_SHADER, SOLID_FRAGMENT_SHADER)?;
        self.default_solid_shader_id = self.shaders.store(solid_shader);
        self.shaders.get_mut(self.default_solid_shader_id)?.id = self.default_solid_shader_id;

        let gradient_shader = Shader::new(self, DEFAULT_VERTEX_SHADER, GRADIENT_FRAGMENT_SHADER)?;
        self.default_gradient_shader_id = self.shaders.store(gradient_shader);
        self.shaders.get_mut(self.default_gradient_shader_id)?.id = self.default_gradient_shader_id;

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
        self.shapes.get_mut(self.default_sprite_shape_id)?.id = self.default_sprite_shape_id;

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
        self.shapes.get_mut(self.default_line_shape_id)?.id = self.default_line_shape_id;

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
        self.shapes.get_mut(self.default_rectangle_shape_id)?.id = self.default_rectangle_shape_id;

        Ok(())
    }

    pub fn init_default_texture(&mut self) -> Result<(), String> {
        let texture = Texture::new(self, &RawTexture::new(Vec2::new(1.0, 1.0), vec![255, 255, 255, 255]))?;
        self.default_texture_id = self.textures.write().unwrap().store(texture);
        self.textures.write().unwrap().get_mut(self.default_texture_id)?.id = self.default_texture_id;

        Ok(())
    }

    pub fn init_batch_renderer(&mut self) {
        self.batch_renderer = Some(BatchRenderer::new(self, 1024 * 1024, 1024 * 1024));
    }

    pub fn get_viewport_size(&self) -> Vec2 {
        self.viewport_size
    }

    pub fn set_viewport_size(&mut self, size: Vec2) {
        unsafe {
            (self.gl.glViewport)(0, 0, size.x as i32, size.y as i32);
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

        let shader = Shader::new(self, &vertex_shader, &fragment_shader)?;
        let id = self.shaders.store(shader);
        self.shaders.get_mut(id)?.id = id;

        Ok(id)
    }

    pub fn set_shader_as_active(&mut self, shader_id: usize) -> Result<(), String> {
        let shader = self.shaders.get_mut(shader_id)?;

        self.active_shader_id = shader_id;
        shader.set_as_active();
        Ok(())
    }

    pub fn create_camera(&mut self, position: Vec2, size: Vec2) -> Result<usize, String> {
        let camera = Camera::new(position, size);
        let id = self.cameras.store(camera);
        self.cameras.get_mut(id)?.id = id;

        Ok(id)
    }

    pub fn get_active_camera(&self) -> Result<&Camera, String> {
        self.cameras.get(self.active_camera_id)
    }

    pub fn get_active_camera_mut(&mut self) -> Result<&mut Camera, String> {
        self.cameras.get_mut(self.active_camera_id)
    }

    pub fn set_camera_as_active(&mut self, camera_id: usize) -> Result<(), String> {
        let camera = self.cameras.get_mut(camera_id)?;

        self.active_camera_id = camera_id;
        camera.set_dirty_flag(true);

        Ok(())
    }

    pub fn set_default_camera(&mut self) -> Result<(), String> {
        self.set_camera_as_active(self.default_camera_id)
    }

    pub fn create_circle(&mut self) -> Result<&mut Circle, String> {
        let texture_storage = self.textures.read().unwrap();
        let texture = texture_storage.get(self.default_texture_id)?;
        let circle = Circle::new(self, texture);

        let id = self.circles.store(circle);
        let circle = self.circles.get_mut(id)?;
        circle.id = id;

        Ok(circle)
    }

    pub fn create_disc(&mut self) -> Result<&mut Disc, String> {
        let texture_storage = self.textures.read().unwrap();
        let texture = texture_storage.get(self.default_texture_id)?;
        let disc = Disc::new(self, texture);

        let id = self.discs.store(disc);
        let disc = self.discs.get_mut(id)?;
        disc.id = id;

        Ok(disc)
    }

    pub fn create_frame(&mut self) -> Result<&mut Frame, String> {
        let texture_storage = self.textures.read().unwrap();
        let texture = texture_storage.get(self.default_texture_id)?;
        let frame = Frame::new(self, texture);

        let id = self.frames.store(frame);
        let frame = self.frames.get_mut(id)?;
        frame.id = id;

        Ok(frame)
    }

    pub fn create_line(&mut self) -> Result<&mut Line, String> {
        let shape = self.shapes.get(self.default_line_shape_id)?;
        let texture_storage = self.textures.read().unwrap();
        let texture = texture_storage.get(self.default_texture_id)?;
        let line = Line::new(self, shape, texture);

        let id = self.lines.store(line);
        let line = self.lines.get_mut(id)?;
        line.id = id;

        Ok(line)
    }

    pub fn create_rectangle(&mut self) -> Result<&mut Rectangle, String> {
        let shape = self.shapes.get(self.default_rectangle_shape_id)?;
        let texture_storage = self.textures.read().unwrap();
        let texture = texture_storage.get(self.default_texture_id)?;
        let rectangle = Rectangle::new(self, shape, texture);

        let id = self.rectangles.store(rectangle);
        let rectangle = self.rectangles.get_mut(id)?;
        rectangle.id = id;

        Ok(rectangle)
    }

    pub fn create_text(&mut self, font_id: usize) -> Result<&mut Text, String> {
        let font_storage = self.fonts.read().unwrap();
        let font = font_storage.get(font_id)?;
        let text = Text::new(self, font);

        let id = self.texts.store(text);
        let text = self.texts.get_mut(id)?;
        text.id = id;

        Ok(text)
    }

    pub fn create_tilemap(&mut self, texture_id: usize) -> Result<&mut Tilemap, String> {
        let texture_storage = self.textures.read().unwrap();
        let texture = texture_storage.get(texture_id)?;
        let tilemap = Tilemap::new(self, texture);

        let id = self.tilemaps.store(tilemap);
        let tilemap = self.tilemaps.get_mut(id)?;
        tilemap.id = id;

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

    pub fn batcher_add_drawable(&mut self, r#type: DrawableEnum, drawable_id: usize) -> Result<(), String> {
        let (transformation_matrix, mut batch) = match r#type {
            DrawableEnum::Circle => (self.circles.get(drawable_id)?.get_transformation_matrix(), self.circles.get(drawable_id)?.get_batch()),
            DrawableEnum::Disc => (self.discs.get(drawable_id)?.get_transformation_matrix(), self.discs.get(drawable_id)?.get_batch()),
            DrawableEnum::Frame => (self.frames.get(drawable_id)?.get_transformation_matrix(), self.frames.get(drawable_id)?.get_batch()),
            DrawableEnum::Line => (self.lines.get(drawable_id)?.get_transformation_matrix(), self.lines.get(drawable_id)?.get_batch()),
            DrawableEnum::Rectangle => (self.rectangles.get(drawable_id)?.get_transformation_matrix(), self.rectangles.get(drawable_id)?.get_batch()),
            DrawableEnum::Text => (self.texts.get(drawable_id)?.get_transformation_matrix(), self.texts.get(drawable_id)?.get_batch()),
            DrawableEnum::Tilemap => (self.tilemaps.get(drawable_id)?.get_transformation_matrix(), self.tilemaps.get(drawable_id)?.get_batch()),
        };

        if let Some(shape_id) = batch.shape_id {
            let shape = self.shapes.get(shape_id)?;
            batch.vertices = Some(shape.get_vertices());
            batch.indices = Some(shape.get_indices());
        }

        self.batch_renderer.as_mut().unwrap().add(transformation_matrix, &batch)?;
        Ok(())
    }

    pub fn batcher_draw(&mut self) -> Result<(), String> {
        let shader_id = match self.batch_renderer.as_ref().unwrap().get_color() {
            Color::SolidColor(_) => self.default_solid_shader_id,
            Color::Gradient(_) => self.default_gradient_shader_id,
        };

        if shader_id != self.active_shader_id || self.cameras.get(self.active_camera_id)?.is_dirty() {
            self.set_shader_as_active(shader_id)?;

            let camera = self.cameras.get_mut(self.active_camera_id)?;
            let shader = self.shaders.get(shader_id)?;
            shader.set_parameter("proj", camera.get_projection_matrix().as_ptr())?;
            shader.set_parameter("view", camera.get_view_matrix().as_ptr())?;
            camera.set_dirty_flag(false);
        }

        self.batch_renderer.as_mut().unwrap().draw(self.shaders.get(shader_id)?)
    }

    pub fn draw(&mut self, r#type: DrawableEnum, drawable_id: usize) -> Result<(), String> {
        let color = match r#type {
            DrawableEnum::Circle => self.circles.get(drawable_id)?.get_color(),
            DrawableEnum::Disc => self.discs.get(drawable_id)?.get_color(),
            DrawableEnum::Frame => self.frames.get(drawable_id)?.get_color(),
            DrawableEnum::Line => self.lines.get(drawable_id)?.get_color(),
            DrawableEnum::Rectangle => self.rectangles.get(drawable_id)?.get_color(),
            DrawableEnum::Text => self.texts.get(drawable_id)?.get_color(),
            DrawableEnum::Tilemap => self.tilemaps.get(drawable_id)?.get_color(),
        };

        let shader_id = match color {
            Color::SolidColor(_) => self.default_solid_shader_id,
            Color::Gradient(_) => self.default_gradient_shader_id,
        };

        if shader_id != self.active_shader_id || self.cameras.get(self.active_camera_id)?.is_dirty() {
            self.set_shader_as_active(shader_id)?;

            let camera = self.cameras.get_mut(self.active_camera_id)?;
            let shader = self.shaders.get(shader_id)?;
            shader.set_parameter("proj", camera.get_projection_matrix().as_ptr())?;
            shader.set_parameter("view", camera.get_view_matrix().as_ptr())?;
            camera.set_dirty_flag(false);
        }

        let shader = self.shaders.get(shader_id)?;
        match r#type {
            DrawableEnum::Circle => self.circles.get_mut(drawable_id)?.draw(shader)?,
            DrawableEnum::Disc => self.discs.get_mut(drawable_id)?.draw(shader)?,
            DrawableEnum::Frame => self.frames.get_mut(drawable_id)?.draw(shader)?,
            DrawableEnum::Line => self.lines.get_mut(drawable_id)?.draw(shader)?,
            DrawableEnum::Rectangle => self.rectangles.get_mut(drawable_id)?.draw(shader)?,
            DrawableEnum::Text => self.texts.get_mut(drawable_id)?.draw(shader)?,
            DrawableEnum::Tilemap => self.tilemaps.get_mut(drawable_id)?.draw(shader)?,
        };

        Ok(())
    }

    pub fn clear(&self, color: SolidColor) {
        unsafe {
            (self.gl.glClearColor)(color.r, color.g, color.b, color.a);
            (self.gl.glClear)(opengl::GL_COLOR_BUFFER_BIT);
        }
    }

    pub fn get_swap_interval(&self) -> u32 {
        self.swap_interval
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
