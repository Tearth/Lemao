use crate::components::cell::CellComponent;
use crate::components::head::{HeadComponent, HeadDirection};
use crate::components::obstacle::ObstacleComponent;
use crate::components::position::PositionComponent;
use crate::components::sprite::SpriteComponent;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::Color;
use lemao_framework::app::Application;
use lemao_framework::ecs::commands::spawn::SpawnCommand;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;

#[derive(Default)]
pub struct InitSystem {}

impl System<GlobalAppData, GameScene, Message> for InitSystem {
    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
        _input: &[InputEvent],
    ) -> Result<(), String> {
        self.init_board(app, world)?;
        self.init_ui(app, scene)?;

        Ok(())
    }
}

impl InitSystem {
    fn init_board(&mut self, app: &mut Application<GlobalAppData>, world: &mut World<GlobalAppData, GameScene, Message>) -> Result<(), String> {
        for row in 0..app.global_data.board_height {
            for col in 0..app.global_data.board_width {
                let cell_id = world.entities.create();
                let mut rectangle = app.renderer.create_rectangle()?;
                rectangle.size = app.global_data.cell_size;

                if row == 0 || row == app.global_data.board_height - 1 || col == 0 || col == app.global_data.board_width - 1 {
                    rectangle.set_texture(app.renderer.textures.get_by_name("border")?);
                    world.commands.send(Box::new(SpawnCommand::new(cell_id, ObstacleComponent::new(cell_id))));
                } else {
                    rectangle.set_texture(app.renderer.textures.get_by_name("cell")?);
                    world.commands.send(Box::new(SpawnCommand::new(cell_id, CellComponent::new(cell_id))));
                }

                rectangle.update();

                world.commands.send(Box::new(SpawnCommand::new(cell_id, PositionComponent::new(cell_id, row, col))));
                world.commands.send(Box::new(SpawnCommand::new(cell_id, SpriteComponent::new(cell_id, rectangle))));
            }
        }

        let window_size = app.window.get_size();
        let mut camera = app.renderer.cameras.get_mut(app.renderer.active_camera_id)?;
        camera.position = Vec2::new(
            -(window_size.x - (app.global_data.board_width as f32 * app.global_data.cell_size.x)) / 2.0,
            -(window_size.y - (app.global_data.board_height as f32 * app.global_data.cell_size.y)) / 2.0,
        );

        let head_id = world.entities.create();
        let mut head_rectangle = app.renderer.create_rectangle()?;
        head_rectangle.size = app.global_data.cell_size;
        head_rectangle.set_texture(app.renderer.textures.get_by_name("head")?);
        head_rectangle.update();

        world.commands.send(Box::new(SpawnCommand::new(head_id, HeadComponent::new(head_id, HeadDirection::Right))));
        world
            .commands
            .send(Box::new(SpawnCommand::new(head_id, PositionComponent::new(head_id, app.global_data.board_height / 2, app.global_data.board_width / 2))));
        world.commands.send(Box::new(SpawnCommand::new(head_id, SpriteComponent::new(head_id, head_rectangle))));

        Ok(())
    }

    fn init_ui(&mut self, app: &mut Application<GlobalAppData>, scene: &mut GameScene) -> Result<(), String> {
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

        Ok(())
    }
}
