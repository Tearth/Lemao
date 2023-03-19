use super::body::BodySystem;
use super::food::FoodSystem;
use super::head::HeadSystem;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::{InputEvent, Key};
use lemao_framework::app::Application;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;
use std::time::SystemTime;

#[derive(Default)]
pub struct SyncSystem {}

impl System<GlobalAppData, GameScene, Message> for SyncSystem {
    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
        _input: &[InputEvent],
    ) -> Result<(), String> {
        let mut tick = scene.state.game.tick_length;
        if app.window.is_key_pressed(Key::Space) {
            tick /= 3;
        }

        if scene.state.game.time_of_last_tick.elapsed().unwrap().as_millis() >= tick as u128 {
            world.messages.send_to_3::<HeadSystem, BodySystem, FoodSystem>(Message::GameTick)?;
            scene.state.game.time_of_last_tick = SystemTime::now();
        }

        Ok(())
    }
}
