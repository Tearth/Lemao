use crate::scenes::game::components::body::{BodyComponent, BodyOrientation};
use crate::scenes::game::components::head::HeadDirection;
use crate::scenes::game::components::position::PositionComponent;
use crate::scenes::game::components::sprite::SpriteComponent;
use crate::scenes::game::messages::Message;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_framework::app::Application;
use lemao_framework::ecs::commands::kill::KillCommand;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;

#[derive(Default)]
pub struct BodySystem {}

impl System<GlobalAppData, GameScene, Message> for BodySystem {
    fn update(
        &mut self,
        _app: &mut Application<GlobalAppData>,
        scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
        _input: &[InputEvent],
    ) -> Result<(), String> {
        while let Some(message) = world.messages.poll_message::<Self>() {
            match message {
                Message::GameTick => {
                    if !scene.state.game.snake_killed {
                        let (bodies, positions, sprites) = world.components.get_and_cast_mut_3::<BodyComponent, PositionComponent, SpriteComponent>();
                        let mut body_positions = Vec::new();

                        for body in bodies.iter_mut().filter(|b| !b.killed) {
                            body.lifetime -= 1;

                            if body.lifetime == 0 {
                                world.commands.send(Box::new(KillCommand::new(body.entity_id)));
                            } else if body.lifetime == 1 {
                                let sprite = sprites.get_mut(body.entity_id)?;
                                sprite.tilemap.frame = match body.direction {
                                    HeadDirection::Up => BodyOrientation::BottomEnd,
                                    HeadDirection::Down => BodyOrientation::TopEnd,
                                    HeadDirection::Right => BodyOrientation::LeftEnd,
                                    HeadDirection::Left => BodyOrientation::RightEnd,
                                } as u32;
                                sprite.tilemap.update();
                            } else {
                                body_positions.push((body.entity_id, positions.get(body.entity_id)?.coordinates))
                            }
                        }
                    }
                }
                Message::KillSnake => {
                    let (bodies, sprites) = world.components.get_and_cast_mut_2::<BodyComponent, SpriteComponent>();

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
                        world.commands.send(Box::new(KillCommand::new(body.entity_id)));
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
