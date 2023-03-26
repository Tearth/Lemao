use super::components::body::BodyComponent;
use super::components::cell::CellComponent;
use super::components::food::FoodComponent;
use super::components::head::HeadComponent;
use super::components::obstacle::ObstacleComponent;
use super::components::position::PositionComponent;
use super::components::sprite::SpriteComponent;
use super::messages::Message;
use super::state::SceneState;
use super::systems::audio::player::AudioPlayerSystem;
use super::systems::core::input::InputSystem;
use super::systems::core::window::WindowSystem;
use super::systems::logic::board::BoardSystem;
use super::systems::logic::body::BodySystem;
use super::systems::logic::food::FoodSystem;
use super::systems::logic::head::HeadSystem;
use super::systems::logic::init::InitSystem;
use super::systems::logic::sync::SyncSystem;
use super::systems::rendering::begin::FrameBeginSystem;
use super::systems::rendering::end::FrameEndSystem;
use super::systems::rendering::renderer::RendererSystem;
use super::systems::ui::logic::UiLogicSystem;
use super::systems::ui::renderer::UiRenderingSystem;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_framework::app::Scene;
use lemao_framework::ecs::components::list::ComponentList;
use lemao_framework::ecs::world::World;
use lemao_ui::context::UiContext;
use std::any::Any;
use std::sync::Arc;
use std::sync::RwLock;

pub type GameWorld = World<GlobalAppData, GameScene, Message>;

pub struct GameScene {
    pub ui: UiContext,
    pub world: Arc<RwLock<GameWorld>>,
    pub state: SceneState,
}

impl GameScene {
    pub fn new(app: &mut GameApp) -> Self {
        Self { ui: UiContext::new(&mut app.renderer).unwrap(), world: Arc::new(RwLock::new(World::default())), state: Default::default() }
    }
}

impl Scene<GlobalAppData> for GameScene {
    fn on_init(&mut self, _app: &mut GameApp) -> Result<(), String> {
        Ok(())
    }

    fn on_activation(&mut self, _app: &mut GameApp) -> Result<(), String> {
        self.world = Arc::new(RwLock::new(World::default()));
        self.state = Default::default();

        let world = self.world.clone();
        let mut world = world.write().unwrap();

        world.components.store::<BodyComponent>(Box::<ComponentList<BodyComponent>>::default())?;
        world.components.store::<CellComponent>(Box::<ComponentList<CellComponent>>::default())?;
        world.components.store::<FoodComponent>(Box::<ComponentList<FoodComponent>>::default())?;
        world.components.store::<HeadComponent>(Box::<ComponentList<HeadComponent>>::default())?;
        world.components.store::<ObstacleComponent>(Box::<ComponentList<ObstacleComponent>>::default())?;
        world.components.store::<PositionComponent>(Box::<ComponentList<PositionComponent>>::default())?;
        world.components.store::<SpriteComponent>(Box::<ComponentList<SpriteComponent>>::default())?;

        // Core
        world.systems.write().unwrap().store::<InputSystem>(Box::<InputSystem>::default())?;
        world.systems.write().unwrap().store::<WindowSystem>(Box::<WindowSystem>::default())?;

        // Logic
        world.systems.write().unwrap().store::<InitSystem>(Box::<InitSystem>::default())?;
        world.systems.write().unwrap().store::<BodySystem>(Box::<BodySystem>::default())?;
        world.systems.write().unwrap().store::<HeadSystem>(Box::<HeadSystem>::default())?;
        world.systems.write().unwrap().store::<FoodSystem>(Box::<FoodSystem>::default())?;
        world.systems.write().unwrap().store::<SyncSystem>(Box::<SyncSystem>::default())?;
        world.systems.write().unwrap().store::<BoardSystem>(Box::<BoardSystem>::default())?;

        // Audio
        world.systems.write().unwrap().store::<AudioPlayerSystem>(Box::<AudioPlayerSystem>::default())?;

        // UI
        world.systems.write().unwrap().store::<UiLogicSystem>(Box::<UiLogicSystem>::default())?;
        world.systems.write().unwrap().store::<UiRenderingSystem>(Box::<UiRenderingSystem>::default())?;

        // Rendering
        world.systems.write().unwrap().store::<FrameBeginSystem>(Box::<FrameBeginSystem>::default())?;
        world.systems.write().unwrap().store::<RendererSystem>(Box::<RendererSystem>::default())?;
        world.systems.write().unwrap().store::<FrameEndSystem>(Box::<FrameEndSystem>::default())?;

        world.messages.register_receiver::<AudioPlayerSystem>()?;
        world.messages.register_receiver::<BoardSystem>()?;
        world.messages.register_receiver::<BodySystem>()?;
        world.messages.register_receiver::<FoodSystem>()?;
        world.messages.register_receiver::<HeadSystem>()?;
        world.messages.register_receiver::<UiLogicSystem>()?;
        world.messages.register_receiver::<WindowSystem>()?;

        Ok(())
    }

    fn on_deactivation(&mut self, app: &mut GameApp) -> Result<(), String> {
        for sound in app.audio.sounds.iter_mut() {
            sound.stop()?;
        }

        Ok(())
    }

    fn on_tick(&mut self, app: &mut GameApp) -> Result<(), String> {
        let world = self.world.clone();
        world.write().unwrap().update(app, self)?;

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
