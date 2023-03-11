use crate::components::sprite::SpriteComponent;
use crate::global::GlobalAppData;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_framework::app::Application;
use lemao_framework::ecs::components::ComponentManager;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;
use std::any::TypeId;

#[derive(Default)]
pub struct RendererSystem {}

impl System<GlobalAppData, GameScene, Message> for RendererSystem {
    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        _scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
        _input: &[InputEvent],
    ) -> Result<(), String> {
        let sprites = world.components.get_mut(&TypeId::of::<SpriteComponent>()).unwrap().clone();
        let mut sprites = sprites.write().unwrap();
        let sprites = sprites.as_any_mut().downcast_mut::<ComponentManager<SpriteComponent>>().unwrap();

        for sprite in sprites.iter_mut() {
            app.renderer.draw(&mut sprite.rectangle)?;
        }

        Ok(())
    }
}
