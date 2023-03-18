use crate::components::body::BodyComponent;
use crate::components::sprite::SpriteComponent;
use crate::messages::Message;
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
                        let bodies = world.components.get_many_mut_1::<BodyComponent>();

                        for body in bodies.iter_mut() {
                            body.lifetime -= 1;

                            if body.lifetime == 0 {
                                world.commands.send(Box::new(KillCommand::new(body.entity_id)));
                            }
                        }
                    }
                }
                Message::KillSnake => {
                    let (bodies, sprites) = world.components.get_many_mut_2::<BodyComponent, SpriteComponent>();

                    for body in bodies.iter_mut() {
                        body.killed = true;

                        let sprite = sprites.get_mut(body.entity_id)?;
                        sprite.blinking = true;
                        sprite.blinking_interval = 200;
                    }
                }
                Message::ResetSnake => {
                    let bodies = world.components.get_many_mut_1::<BodyComponent>();
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
