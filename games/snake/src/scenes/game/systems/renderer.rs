use std::time::SystemTime;

use crate::scenes::game::components::position::PositionComponent;
use crate::scenes::game::components::sprite::SpriteComponent;
use crate::scenes::game::messages::Message;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::app::Application;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;

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
        let mut layers = Vec::new();

        for sprite in sprites.iter_mut() {
            let position = positions.get_mut(sprite.entity_id)?;
            if position.changed {
                sprite.tilemap.position = Vec2::new(position.coordinates.col as f32, position.coordinates.row as f32) * app.global_data.cell_size;
                sprite.tilemap.position = sprite.tilemap.position.floor();
                position.changed = false;
            }

            if sprite.blinking {
                if sprite.blinking_last_change_time.elapsed().unwrap().as_millis() >= sprite.blinking_interval as u128 {
                    let alpha = 1.0 - sprite.tilemap.color.get_alpha();
                    sprite.tilemap.color.set_alpha(alpha);
                    sprite.blinking_last_change_time = SystemTime::now();
                }
            } else {
                sprite.tilemap.color.set_alpha(1.0);
            }

            if sprite.layer as usize >= layers.len() {
                layers.resize(sprite.layer as usize + 1, Vec::new());
            }

            layers[sprite.layer as usize].push(sprite.entity_id);
        }

        for layer in layers.iter().rev() {
            for entity_id in layer {
                app.renderer.draw(&mut world.components.get_many_mut_1::<SpriteComponent>().get_mut(*entity_id)?.tilemap)?;
            }
        }

        Ok(())
    }
}
