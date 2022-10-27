use lemao_winapi::bindings::winapi;

pub enum InputEvent {
    Invalid,
    WindowClosed,
    KeyPressed(Key),
    KeyReleased(Key),
    CharPressed(char),
}

#[derive(Debug)]
pub enum Key {
    Enter = 0x0d,
    Escape = 0x1b,
    Space = 0x20,

    ArrowLeft = 0x25,
    ArrowUp = 0x26,
    ArrowRight = 0x27,
    ArrowDown = 0x28,

    Key0 = 0x30,
    Key1 = 0x31,
    Key2 = 0x32,
    Key3 = 0x33,
    Key4 = 0x34,
    Key5 = 0x35,
    Key6 = 0x36,
    Key7 = 0x37,
    Key8 = 0x38,
    Key9 = 0x39,

    KeyA = 0x41,
    KeyB = 0x42,
    KeyC = 0x43,
    KeyD = 0x44,
    KeyE = 0x45,
    KeyF = 0x46,
    KeyG = 0x47,
    KeyH = 0x48,
    KeyI = 0x49,
    KeyJ = 0x4a,
    KeyK = 0x4b,
    KeyL = 0x4c,
    KeyM = 0x4d,
    KeyN = 0x4e,
    KeyO = 0x4f,
    KeyP = 0x50,
    KeyQ = 0x51,
    KeyR = 0x52,
    KeyS = 0x53,
    KeyT = 0x54,
    KeyU = 0x55,
    KeyV = 0x56,
    KeyW = 0x57,
    KeyX = 0x58,
    KeyY = 0x59,
    KeyZ = 0x5a,

    Num0 = 0x60,
    Num1 = 0x61,
    Num2 = 0x62,
    Num3 = 0x63,
    Num4 = 0x64,
    Num5 = 0x65,
    Num6 = 0x66,
    Num7 = 0x67,
    Num8 = 0x68,
    Num9 = 0x69,
}

impl From<winapi::MSG> for InputEvent {
    fn from(message: winapi::MSG) -> InputEvent {
        match message.message {
            winapi::WM_KEYDOWN => match message.wParam {
                0x0d => InputEvent::KeyPressed(Key::Enter),
                0x1b => InputEvent::KeyPressed(Key::Escape),
                0x20 => InputEvent::KeyPressed(Key::Space),

                0x25 => InputEvent::KeyPressed(Key::ArrowLeft),
                0x26 => InputEvent::KeyPressed(Key::ArrowUp),
                0x27 => InputEvent::KeyPressed(Key::ArrowRight),
                0x28 => InputEvent::KeyPressed(Key::ArrowDown),

                0x30 => InputEvent::KeyPressed(Key::Key0),
                0x31 => InputEvent::KeyPressed(Key::Key1),
                0x32 => InputEvent::KeyPressed(Key::Key2),
                0x33 => InputEvent::KeyPressed(Key::Key3),
                0x34 => InputEvent::KeyPressed(Key::Key4),
                0x35 => InputEvent::KeyPressed(Key::Key5),
                0x36 => InputEvent::KeyPressed(Key::Key6),
                0x37 => InputEvent::KeyPressed(Key::Key7),
                0x38 => InputEvent::KeyPressed(Key::Key8),
                0x39 => InputEvent::KeyPressed(Key::Key9),

                0x41 => InputEvent::KeyPressed(Key::KeyA),
                0x42 => InputEvent::KeyPressed(Key::KeyB),
                0x43 => InputEvent::KeyPressed(Key::KeyC),
                0x44 => InputEvent::KeyPressed(Key::KeyD),
                0x45 => InputEvent::KeyPressed(Key::KeyE),
                0x46 => InputEvent::KeyPressed(Key::KeyF),
                0x47 => InputEvent::KeyPressed(Key::KeyG),
                0x48 => InputEvent::KeyPressed(Key::KeyH),
                0x49 => InputEvent::KeyPressed(Key::KeyI),
                0x4a => InputEvent::KeyPressed(Key::KeyJ),
                0x4b => InputEvent::KeyPressed(Key::KeyK),
                0x4c => InputEvent::KeyPressed(Key::KeyL),
                0x4d => InputEvent::KeyPressed(Key::KeyM),
                0x4e => InputEvent::KeyPressed(Key::KeyN),
                0x4f => InputEvent::KeyPressed(Key::KeyO),
                0x50 => InputEvent::KeyPressed(Key::KeyP),
                0x51 => InputEvent::KeyPressed(Key::KeyQ),
                0x52 => InputEvent::KeyPressed(Key::KeyR),
                0x53 => InputEvent::KeyPressed(Key::KeyS),
                0x54 => InputEvent::KeyPressed(Key::KeyT),
                0x55 => InputEvent::KeyPressed(Key::KeyU),
                0x56 => InputEvent::KeyPressed(Key::KeyV),
                0x57 => InputEvent::KeyPressed(Key::KeyW),
                0x58 => InputEvent::KeyPressed(Key::KeyX),
                0x59 => InputEvent::KeyPressed(Key::KeyY),
                0x5a => InputEvent::KeyPressed(Key::KeyZ),

                0x60 => InputEvent::KeyPressed(Key::Num0),
                0x61 => InputEvent::KeyPressed(Key::Num1),
                0x62 => InputEvent::KeyPressed(Key::Num2),
                0x63 => InputEvent::KeyPressed(Key::Num3),
                0x64 => InputEvent::KeyPressed(Key::Num4),
                0x65 => InputEvent::KeyPressed(Key::Num5),
                0x66 => InputEvent::KeyPressed(Key::Num6),
                0x67 => InputEvent::KeyPressed(Key::Num7),
                0x68 => InputEvent::KeyPressed(Key::Num8),
                0x69 => InputEvent::KeyPressed(Key::Num9),

                _ => InputEvent::Invalid,
            },
            winapi::WM_CHAR => InputEvent::CharPressed(char::from_u32(message.wParam as u32).unwrap()),
            _ => InputEvent::Invalid,
        }
    }
}

pub fn is_key_pressed(key: Key) -> bool {
    unsafe { ((winapi::GetKeyState(key as i32) as u16) & 0x8000) != 0 }
}
