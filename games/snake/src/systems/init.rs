use crate::components::cell::CellComponent;
use crate::components::head::{HeadComponent, HeadDirection};
use crate::components::obstacle::ObstacleComponent;
use crate::components::position::PositionComponent;
use crate::components::sprite::SpriteComponent;
use crate::global::GlobalAppData;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_framework::app::Application;
use lemao_framework::ecs::commands::spawn::SpawnCommand;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;

#[derive(Default)]
pub struct InitSystem {}

impl System<GlobalAppData, GameScene, Message> for InitSystem {
    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        _scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
        _input: &[InputEvent],
    ) -> Result<(), String> {
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
}
