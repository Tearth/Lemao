use lemao_math::vec2::Vec2;

pub enum InputEvent {
    Unknown,
    WindowMoved(Vec2),
    WindowSizeChanged(Vec2),
    WindowClosed,
    KeyPressed(Key),
    KeyReleased(Key),
    CharPressed(char),
    MouseButtonPressed(MouseButton, Vec2),
    MouseButtonReleased(MouseButton, Vec2),
    MouseMoved(Vec2, Vec2),
    MouseWheelRotated(MouseWheelDirection, Vec2),
}

#[derive(Debug, Copy, Clone)]
pub enum Key {
    Unknown,
    Enter,
    Escape,
    Space,

    ArrowLeft,
    ArrowUp,
    ArrowRight,
    ArrowDown,

    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,

    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,

    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MouseWheelDirection {
    Up,
    Down,
    None,
}
