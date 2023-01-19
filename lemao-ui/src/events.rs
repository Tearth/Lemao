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

    ScrollCursorEnter(usize, Vec2),
    ScrollCursorLeave(usize, Vec2),
    ScrollMouseButtonPressed(usize, MouseButton),
    ScrollMouseButtonReleased(usize, MouseButton),
    ScrollMoved(usize, MouseWheelDirection),

    TextBoxActivated(usize, MouseButton),
    TextBoxDeactivated(usize, MouseButton),
    TextBoxContentChanged(usize, char),
}
