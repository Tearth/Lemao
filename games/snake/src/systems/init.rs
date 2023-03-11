use crate::components::sprite::Sprite;
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
    let tile_id = world.create_entity();
    let mut rectangle = app.renderer.create_rectangle()?;
    rectangle.set_texture(app.renderer.textures.get_by_name("cell")?);
    rectangle.position = Vec2::new(200.0, 200.0);
    rectangle.size = Vec2::new(32.0, 32.0);
    rectangle.update();

    world.create_component(tile_id, Sprite::new(rectangle))?;
    Ok(())
}
