use lemao_winapi::bindings::winapi::{self, tagCURSORINFO};
use std::{mem, ptr};

use super::context::WindowContext;

pub enum InputEvent {
    Unknown,
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

#[derive(Debug)]
pub enum Key {
    Unknown = 0x00,
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

#[derive(Debug)]
pub enum MouseButton {
    Unknown = 0x00,
    Left = 0x01,
    Right = 0x02,
    Middle = 0x04,
}

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

pub fn is_key_pressed(key: Key) -> bool {
    unsafe { ((winapi::GetKeyState(key as i32) as u16) & 0x8000) != 0 }
}

pub fn is_mouse_button_pressed(button: MouseButton) -> bool {
    unsafe { ((winapi::GetKeyState(button as i32) as u16) & 0x8000) != 0 }
}

pub fn get_cursor_position(window: &WindowContext) -> (i32, i32) {
    unsafe {
        let mut point = mem::zeroed();
        winapi::GetCursorPos(&mut point);
        winapi::ScreenToClient(window.hwnd, &mut point);

        (point.x, point.y)
    }
}

pub fn set_cursor_visibility(visible: bool) {
    unsafe {
        match visible {
            true => while winapi::ShowCursor(1) < 0 {},
            false => while winapi::ShowCursor(0) >= 0 {},
        };
    }
}

pub fn is_cursor_visible() -> bool {
    unsafe {
        let mut cursor_info: winapi::tagCURSORINFO = mem::zeroed();
        cursor_info.cbSize = mem::size_of::<winapi::tagCURSORINFO>() as u32;
        winapi::GetCursorInfo(&mut cursor_info);

        cursor_info.flags != 0
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
