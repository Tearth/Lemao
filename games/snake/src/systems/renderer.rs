use crate::components::position::PositionComponent;
use crate::components::sprite::SpriteComponent;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::app::Application;
use lemao_framework::ecs::components::ComponentManagerHashMap;
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
        let (sprites, positions) = world.components.get_many_mut_2::<SpriteComponent, PositionComponent>();

        for sprite in sprites.iter_mut() {
            let position = positions.get_mut(sprite.entity_id)?;
            if position.changed {
                sprite.rectangle.position = Vec2::new(position.col as f32, position.row as f32) * app.global_data.cell_size;
                position.changed = false;
            }

            app.renderer.draw(&mut sprite.rectangle)?;
        }

        Ok(())
    }
}
