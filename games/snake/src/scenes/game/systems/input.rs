use std::any::TypeId;

use crate::scenes::game::messages::Message;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_framework::app::Application;
use lemao_framework::ecs::systems::{System, SystemStage};
use lemao_framework::ecs::world::World;

#[derive(Default)]
pub struct InputSystem {}

impl System<GlobalAppData, GameScene, Message> for InputSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::Input
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<InputSystem>()
    }

    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        _scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
    ) -> Result<(), String> {
        while let Some(event) = app.window.poll_event() {
            world.messages.broadcast(Message::InputEvent(event))?;
        }

        Ok(())
    }
}
