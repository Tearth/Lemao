use super::batcher::BatchRenderer;
use super::cameras::storage::CameraStorage;
use super::cameras::Camera;
use super::drawable::circle::Circle;
use super::drawable::disc::Disc;
use super::drawable::frame::Frame;
use super::drawable::line::Line;
use super::drawable::rectangle::Rectangle;
use super::drawable::storage::DrawableStorage;
use super::drawable::text::Text;
use super::drawable::tilemap::Tilemap;
use super::drawable::Color;
use super::drawable::Drawable;
use super::fonts::storage::FontStorage;
use super::shaders::storage::ShaderStorage;
use super::shaders::Shader;
use super::shaders::DEFAULT_VERTEX_SHADER;
use super::shaders::GRADIENT_FRAGMENT_SHADER;
use super::shaders::SOLID_FRAGMENT_SHADER;
use super::shapes::storage::ShapeStorage;
use super::shapes::Shape;
use super::textures::storage::TextureStorage;
use super::textures::RawTexture;
use super::textures::Texture;
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

    renderer_platform_specific: Box<dyn RendererPlatformSpecific>,
    textures: Arc<RwLock<TextureStorage>>,
    fonts: Arc<RwLock<FontStorage>>,
    cameras: Option<CameraStorage>,
    shaders: Option<ShaderStorage>,
    drawables: Option<DrawableStorage>,
    shapes: Option<ShapeStorage>,
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

            renderer_platform_specific,
            textures: Default::default(),
            fonts: Default::default(),
            shaders: Some(Default::default()),
            drawables: Some(Default::default()),
            cameras: Some(Default::default()),
            shapes: Some(Default::default()),
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
        self.init_default_shapes();
        self.init_default_texture();
        self.init_batch_renderer();

        Ok(())
    }

    pub fn init_default_camera(&mut self) -> Result<(), String> {
        let camera = Camera::new(Default::default(), Default::default());
        self.default_camera_id = self.cameras.as_mut().unwrap().store(camera);
        self.set_default_camera()?;

        Ok(())
    }

    pub fn init_default_shaders(&mut self) -> Result<(), String> {
        let solid_shader = Shader::new(self, DEFAULT_VERTEX_SHADER, SOLID_FRAGMENT_SHADER)?;
        self.default_solid_shader_id = self.shaders.as_mut().unwrap().store(solid_shader);

        let gradient_shader = Shader::new(self, DEFAULT_VERTEX_SHADER, GRADIENT_FRAGMENT_SHADER)?;
        self.default_gradient_shader_id = self.shaders.as_mut().unwrap().store(gradient_shader);

        Ok(())
    }

    pub fn init_default_shapes(&mut self) {
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
        self.default_sprite_shape_id = self.shapes.as_mut().unwrap().store(sprite_shape);

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
        self.default_line_shape_id = self.shapes.as_mut().unwrap().store(line_shape);

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
        self.default_rectangle_shape_id = self.shapes.as_mut().unwrap().store(rectangle_shape);
    }

    pub fn init_default_texture(&mut self) {
        let texture = Texture::new(self, &RawTexture::new(Vec2::new(1.0, 1.0), vec![255, 255, 255, 255]));
        self.default_texture_id = self.textures.write().unwrap().store(texture);
    }

    pub fn init_batch_renderer(&mut self) {
        self.batch_renderer = Some(BatchRenderer::new(self, 1024 * 1024, 1024 * 100));
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

    pub fn set_default_camera(&mut self) -> Result<(), String> {
        self.set_camera_as_active(self.default_camera_id)
    }

    pub fn get_fonts(&self) -> Arc<RwLock<FontStorage>> {
        self.fonts.clone()
    }

    pub fn get_textures(&self) -> Arc<RwLock<TextureStorage>> {
        self.textures.clone()
    }

    pub fn create_circle(&mut self, radius: f32, sides: u32) -> Result<usize, String> {
        let texture_storage = self.textures.read().unwrap();
        let texture = texture_storage.get(self.default_texture_id)?;
        let circle = Box::new(Circle::new(self, texture, radius, sides));

        Ok(self.drawables.as_mut().unwrap().store_circle(circle))
    }

    pub fn create_disc(&mut self, radius: f32, sides: u32) -> Result<usize, String> {
        let texture_storage = self.textures.read().unwrap();
        let texture = texture_storage.get(self.default_texture_id)?;
        let disc = Box::new(Disc::new(self, texture, radius, sides));

        Ok(self.drawables.as_mut().unwrap().store_disc(disc))
    }

    pub fn create_frame(&mut self, size: Vec2) -> Result<usize, String> {
        let texture_storage = self.textures.read().unwrap();
        let texture = texture_storage.get(self.default_texture_id)?;
        let frame = Box::new(Frame::new(self, texture, size));

        Ok(self.drawables.as_mut().unwrap().store_frame(frame))
    }

    pub fn create_line(&mut self, from: Vec2, to: Vec2) -> Result<usize, String> {
        let shape = self.shapes.as_ref().unwrap().get(self.default_line_shape_id)?;
        let texture_storage = self.textures.read().unwrap();
        let texture = texture_storage.get(self.default_texture_id)?;
        let line = Box::new(Line::new(self, shape, texture, from, to));

        Ok(self.drawables.as_mut().unwrap().store_line(line))
    }

    pub fn create_rectangle(&mut self) -> Result<usize, String> {
        let shape = self.shapes.as_ref().unwrap().get(self.default_rectangle_shape_id)?;
        let texture_storage = self.textures.read().unwrap();
        let texture = texture_storage.get(self.default_texture_id)?;
        let rectangle = Box::new(Rectangle::new(self, shape, texture));

        Ok(self.drawables.as_mut().unwrap().store_rectangle(rectangle))
    }

    pub fn create_text(&mut self, font_id: usize) -> Result<usize, String> {
        let font_storage = self.fonts.read().unwrap();
        let font = font_storage.get(font_id)?;
        let text = Box::new(Text::new(self, font));

        Ok(self.drawables.as_mut().unwrap().store_text(text))
    }

    pub fn create_tilemap(&mut self, texture_id: usize, tile_size: Vec2) -> Result<usize, String> {
        let texture_storage = self.textures.read().unwrap();
        let texture = texture_storage.get(texture_id)?;
        let tilemap = Box::new(Tilemap::new(self, texture, tile_size));

        Ok(self.drawables.as_mut().unwrap().store_tilemap(tilemap))
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

    pub fn remove_drawable(&mut self, drawable_id: usize) -> Result<(), String> {
        self.drawables.as_mut().unwrap().remove(drawable_id)
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

    pub fn batcher_add_drawable(&mut self, drawable_id: usize) -> Result<(), String> {
        let drawable = self.drawables.as_ref().unwrap().get(drawable_id)?;
        let mut batch = drawable.get_batch();

        if let Some(shape_id) = batch.shape_id {
            let shape = self.shapes.as_ref().unwrap().get(shape_id)?;
            batch.vertices = Some(shape.get_vertices());
            batch.indices = Some(shape.get_indices());
        }

        self.batch_renderer.as_mut().unwrap().add(drawable, &batch)?;
        Ok(())
    }

    pub fn batcher_draw(&mut self) -> Result<(), String> {
        let shader_id = match self.batch_renderer.as_ref().unwrap().get_color() {
            Color::SolidColor(_) => self.default_solid_shader_id,
            Color::Gradient(_) => self.default_gradient_shader_id,
        };

        if shader_id != self.active_shader_id || self.cameras.as_mut().unwrap().get_mut(self.active_camera_id)?.is_dirty() {
            self.set_shader_as_active(shader_id)?;

            let camera = self.cameras.as_mut().unwrap().get_mut(self.active_camera_id)?;
            let shader = self.shaders.as_ref().unwrap().get(shader_id)?;
            shader.set_parameter("proj", camera.get_projection_matrix().as_ptr())?;
            shader.set_parameter("view", camera.get_view_matrix().as_ptr())?;
            camera.set_dirty_flag(false);
        }

        self.batch_renderer.as_mut().unwrap().draw(self.shaders.as_ref().unwrap().get(shader_id)?)
    }

    pub fn draw(&mut self, drawable_id: usize) -> Result<(), String> {
        let drawable = self.drawables.as_ref().unwrap().get(drawable_id)?;
        let shader_id = match drawable.get_color() {
            Color::SolidColor(_) => self.default_solid_shader_id,
            Color::Gradient(_) => self.default_gradient_shader_id,
        };

        if shader_id != self.active_shader_id || self.cameras.as_mut().unwrap().get_mut(self.active_camera_id)?.is_dirty() {
            self.set_shader_as_active(shader_id)?;

            let camera = self.cameras.as_mut().unwrap().get_mut(self.active_camera_id)?;
            let shader = self.shaders.as_ref().unwrap().get(shader_id)?;
            shader.set_parameter("proj", camera.get_projection_matrix().as_ptr())?;
            shader.set_parameter("view", camera.get_view_matrix().as_ptr())?;
            camera.set_dirty_flag(false);
        }

        self.get_drawable(drawable_id)?.draw(self.shaders.as_ref().unwrap().get(shader_id)?)?;
        Ok(())
    }

    pub fn clear(&self, color: SolidColor) {
        unsafe {
            (self.gl.glClearColor)(color.r, color.g, color.b, color.a);
            (self.gl.glClear)(opengl::GL_COLOR_BUFFER_BIT);
        }
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
