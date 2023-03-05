use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::app::Application;
use scenes::game::GameScene;

pub mod global;
pub mod scenes;

pub fn main() -> Result<(), String> {
    Application::new("Snake", WindowStyle::Window { position: Default::default(), size: Vec2::new(1366.0, 768.0) })?
        .register_scene("Game", |app| Box::new(GameScene::new(app)), true)?
        .run()
}
