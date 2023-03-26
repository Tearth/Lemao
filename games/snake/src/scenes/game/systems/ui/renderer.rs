use crate::scenes::game::messages::Message;
use crate::scenes::game::scene::GameScene;
use crate::scenes::game::scene::GameWorld;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::systems::SystemStage;
use std::any::TypeId;

#[derive(Default)]
pub struct UiRenderingSystem {}

impl System<GlobalAppData, GameScene, Message> for UiRenderingSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::UiRendering
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn update(&mut self, app: &mut GameApp, scene: &mut GameScene, _world: &mut GameWorld) -> Result<(), String> {
        scene.ui.update(&mut app.renderer)?;
        scene.ui.draw(&mut app.renderer, scene.state.ui.score_label_id)?;
        scene.ui.draw(&mut app.renderer, scene.state.ui.best_score_label_id)?;
        scene.ui.draw(&mut app.renderer, scene.state.ui.clock_label_id)?;

        Ok(())
    }
}
