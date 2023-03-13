use crate::components::body::BodyComponent;
use crate::global::GlobalAppData;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_framework::app::Application;
use lemao_framework::ecs::components::ComponentManagerHashMap;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;
use std::any::TypeId;

#[derive(Default)]
pub struct BodySystem {}

impl System<GlobalAppData, GameScene, Message> for BodySystem {
    fn update(
        &mut self,
        _app: &mut Application<GlobalAppData>,
        _scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
        _input: &[InputEvent],
    ) -> Result<(), String> {
        while let Some(message) = world.bus.poll_message::<Self>() {
            match message {
                Message::GameTick => {
                    let bodies = world.components.get_component_managers_1::<BodyComponent>();

                    let mut entites_to_remove = Vec::new();
                    for body in bodies.iter_mut() {
                        body.lifetime -= 1;

                        if body.lifetime == 0 {
                            entites_to_remove.push(body.entity_id);
                        }
                    }

                    //drop(bodies);

                    for entity_id in entites_to_remove {
                        world.remove_entity(entity_id)?;
                    }
                }
            }
        }

        Ok(())
    }
}
