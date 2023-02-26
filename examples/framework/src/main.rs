use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::app::Application;
use scenes::first::FirstScene;
use scenes::second::SecondScene;

pub mod global;
pub mod scenes;

pub fn main() -> Result<(), String> {
    Application::new("Framework", WindowStyle::Window { position: Default::default(), size: Vec2::new(1366.0, 768.0) })?
        .register_scene("Scene 1", Box::new(FirstScene::new()), true)?
        .register_scene("Scene 2", Box::new(SecondScene::new()), false)?
        .run()
}
