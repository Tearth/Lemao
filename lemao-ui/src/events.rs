use lemao_core::lemao_common_platform::input::{MouseButton, MouseWheelDirection};
use lemao_math::vec2::Vec2;

pub enum UiEvent {
    CursorEnter(usize, Vec2),
    CursorLeave(usize, Vec2),
    MouseButtonPressed(usize, MouseButton),
    MouseButtonReleased(usize, MouseButton),
    ButtonClicked(usize, MouseButton),
    CheckboxChecked(usize, MouseButton),
    CheckboxUnchecked(usize, MouseButton),
    CheckboxChanged(usize, MouseButton, bool),
    ScrollboxScroll(usize, MouseWheelDirection),
}
