use std::any::TypeId;

use crate::scenes::game::messages::Message;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::{InputEvent, Key};
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::Color;
use lemao_framework::app::Application;
use lemao_framework::ecs::systems::{System, SystemStage};
use lemao_framework::ecs::world::World;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;

#[derive(Default)]
pub struct UiRenderingSystem {}

impl System<GlobalAppData, GameScene, Message> for UiRenderingSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::UiRendering
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<UiRenderingSystem>()
    }

    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
    ) -> Result<(), String> {
        scene.ui.update(&mut app.renderer)?;
        scene.ui.draw(&mut app.renderer, scene.state.ui.score_label_id)?;
        scene.ui.draw(&mut app.renderer, scene.state.ui.best_score_label_id)?;
        scene.ui.draw(&mut app.renderer, scene.state.ui.clock_label_id)?;

        Ok(())
    }
}
