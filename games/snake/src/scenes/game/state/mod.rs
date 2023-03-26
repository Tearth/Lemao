use self::game::GameState;
use self::ui::UiState;

pub mod game;
pub mod ui;

#[derive(Default)]
pub struct SceneState {
    pub game: GameState,
    pub ui: UiState,
}
