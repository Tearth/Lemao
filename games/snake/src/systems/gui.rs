use crate::global::GlobalAppData;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_framework::app::Application;
use lemao_framework::ecs::world::World;

pub fn update(
    app: &mut Application<GlobalAppData>,
    scene: &mut GameScene,
    _world: &mut World<GlobalAppData, GameScene>,
    input: &[InputEvent],
) -> Result<(), String> {
    for event in input {
        scene.ui.process_window_event(&mut app.renderer, event)?;
    }

    scene.ui.update(&mut app.renderer)?;

    Ok(())
}
