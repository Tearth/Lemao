use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::app::Application;
use scenes::game::scene::GameScene;
use scenes::menu::scene::MenuScene;
use state::global::GlobalAppData;

pub mod scenes;
pub mod state;

type GameApp = Application<GlobalAppData>;

#[no_mangle]
#[cfg(windows)]
#[cfg(not(debug_assertions))]
pub static NvOptimusEnablement: i32 = 1;

#[no_mangle]
#[cfg(windows)]
#[cfg(not(debug_assertions))]
pub static AmdPowerXpressRequestHighPerformance: i32 = 1;

pub fn main() -> Result<(), String> {
    Application::new("Theo The Snake", WindowStyle::Window { position: Default::default(), size: Vec2::new(1366.0, 768.0) })?
        .register_scene("Menu", |app| Box::new(MenuScene::new(app)), true)?
        .register_scene("Game", |app| Box::new(GameScene::new(app)), false)?
        .run()
}
