use super::{audio::AudioState, ui::UiState, GameState};

#[derive(Default)]
pub struct SceneState {
    pub game: GameState,
    pub ui: UiState,
    pub audio: AudioState,
}
