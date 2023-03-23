use super::{ui::UiState, GameState};

#[derive(Default)]
pub struct SceneState {
    pub game: GameState,
    pub ui: UiState,
}
