use crate::components::cell::CellComponent;
use crate::components::obstacle::ObstacleComponent;
use crate::components::position::PositionComponent;
use crate::components::sprite::SpriteComponent;
use crate::global::GlobalAppData;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::app::Application;
use lemao_framework::ecs::world::World;

pub fn update(
    app: &mut Application<GlobalAppData>,
    _scene: &mut GameScene,
    world: &mut World<GlobalAppData, GameScene>,
    _input: &[InputEvent],
) -> Result<(), String> {
    let rows = 20;
    let cols = 40;

    for row in 0..rows {
        for col in 0..cols {
            let cell_id = world.create_entity();
            let mut rectangle = app.renderer.create_rectangle()?;
            rectangle.position = Vec2::new(col as f32 * 24.0, row as f32 * 24.0);
            rectangle.size = Vec2::new(24.0, 24.0);

            if row == 0 || row == rows - 1 || col == 0 || col == cols - 1 {
                rectangle.set_texture(app.renderer.textures.get_by_name("border")?);
                world.create_component(cell_id, ObstacleComponent::default())?;
            } else {
                rectangle.set_texture(app.renderer.textures.get_by_name("cell")?);
                world.create_component(cell_id, CellComponent::default())?;
            }

            rectangle.update();

            world.create_component(cell_id, PositionComponent::new(row, col))?;
            world.create_component(cell_id, SpriteComponent::new(rectangle))?;
        }
    }

    Ok(())
}
