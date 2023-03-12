use super::body::BodySystem;
use super::head::HeadSystem;
use crate::global::GlobalAppData;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_framework::app::Application;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;
use std::time::SystemTime;

#[derive(Default)]
pub struct SyncSystem {}

impl System<GlobalAppData, GameScene, Message> for SyncSystem {
    fn update(
        &mut self,
        _app: &mut Application<GlobalAppData>,
        scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
        _input: &[InputEvent],
    ) -> Result<(), String> {
        if scene.time_of_last_tick.elapsed().unwrap().as_millis() >= scene.tick_length as u128 {
            world.bus.send_to_2::<HeadSystem, BodySystem>(Message::GameTick)?;
            scene.time_of_last_tick = SystemTime::now();
        }

        Ok(())
    }
}
