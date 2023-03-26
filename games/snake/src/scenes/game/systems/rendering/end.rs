use crate::scenes::game::messages::Message;
use crate::scenes::game::scene::GameScene;
use crate::scenes::game::scene::GameWorld;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::systems::SystemStage;
use std::any::TypeId;

#[derive(Default)]
pub struct FrameEndSystem {}

impl System<GlobalAppData, GameScene, Message> for FrameEndSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::FrameEnd
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn update(&mut self, app: &mut GameApp, _scene: &mut GameScene, _world: &mut GameWorld) -> Result<(), String> {
        app.window.swap_buffers();

        Ok(())
    }
}
