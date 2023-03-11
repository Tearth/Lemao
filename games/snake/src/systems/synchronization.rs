use crate::global::GlobalAppData;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_framework::app::Application;
use lemao_framework::ecs::world::World;
use std::time::SystemTime;

pub fn update(
    _app: &mut Application<GlobalAppData>,
    scene: &mut GameScene,
    _world: &mut World<GlobalAppData, GameScene>,
    _input: &[InputEvent],
) -> Result<(), String> {
    if scene.time_of_last_tick.elapsed().unwrap().as_millis() >= scene.tick_length as u128 {
        println!("test");
        scene.time_of_last_tick = SystemTime::now();
    }

    Ok(())
}
