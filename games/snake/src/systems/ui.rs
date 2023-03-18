use crate::messages::Message;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::Color;
use lemao_framework::app::Application;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;

#[derive(Default)]
pub struct UiSystem {}

impl System<GlobalAppData, GameScene, Message> for UiSystem {
    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
        input: &[InputEvent],
    ) -> Result<(), String> {
        while let Some(message) = world.messages.poll_message::<Self>() {
            match message {
                Message::Init => {
                    let font_id = app.renderer.fonts.get_by_name("pixeled")?.id;
                    scene.state.ui.score_label_id = scene.ui.create_label(&mut app.renderer, font_id)?;
                    let score_label = scene.ui.get_component_and_cast_mut::<Label>(scene.state.ui.score_label_id)?;
                    score_label.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.0));
                    score_label.anchor = Vec2::new(0.5, 0.5);
                    score_label.offset = Vec2::new(0.0, 50.0);
                    score_label.label_text = "SCORE: 0".to_string();
                    score_label.shadow_enabled = true;
                    score_label.shadow_offset = Vec2::new(1.0, -1.0);
                    score_label.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
                    scene.ui.get_component_mut(scene.ui.main_canvas_id)?.add_child(scene.state.ui.score_label_id);
                }
                _ => {}
            }
        }

        for event in input {
            scene.ui.process_window_event(&mut app.renderer, event)?;
        }

        scene.ui.update(&mut app.renderer)?;
        scene.ui.draw(&mut app.renderer, scene.state.ui.score_label_id)?;

        Ok(())
    }
}
