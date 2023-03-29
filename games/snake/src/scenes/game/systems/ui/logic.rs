use crate::scenes::game::messages::Message;
use crate::scenes::game::scene::GameScene;
use crate::scenes::game::scene::GameWorld;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::Color;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::systems::SystemStage;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;
use std::any::TypeId;

#[derive(Default)]
pub struct UiLogicSystem {}

impl System<GlobalAppData, GameScene, Message> for UiLogicSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::UiLogic
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn update(&mut self, app: &mut GameApp, scene: &mut GameScene, world: &mut GameWorld) -> Result<(), String> {
        let mut update_score = false;

        while let Some(message) = world.messages.poll_message::<Self>() {
            match message {
                Message::Init => {
                    let font_id = app.renderer.fonts.get_by_name("pixeled")?.id;

                    scene.state.ui.score_label_id = scene.ui.components.store(Label::new(&mut app.renderer, font_id)?);
                    let score_label = scene.ui.components.get_and_cast_mut::<Label>(scene.state.ui.score_label_id)?;
                    score_label.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.0));
                    score_label.anchor = Vec2::new(0.0, 0.5);
                    score_label.offset = Vec2::new(-250.0, 50.0);
                    score_label.label_text = "SCORE: 0".to_string();
                    score_label.shadow_enabled = true;
                    score_label.shadow_offset = Vec2::new(1.0, -1.0);
                    score_label.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
                    scene.ui.components.get_mut(scene.ui.main_canvas_id)?.add_child(scene.state.ui.score_label_id);

                    scene.state.ui.best_score_label_id = scene.ui.components.store(Label::new(&mut app.renderer, font_id)?);
                    let best_score_label = scene.ui.components.get_and_cast_mut::<Label>(scene.state.ui.best_score_label_id)?;
                    best_score_label.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.0));
                    best_score_label.anchor = Vec2::new(1.0, 0.5);
                    best_score_label.offset = Vec2::new(250.0, 50.0);
                    best_score_label.label_text = "BEST SCORE: 0".to_string();
                    best_score_label.shadow_enabled = true;
                    best_score_label.shadow_offset = Vec2::new(1.0, -1.0);
                    best_score_label.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
                    scene.ui.components.get_mut(scene.ui.main_canvas_id)?.add_child(scene.state.ui.best_score_label_id);

                    scene.state.ui.clock_label_id = scene.ui.components.store(Label::new(&mut app.renderer, font_id)?);
                    let clock_label = scene.ui.components.get_and_cast_mut::<Label>(scene.state.ui.clock_label_id)?;
                    clock_label.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.0));
                    clock_label.anchor = Vec2::new(0.5, 0.5);
                    clock_label.offset = Vec2::new(0.0, 50.0);
                    clock_label.label_text = "00:00".to_string();
                    clock_label.shadow_enabled = true;
                    clock_label.shadow_offset = Vec2::new(1.0, -1.0);
                    clock_label.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
                    scene.ui.components.get_mut(scene.ui.main_canvas_id)?.add_child(scene.state.ui.clock_label_id);

                    scene.state.ui.instruction_label_id = scene.ui.components.store(Label::new(&mut app.renderer, font_id)?);
                    let instruction_label = scene.ui.components.get_and_cast_mut::<Label>(scene.state.ui.instruction_label_id)?;
                    instruction_label.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.0));
                    instruction_label.anchor = Vec2::new(0.5, 0.5);
                    instruction_label.offset = Vec2::new(0.0, 90.0);
                    instruction_label.label_text = "WASD - SNAKE CONTROL        SPACE - ACCELERATION".to_string();
                    instruction_label.shadow_enabled = true;
                    instruction_label.shadow_offset = Vec2::new(1.0, -1.0);
                    instruction_label.shadow_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
                    scene.ui.components.get_mut(scene.ui.main_canvas_id)?.add_child(scene.state.ui.instruction_label_id);

                    return Ok(());
                }
                Message::InputEvent(event) => {
                    if let InputEvent::KeyPressed(Key::Escape) = event {
                        app.switch_to_scene("Menu");
                    }

                    scene.ui.process_window_event(&mut app.renderer, &event)?
                }
                Message::FoodEaten => {
                    update_score = true;
                }
                Message::ResetSnake => {
                    update_score = true;
                }
                _ => {}
            }
        }

        if update_score {
            let score_label = scene.ui.components.get_and_cast_mut::<Label>(scene.state.ui.score_label_id)?;
            score_label.label_text = format!("SCORE: {}", scene.state.game.score);
            score_label.dirty = true;

            let best_score_label = scene.ui.components.get_and_cast_mut::<Label>(scene.state.ui.best_score_label_id)?;
            best_score_label.label_text = format!("BEST SCORE: {}", scene.state.game.best_score);
            best_score_label.dirty = true;
        }

        if !scene.state.game.snake_killed {
            let time_since_game_start = scene.state.game.game_start_time.elapsed().unwrap().as_secs();
            let clock = format!("{:02}:{:02}", time_since_game_start / 60, time_since_game_start % 60);
            let clock_label = scene.ui.components.get_and_cast_mut::<Label>(scene.state.ui.clock_label_id)?;

            if clock_label.label_text != clock {
                clock_label.label_text = clock;
                clock_label.dirty = true;
            }
        }

        Ok(())
    }
}
