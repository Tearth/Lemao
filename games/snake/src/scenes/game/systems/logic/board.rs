use super::*;
use crate::scenes::game::components::cell::CellComponent;
use crate::scenes::game::components::obstacle::ObstacleComponent;
use crate::scenes::game::components::position::PositionComponent;
use crate::scenes::game::components::sprite::SpriteComponent;
use crate::scenes::game::messages::Message;
use crate::scenes::game::scene::GameScene;
use crate::scenes::game::scene::GameWorld;
use crate::scenes::game::utils::Coordinates;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::ecs::commands::spawn::SpawnCommand;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::systems::SystemStage;
use std::any::TypeId;

#[derive(Default)]
pub struct BoardSystem {}

impl System<GlobalAppData, GameScene, Message> for BoardSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::GameLogic
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn update(&mut self, app: &mut GameApp, _scene: &mut GameScene, world: &mut GameWorld) -> Result<(), String> {
        while let Some(message) = world.messages.poll_message::<Self>() {
            if let Message::Init = message {
                for row in 0..app.global_data.board_height {
                    for col in 0..app.global_data.board_width {
                        let border = row == 0 || row == app.global_data.board_height - 1 || col == 0 || col == app.global_data.board_width - 1;
                        let texture_name = if border { "border" } else { "cell" };

                        let cell_id = world.entities.create();
                        let mut rectangle = app.renderer.create_tilemap(app.renderer.textures.get_by_name(texture_name)?.id)?;
                        rectangle.size = app.global_data.cell_size;
                        rectangle.anchor = Vec2::new(0.5, 0.5);

                        if row == 0 || row == app.global_data.board_height - 1 || col == 0 || col == app.global_data.board_width - 1 {
                            world.commands.send(SpawnCommand::new(cell_id, ObstacleComponent::new(cell_id)));
                        } else {
                            world.commands.send(SpawnCommand::new(cell_id, CellComponent::new(cell_id)));
                        }

                        rectangle.update();

                        world.commands.send(SpawnCommand::new(cell_id, PositionComponent::new(cell_id, Coordinates::new(row, col))));
                        world.commands.send(SpawnCommand::new(cell_id, SpriteComponent::new(cell_id, rectangle, LAYER_BOARD)));
                    }
                }

                return Ok(());
            }
        }

        Ok(())
    }
}
