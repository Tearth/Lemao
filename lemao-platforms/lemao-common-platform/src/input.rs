pub enum InputEvent {
    Unknown,
    WindowMoved(i32, i32),
    WindowSizeChanged(u32, u32),
    WindowClosed,
    KeyPressed(Key),
    KeyReleased(Key),
    CharPressed(char),
    MouseButtonPressed(MouseButton),
    MouseButtonReleased(MouseButton),
    MouseMoved(i32, i32),
    MouseWheelRotated(i32),
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

#[derive(Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Unknown,
}
