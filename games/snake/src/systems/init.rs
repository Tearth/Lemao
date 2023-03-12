use crate::components::cell::CellComponent;
use crate::components::head::{HeadComponent, HeadDirection};
use crate::components::obstacle::ObstacleComponent;
use crate::components::position::PositionComponent;
use crate::components::sprite::SpriteComponent;
use crate::global::GlobalAppData;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::app::Application;
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
        let rows = 20;
        let cols = 40;

        for row in 0..rows {
            for col in 0..cols {
                let cell_id = world.create_entity();
                let mut rectangle = app.renderer.create_rectangle()?;
                rectangle.size = Vec2::new(24.0, 24.0);

                if row == 0 || row == rows - 1 || col == 0 || col == cols - 1 {
                    rectangle.set_texture(app.renderer.textures.get_by_name("border")?);
                    world.create_component(cell_id, ObstacleComponent::new(cell_id))?;
                } else {
                    rectangle.set_texture(app.renderer.textures.get_by_name("cell")?);
                    world.create_component(cell_id, CellComponent::new(cell_id))?;
                }

                rectangle.update();

                world.create_component(cell_id, PositionComponent::new(cell_id, row, col))?;
                world.create_component(cell_id, SpriteComponent::new(cell_id, rectangle))?;
            }
        }

        let head_id = world.create_entity();
        let mut head_rectangle = app.renderer.create_rectangle()?;
        head_rectangle.size = Vec2::new(24.0, 24.0);
        head_rectangle.set_texture(app.renderer.textures.get_by_name("head")?);
        head_rectangle.update();

        world.create_component(head_id, HeadComponent::new(head_id, HeadDirection::Right))?;
        world.create_component(head_id, PositionComponent::new(head_id, 10, 10))?;
        world.create_component(head_id, SpriteComponent::new(head_id, head_rectangle))?;

        Ok(())
    }
}
