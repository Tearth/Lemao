use crate::components::sprite::Sprite;
use crate::global::GlobalAppData;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_framework::app::Application;
use lemao_framework::ecs::components::ComponentManager;
use lemao_framework::ecs::world::World;
use std::any::TypeId;

pub fn update(
    app: &mut Application<GlobalAppData>,
    _scene: &mut GameScene,
    world: &mut World<GlobalAppData, GameScene>,
    _input: &[InputEvent],
) -> Result<(), String> {
    let sprites = world.components.get_mut(&TypeId::of::<Sprite>()).unwrap().clone();
    let mut sprites = sprites.write().unwrap();
    let sprites = sprites.as_any_mut().downcast_mut::<ComponentManager<Sprite>>().unwrap();

    for sprite in sprites.iter_mut() {
        app.renderer.draw(&mut sprite.rectangle)?;
    }

    Ok(())
}
