use crate::state::global::GlobalAppData;

use self::components::body::BodyComponent;
use self::components::cell::CellComponent;
use self::components::food::FoodComponent;
use self::components::head::HeadComponent;
use self::components::obstacle::ObstacleComponent;
use self::components::position::PositionComponent;
use self::components::sprite::SpriteComponent;
use self::messages::Message;
use self::state::scene::SceneState;
use self::systems::board::BoardSystem;
use self::systems::body::BodySystem;
use self::systems::food::FoodSystem;
use self::systems::head::HeadSystem;
use self::systems::init::InitSystem;
use self::systems::renderer::RendererSystem;
use self::systems::sync::SyncSystem;
use self::systems::ui::UiSystem;
use self::systems::window::WindowSystem;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::Texture;
use lemao_framework::app::Application;
use lemao_framework::app::Scene;
use lemao_framework::assets::AssetsLoader;
use lemao_framework::ecs::components::list::ComponentList;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;
use lemao_ui::context::UiContext;
use std::any::Any;
use std::sync::Arc;
use std::sync::RwLock;

pub mod components;
pub mod messages;
pub mod state;
pub mod systems;
pub mod utils;

pub struct GameScene {
    pub ui: UiContext,
    pub world: Arc<RwLock<World<GlobalAppData, GameScene, Message>>>,
    pub state: SceneState,
}

impl GameScene {
    pub fn new(app: &mut Application<GlobalAppData>) -> Self {
        Self { ui: UiContext::new(&mut app.renderer).unwrap(), world: Arc::new(RwLock::new(World::new())), state: Default::default() }
    }
}

impl Scene<GlobalAppData> for GameScene {
    fn on_init(&mut self, app: &mut Application<GlobalAppData>) -> Result<(), String> {
        Ok(())
    }

    fn on_activation(&mut self, app: &mut Application<GlobalAppData>) -> Result<(), String> {
        self.world = Arc::new(RwLock::new(World::new()));
        self.state = Default::default();

        let world = self.world.clone();
        let mut world = world.write().unwrap();

        world.components.store::<BodyComponent>(Box::new(ComponentList::<BodyComponent>::new()))?;
        world.components.store::<CellComponent>(Box::new(ComponentList::<CellComponent>::new()))?;
        world.components.store::<FoodComponent>(Box::new(ComponentList::<FoodComponent>::new()))?;
        world.components.store::<HeadComponent>(Box::new(ComponentList::<HeadComponent>::new()))?;
        world.components.store::<ObstacleComponent>(Box::new(ComponentList::<ObstacleComponent>::new()))?;
        world.components.store::<PositionComponent>(Box::new(ComponentList::<PositionComponent>::new()))?;
        world.components.store::<SpriteComponent>(Box::new(ComponentList::<SpriteComponent>::new()))?;

        world.systems.write().unwrap().store(Box::<BodySystem>::default())?;
        world.systems.write().unwrap().store(Box::<HeadSystem>::default())?;
        world.systems.write().unwrap().store(Box::<FoodSystem>::default())?;
        world.systems.write().unwrap().store(Box::<RendererSystem>::default())?;
        world.systems.write().unwrap().store(Box::<SyncSystem>::default())?;
        world.systems.write().unwrap().store(Box::<WindowSystem>::default())?;
        world.systems.write().unwrap().store(Box::<BoardSystem>::default())?;
        world.systems.write().unwrap().store(Box::<UiSystem>::default())?;

        world.messages.register_receiver::<BodySystem>()?;
        world.messages.register_receiver::<HeadSystem>()?;
        world.messages.register_receiver::<FoodSystem>()?;
        world.messages.register_receiver::<BoardSystem>()?;
        world.messages.register_receiver::<UiSystem>()?;

        InitSystem::default().update(app, self, &mut world, &Vec::new())?;

        Ok(())
    }

    fn on_deactivation(&mut self, _app: &mut Application<GlobalAppData>) -> Result<(), String> {
        Ok(())
    }

    fn on_tick(&mut self, app: &mut Application<GlobalAppData>) -> Result<(), String> {
        let mut events = Vec::new();
        while let Some(event) = app.window.poll_event() {
            events.push(event);
        }

        app.renderer.clear(SolidColor::new_rgb(210, 150, 100, 255));

        let world = self.world.clone();
        world.write().unwrap().update(app, self, &events)?;

        app.window.swap_buffers();
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
