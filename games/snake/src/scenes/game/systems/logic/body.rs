use crate::scenes::game::components::body::BodyComponent;
use crate::scenes::game::components::body::BodyOrientation;
use crate::scenes::game::components::sprite::SpriteComponent;
use crate::scenes::game::messages::Message;
use crate::scenes::game::scene::{GameScene, GameWorld};
use crate::scenes::game::utils::Direction;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_framework::ecs::commands::kill::KillCommand;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::systems::SystemStage;
use std::any::TypeId;
use std::cmp;

#[derive(Default)]
pub struct BodySystem {}

impl System<GlobalAppData, GameScene, Message> for BodySystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::GameLogic
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn update(&mut self, _app: &mut GameApp, scene: &mut GameScene, world: &mut GameWorld) -> Result<(), String> {
        let mut tick = false;

        while let Some(message) = world.messages.poll_message::<Self>() {
            match message {
                Message::GameTick => {
                    tick = true;
                }
                Message::FoodEaten => {
                    let bodies = world.components.get_and_cast_mut::<BodyComponent>()?;
                    for body in bodies.iter_mut() {
                        body.lifetime = cmp::min(body.lifetime + 1, scene.state.game.lifetime);
                    }
                }
                Message::KillSnake => {
                    let (bodies, sprites) = world.components.get_and_cast_mut_2::<BodyComponent, SpriteComponent>()?;

                    for body in bodies.iter_mut() {
                        body.killed = true;

                        let sprite = sprites.get_mut(body.entity_id)?;
                        sprite.blinking = true;
                        sprite.blinking_interval = 200;
                        sprite.blinking_last_change_time = scene.state.game.snake_killed_time;
                    }
                }
                Message::ResetSnake => {
                    let bodies = world.components.get_and_cast_mut::<BodyComponent>()?;
                    for body in bodies.iter() {
                        world.commands.send(KillCommand::new(body.entity_id));
                    }
                }
                _ => {}
            }
        }

        if tick && !scene.state.game.snake_killed {
            let (bodies, sprites) = world.components.get_and_cast_mut_2::<BodyComponent, SpriteComponent>()?;

            for body in bodies.iter_mut().filter(|b| !b.killed) {
                body.lifetime -= 1;

                if body.lifetime == 0 {
                    world.commands.send(KillCommand::new(body.entity_id));
                } else if body.lifetime == 1 {
                    let sprite = sprites.get_mut(body.entity_id)?;
                    sprite.tilemap.frame = match body.direction {
                        Direction::Up => BodyOrientation::BottomEnd,
                        Direction::Down => BodyOrientation::TopEnd,
                        Direction::Right => BodyOrientation::LeftEnd,
                        Direction::Left => BodyOrientation::RightEnd,
                    } as u32;
                    sprite.tilemap.update();
                }
            }
        }

        Ok(())
    }
}
