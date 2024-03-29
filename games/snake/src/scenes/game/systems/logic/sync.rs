use super::body::BodySystem;
use super::food::FoodSystem;
use super::head::HeadSystem;
use crate::scenes::game::messages::Message;
use crate::scenes::game::scene::GameScene;
use crate::scenes::game::scene::GameWorld;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_core::lemao_common_platform::input::Key;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::systems::SystemStage;
use std::any::TypeId;
use std::time::SystemTime;

#[derive(Default)]
pub struct SyncSystem {}

impl System<GlobalAppData, GameScene, Message> for SyncSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::GameLogic
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn update(&mut self, app: &mut GameApp, scene: &mut GameScene, world: &mut GameWorld) -> Result<(), String> {
        let mut tick = scene.state.game.tick_length;
        if app.window.is_key_pressed(Key::Space) {
            tick /= app.global_data.space_acceleration;
        }

        if scene.state.game.time_of_last_tick.elapsed().unwrap().as_millis() >= tick as u128 {
            world.messages.send_to_3::<HeadSystem, BodySystem, FoodSystem>(Message::GameTick)?;
            scene.state.game.time_of_last_tick = SystemTime::now();
        }

        Ok(())
    }
}
