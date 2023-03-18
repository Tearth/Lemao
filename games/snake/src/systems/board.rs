use crate::components::cell::CellComponent;
use crate::components::obstacle::ObstacleComponent;
use crate::components::position::PositionComponent;
use crate::components::sprite::SpriteComponent;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::app::Application;
use lemao_framework::ecs::commands::spawn::SpawnCommand;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;

#[derive(Default)]
pub struct BoardSystem {}

impl System<GlobalAppData, GameScene, Message> for BoardSystem {
    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        _scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
        _input: &[InputEvent],
    ) -> Result<(), String> {
        while let Some(message) = world.messages.poll_message::<Self>() {
            match message {
                Message::Init => {
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
                }
                _ => {}
            }
        }

        Ok(())
    }
}
