use crate::utils::storage::SceneStorage;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::renderer::context::RendererContext;
use lemao_core::window::context::WindowContext;
use std::any::Any;
use std::sync::Arc;
use std::sync::RwLock;

pub struct Application<G> {
    window: WindowContext,
    renderer: RendererContext,
    global_data: G,

    running: bool,
    default_scene: String,
    current_scene: String,
    pending_scene: String,
    scenes: Arc<RwLock<SceneStorage<G>>>,
}

pub trait Scene<G> {
    fn on_init(&mut self, app: &mut Application<G>) -> Result<(), String>;
    fn on_activation(&mut self, app: &mut Application<G>) -> Result<(), String>;
    fn on_deactivation(&mut self, app: &mut Application<G>) -> Result<(), String>;
    fn on_tick(&mut self, app: &mut Application<G>) -> Result<(), String>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<G> Application<G>
where
    G: Default,
{
    pub fn new(window_title: &str, window_style: WindowStyle) -> Result<Self, String> {
        let mut window = WindowContext::new(window_title, window_style)?;
        let mut renderer = window.create_renderer()?;
        renderer.set_swap_interval(1);

        Ok(Self {
            window,
            renderer,
            global_data: Default::default(),
            running: true,
            default_scene: Default::default(),
            current_scene: Default::default(),
            pending_scene: Default::default(),
            scenes: Default::default(),
        })
    }

    pub fn get_renderer(&self) -> &RendererContext {
        &self.renderer
    }

    pub fn get_renderer_mut(&mut self) -> &mut RendererContext {
        &mut self.renderer
    }

    pub fn get_window(&self) -> &WindowContext {
        &self.window
    }

    pub fn get_window_mut(&mut self) -> &mut WindowContext {
        &mut self.window
    }

    pub fn get_global_data(&self) -> &G {
        &self.global_data
    }

    pub fn get_global_data_mut(&mut self) -> &mut G {
        &mut self.global_data
    }

    pub fn register_scene(mut self, name: &str, mut scene: Box<dyn Scene<G>>, default: bool) -> Result<Self, String> {
        if default {
            self.default_scene = name.to_string();
            self.pending_scene = name.to_string();
        }

        scene.on_init(&mut self)?;

        let scene_storage = self.scenes.clone();
        let mut scene_storage_lock = scene_storage.write().unwrap();
        scene_storage_lock.store(name, scene);

        Ok(self)
    }

    pub fn run(mut self) -> Result<(), String> {
        self.current_scene = self.default_scene.clone();

        while self.running {
            if self.current_scene != self.pending_scene {
                let scene_storage = self.scenes.clone();
                let mut scene_storage_lock = scene_storage.write().unwrap();

                let current_scene = scene_storage_lock.get_mut(&self.current_scene)?;
                current_scene.on_deactivation(&mut self)?;

                let scene = scene_storage_lock.get_mut(&self.pending_scene)?;
                scene.on_activation(&mut self)?;

                self.current_scene = self.pending_scene.to_string();
            }

            let scene_storage = self.scenes.clone();
            let mut scene_storage_lock = scene_storage.write().unwrap();
            let scene = scene_storage_lock.get_mut(&self.current_scene)?;

            scene.on_tick(&mut self)?;
        }

        Ok(())
    }

    pub fn switch_to_scene(&mut self, name: &str) {
        self.pending_scene = name.to_string();
    }

    pub fn close(&mut self) {
        self.running = false;
    }
}
