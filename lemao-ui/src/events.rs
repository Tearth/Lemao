use lemao_core::lemao_common_platform::input::{MouseButton, MouseWheelDirection};
use lemao_math::vec2::Vec2;

pub enum UiEvent {
    CursorEnter(usize, Vec2),
    CursorLeave(usize, Vec2),
    MouseButtonPressed(usize, MouseButton),
    MouseButtonReleased(usize, MouseButton),

    ButtonPressed(usize, MouseButton),
    ButtonReleased(usize, MouseButton),
    ButtonClicked(usize, MouseButton),

    CheckboxChecked(usize, MouseButton),
    CheckboxUnchecked(usize, MouseButton),
    CheckboxChanged(usize, MouseButton, bool),

    ScrollCursorEnter(usize, Vec2),
    ScrollCursorLeave(usize, Vec2),
    ScrollMouseButtonPressed(usize, MouseButton),
    ScrollMouseButtonReleased(usize, MouseButton),
    ScrollMoved(usize, f32),

    SelectorCursorEnter(usize, Vec2),
    SelectorCursorLeave(usize, Vec2),
    SelectorMouseButtonPressed(usize, MouseButton),
    SelectorMouseButtonReleased(usize, MouseButton),
    SelectorMoved(usize, f32),

    TextBoxActivated(usize, MouseButton),
    TextBoxDeactivated(usize, MouseButton),
    TextBoxContentChanged(usize, char),
}
