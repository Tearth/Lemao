use crate::scenes::game::components::position::PositionComponent;
use crate::scenes::game::components::sprite::SpriteComponent;
use crate::scenes::game::messages::Message;
use crate::scenes::game::scene::GameScene;
use crate::scenes::game::scene::GameWorld;
use crate::scenes::game::utils::Direction;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::systems::SystemStage;
use std::any::TypeId;
use std::time::SystemTime;

#[derive(Default)]
pub struct RendererSystem {}

impl System<GlobalAppData, GameScene, Message> for RendererSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::GameRendering
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn update(&mut self, app: &mut GameApp, _scene: &mut GameScene, world: &mut GameWorld) -> Result<(), String> {
        let (sprites, positions) = world.components.get_and_cast_mut_2::<SpriteComponent, PositionComponent>()?;
        let mut layers = Vec::new();

        for sprite in sprites.iter_mut() {
            let position = positions.get_mut(sprite.entity_id)?;
            if position.changed {
                sprite.tilemap.position = Vec2::new(position.coordinates.col as f32, position.coordinates.row as f32) * app.global_data.cell_size;
                sprite.tilemap.position = sprite.tilemap.position.floor();
                sprite.tilemap.rotation = match position.direction {
                    Some(Direction::Up) => 0.50 * 2.0 * std::f32::consts::PI,
                    Some(Direction::Down) => 0.00 * 2.0 * std::f32::consts::PI,
                    Some(Direction::Right) => 0.25 * 2.0 * std::f32::consts::PI,
                    Some(Direction::Left) => 0.75 * 2.0 * std::f32::consts::PI,
                    _ => 0.0,
                };
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
                app.renderer.draw(&mut world.components.get_and_cast_mut::<SpriteComponent>()?.get_mut(*entity_id)?.tilemap)?;
            }
        }

        Ok(())
    }
}
