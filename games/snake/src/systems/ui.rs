use crate::messages::Message;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_framework::app::Application;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;

#[derive(Default)]
pub struct UiSystem {}

impl System<GlobalAppData, GameScene, Message> for UiSystem {
    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        scene: &mut GameScene,
        _world: &mut World<GlobalAppData, GameScene, Message>,
        input: &[InputEvent],
    ) -> Result<(), String> {
        for event in input {
            scene.ui.process_window_event(&mut app.renderer, event)?;
        }

        scene.ui.update(&mut app.renderer)?;
        scene.ui.draw(&mut app.renderer, scene.state.ui.score_label_id)?;

        Ok(())
    }
}
