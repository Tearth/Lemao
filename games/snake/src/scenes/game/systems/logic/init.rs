use crate::scenes::game::messages::Message;
use crate::scenes::game::scene::GameScene;
use crate::scenes::game::scene::GameWorld;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::systems::SystemStage;
use std::any::TypeId;

#[derive(Default)]
pub struct InitSystem {}

impl System<GlobalAppData, GameScene, Message> for InitSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::Initialization
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<InitSystem>()
    }

    fn update(&mut self, _app: &mut GameApp, _scene: &mut GameScene, world: &mut GameWorld) -> Result<(), String> {
        world.messages.broadcast(Message::Init)
    }
}
