use crate::scenes::game::components::cell::CellComponent;
use crate::scenes::game::components::obstacle::ObstacleComponent;
use crate::scenes::game::components::position::PositionComponent;
use crate::scenes::game::components::sprite::SpriteComponent;
use crate::scenes::game::messages::Message;
use crate::scenes::game::state::global::GlobalAppData;
use crate::scenes::game::utils::Coordinates;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::app::Application;
use lemao_framework::ecs::commands::spawn::SpawnCommand;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;

use super::LAYER_BOARD;

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
                            let border = if row == 0 || row == app.global_data.board_height - 1 || col == 0 || col == app.global_data.board_width - 1 {
                                true
                            } else {
                                false
                            };
                            let texture_name = if border { "border" } else { "cell" };

                            let cell_id = world.entities.create();
                            let mut rectangle = app.renderer.create_tilemap(app.renderer.textures.get_by_name(texture_name)?.id)?;
                            rectangle.size = app.global_data.cell_size;
                            rectangle.anchor = Vec2::new(0.5, 0.5);

                            if row == 0 || row == app.global_data.board_height - 1 || col == 0 || col == app.global_data.board_width - 1 {
                                world.commands.send(Box::new(SpawnCommand::new(cell_id, ObstacleComponent::new(cell_id))));
                            } else {
                                world.commands.send(Box::new(SpawnCommand::new(cell_id, CellComponent::new(cell_id))));
                            }

                            rectangle.update();

                            world.commands.send(Box::new(SpawnCommand::new(cell_id, PositionComponent::new(cell_id, Coordinates::new(row, col)))));
                            world.commands.send(Box::new(SpawnCommand::new(cell_id, SpriteComponent::new(cell_id, rectangle, LAYER_BOARD))));
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
