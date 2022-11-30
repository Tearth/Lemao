use crate::bindings::winapi;
use lemao_common_platform::input::InputEvent;
use lemao_common_platform::input::Key;
use lemao_common_platform::input::MouseButton;

impl From<winapi::MSG> for InputEvent {
    fn from(message: winapi::MSG) -> InputEvent {
        match message.message {
            winapi::WM_KEYDOWN => InputEvent::KeyPressed(virtual_key_to_key(message.wParam)),
            winapi::WM_KEYUP => InputEvent::KeyReleased(virtual_key_to_key(message.wParam)),
            winapi::WM_CHAR => InputEvent::CharPressed(char::from_u32(message.wParam as u32).unwrap()),
            winapi::WM_LBUTTONDOWN => InputEvent::MouseButtonPressed(MouseButton::Left),
            winapi::WM_RBUTTONDOWN => InputEvent::MouseButtonPressed(MouseButton::Right),
            winapi::WM_MBUTTONDOWN => InputEvent::MouseButtonPressed(MouseButton::Middle),
            winapi::WM_LBUTTONUP => InputEvent::MouseButtonReleased(MouseButton::Left),
            winapi::WM_RBUTTONUP => InputEvent::MouseButtonReleased(MouseButton::Right),
            winapi::WM_MBUTTONUP => InputEvent::MouseButtonReleased(MouseButton::Middle),
            winapi::WM_MOUSEMOVE => InputEvent::MouseMoved((message.lParam as i32) & 0xffff, (message.lParam as i32) >> 16),
            winapi::WM_MOUSEWHEEL => InputEvent::MouseWheelRotated((message.wParam as i32) >> 16),
            _ => InputEvent::Unknown,
        }
    }
}
fn virtual_key_to_key(virtual_key: u64) -> Key {
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
