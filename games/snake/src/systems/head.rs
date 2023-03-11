use crate::global::GlobalAppData;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_framework::app::Application;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;

#[derive(Default)]
pub struct HeadSystem {}

impl System<GlobalAppData, GameScene, Message> for HeadSystem {
    fn update(
        &mut self,
        _app: &mut Application<GlobalAppData>,
        _scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
        _input: &[InputEvent],
    ) -> Result<(), String> {
        while let Some(message) = world.bus.poll_message::<HeadSystem>() {
            match message {
                Message::GameTick => {
                    println!("TICK");
                }
            }
        }

        Ok(())
    }
}
