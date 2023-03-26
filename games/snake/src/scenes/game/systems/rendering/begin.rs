use crate::scenes::game::messages::Message;
use crate::scenes::game::scene::GameScene;
use crate::scenes::game::scene::GameWorld;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_core::lemao_math::color::SolidColor;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::systems::SystemStage;
use std::any::TypeId;

#[derive(Default)]
pub struct FrameBeginSystem {}

impl System<GlobalAppData, GameScene, Message> for FrameBeginSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::GameRendering
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn update(&mut self, app: &mut GameApp, _scene: &mut GameScene, _world: &mut GameWorld) -> Result<(), String> {
        app.renderer.clear(SolidColor::new_rgb(210, 150, 100, 255));

        Ok(())
    }
}
