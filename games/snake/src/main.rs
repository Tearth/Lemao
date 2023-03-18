#![allow(clippy::single_match)]

use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::app::Application;
use scenes::game::GameScene;

pub mod components;
pub mod core;
pub mod messages;
pub mod scenes;
pub mod state;
pub mod systems;

#[no_mangle]
#[cfg(windows)]
#[cfg(not(debug_assertions))]
pub static NvOptimusEnablement: i32 = 1;

#[no_mangle]
#[cfg(windows)]
#[cfg(not(debug_assertions))]
pub static AmdPowerXpressRequestHighPerformance: i32 = 1;

pub fn main() -> Result<(), String> {
    Application::new("Snake", WindowStyle::Window { position: Default::default(), size: Vec2::new(1366.0, 768.0) })?
        .register_scene("Game", |app| Box::new(GameScene::new(app)), true)?
        .run()
}
