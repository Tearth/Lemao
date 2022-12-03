use lemao_common_platform::input::Key;
use lemao_common_platform::input::MouseButton;

pub fn key_to_virtual_key(key: Key) -> u64 {
    match key {
        Key::Enter => 0x0d,
        Key::Escape => 0x1b,
        Key::Space => 0x20,

        Key::ArrowLeft => 0x25,
        Key::ArrowUp => 0x26,
        Key::ArrowRight => 0x27,
        Key::ArrowDown => 0x28,

        Key::Key0 => 0x30,
        Key::Key1 => 0x31,
        Key::Key2 => 0x32,
        Key::Key3 => 0x33,
        Key::Key4 => 0x34,
        Key::Key5 => 0x35,
        Key::Key6 => 0x36,
        Key::Key7 => 0x37,
        Key::Key8 => 0x38,
        Key::Key9 => 0x39,

        Key::KeyA => 0x41,
        Key::KeyB => 0x42,
        Key::KeyC => 0x43,
        Key::KeyD => 0x44,
        Key::KeyE => 0x45,
        Key::KeyF => 0x46,
        Key::KeyG => 0x47,
        Key::KeyH => 0x48,
        Key::KeyI => 0x49,
        Key::KeyJ => 0x4a,
        Key::KeyK => 0x4b,
        Key::KeyL => 0x4c,
        Key::KeyM => 0x4d,
        Key::KeyN => 0x4e,
        Key::KeyO => 0x4f,
        Key::KeyP => 0x50,
        Key::KeyQ => 0x51,
        Key::KeyR => 0x52,
        Key::KeyS => 0x53,
        Key::KeyT => 0x54,
        Key::KeyU => 0x55,
        Key::KeyV => 0x56,
        Key::KeyW => 0x57,
        Key::KeyX => 0x58,
        Key::KeyY => 0x59,
        Key::KeyZ => 0x5a,

        Key::Num0 => 0x60,
        Key::Num1 => 0x61,
        Key::Num2 => 0x62,
        Key::Num3 => 0x63,
        Key::Num4 => 0x64,
        Key::Num5 => 0x65,
        Key::Num6 => 0x66,
        Key::Num7 => 0x67,
        Key::Num8 => 0x68,
        Key::Num9 => 0x69,

        Key::Unknown => 0x00,
    }
}

pub fn virtual_key_to_key(virtual_key: u64) -> Key {
    match virtual_key {
        0x0d => Key::Enter,
        0x1b => Key::Escape,
        0x20 => Key::Space,

        0x25 => Key::ArrowLeft,
        0x26 => Key::ArrowUp,
        0x27 => Key::ArrowRight,
        0x28 => Key::ArrowDown,

        0x30 => Key::Key0,
        0x31 => Key::Key1,
        0x32 => Key::Key2,
        0x33 => Key::Key3,
        0x34 => Key::Key4,
        0x35 => Key::Key5,
        0x36 => Key::Key6,
        0x37 => Key::Key7,
        0x38 => Key::Key8,
        0x39 => Key::Key9,

        0x41 => Key::KeyA,
        0x42 => Key::KeyB,
        0x43 => Key::KeyC,
        0x44 => Key::KeyD,
        0x45 => Key::KeyE,
        0x46 => Key::KeyF,
        0x47 => Key::KeyG,
        0x48 => Key::KeyH,
        0x49 => Key::KeyI,
        0x4a => Key::KeyJ,
        0x4b => Key::KeyK,
        0x4c => Key::KeyL,
        0x4d => Key::KeyM,
        0x4e => Key::KeyN,
        0x4f => Key::KeyO,
        0x50 => Key::KeyP,
        0x51 => Key::KeyQ,
        0x52 => Key::KeyR,
        0x53 => Key::KeyS,
        0x54 => Key::KeyT,
        0x55 => Key::KeyU,
        0x56 => Key::KeyV,
        0x57 => Key::KeyW,
        0x58 => Key::KeyX,
        0x59 => Key::KeyY,
        0x5a => Key::KeyZ,

        0x60 => Key::Num0,
        0x61 => Key::Num1,
        0x62 => Key::Num2,
        0x63 => Key::Num3,
        0x64 => Key::Num4,
        0x65 => Key::Num5,
        0x66 => Key::Num6,
        0x67 => Key::Num7,
        0x68 => Key::Num8,
        0x69 => Key::Num9,

        _ => Key::Unknown,
    }
}

pub fn button_to_virtual_button(button: MouseButton) -> u64 {
    match button {
        MouseButton::Unknown => 0x00,
        MouseButton::Left => 0x01,
        MouseButton::Right => 0x02,
        MouseButton::Middle => 0x04,
    }
}
