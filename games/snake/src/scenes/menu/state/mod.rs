use self::ui::UiState;

pub mod ui;

#[derive(Default)]
pub struct MenuState {
    pub ui: UiState,
}
