use super::{game::GameState, ui::UiState};

#[derive(Default)]
pub struct SceneState {
    pub game: GameState,
    pub ui: UiState,
}
