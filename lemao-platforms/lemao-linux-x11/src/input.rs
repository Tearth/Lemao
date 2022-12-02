use crate::bindings::x11;
use lemao_common_platform::input::InputEvent;
use lemao_common_platform::input::Key;

impl From<x11::_XEvent> for InputEvent {
    fn from(event: x11::_XEvent) -> InputEvent {
        unsafe {
            match event.type_ as u32 {
                x11::KeyPress => InputEvent::KeyPressed(virtual_key_to_key(event.xkey.keycode)),
                //winapi::WM_KEYUP => InputEvent::KeyReleased(virtual_key_to_key(message.wParam)),
                //winapi::WM_CHAR => InputEvent::CharPressed(char::from_u32(message.wParam as u32).unwrap()),
                //winapi::WM_LBUTTONDOWN => InputEvent::MouseButtonPressed(MouseButton::Left),
                //winapi::WM_RBUTTONDOWN => InputEvent::MouseButtonPressed(MouseButton::Right),
                //winapi::WM_MBUTTONDOWN => InputEvent::MouseButtonPressed(MouseButton::Middle),
                //winapi::WM_LBUTTONUP => InputEvent::MouseButtonReleased(MouseButton::Left),
                //winapi::WM_RBUTTONUP => InputEvent::MouseButtonReleased(MouseButton::Right),
                //winapi::WM_MBUTTONUP => InputEvent::MouseButtonReleased(MouseButton::Middle),
                //winapi::WM_MOUSEMOVE => InputEvent::MouseMoved((message.lParam as i32) & 0xffff, (message.lParam as i32) >> 16),
                //winapi::WM_MOUSEWHEEL => InputEvent::MouseWheelRotated((message.wParam as i32) >> 16),
                _ => InputEvent::Unknown,
            }
        }
    }
}

pub fn virtual_key_to_key(virtual_key: u32) -> Key {
    match virtual_key {
        x11::XK_ISO_Enter => Key::Enter,
        x11::XK_Escape => Key::Escape,
        x11::XK_space => Key::Space,

        x11::XK_Left => Key::ArrowLeft,
        x11::XK_Up => Key::ArrowUp,
        x11::XK_Right => Key::ArrowRight,
        x11::XK_Down => Key::ArrowDown,

        x11::XK_0 => Key::Key0,
        x11::XK_1 => Key::Key1,
        x11::XK_2 => Key::Key2,
        x11::XK_3 => Key::Key3,
        x11::XK_4 => Key::Key4,
        x11::XK_5 => Key::Key5,
        x11::XK_6 => Key::Key6,
        x11::XK_7 => Key::Key7,
        x11::XK_8 => Key::Key8,
        x11::XK_9 => Key::Key9,

        x11::XK_A | x11::XK_a => Key::KeyA,
        x11::XK_B | x11::XK_b => Key::KeyB,
        x11::XK_C | x11::XK_c => Key::KeyC,
        x11::XK_D | x11::XK_d => Key::KeyD,
        x11::XK_E | x11::XK_e => Key::KeyE,
        x11::XK_F | x11::XK_f => Key::KeyF,
        x11::XK_G | x11::XK_g => Key::KeyG,
        x11::XK_H | x11::XK_h => Key::KeyH,
        x11::XK_I | x11::XK_i => Key::KeyI,
        x11::XK_J | x11::XK_j => Key::KeyJ,
        x11::XK_K | x11::XK_k => Key::KeyK,
        x11::XK_L | x11::XK_l => Key::KeyL,
        x11::XK_M | x11::XK_m => Key::KeyM,
        x11::XK_N | x11::XK_n => Key::KeyN,
        x11::XK_O | x11::XK_o => Key::KeyO,
        x11::XK_P | x11::XK_p => Key::KeyP,
        x11::XK_Q | x11::XK_q => Key::KeyQ,
        x11::XK_R | x11::XK_r => Key::KeyR,
        x11::XK_S | x11::XK_s => Key::KeyS,
        x11::XK_T | x11::XK_t => Key::KeyT,
        x11::XK_U | x11::XK_u => Key::KeyU,
        x11::XK_V | x11::XK_v => Key::KeyV,
        x11::XK_W | x11::XK_w => Key::KeyW,
        x11::XK_X | x11::XK_x => Key::KeyX,
        x11::XK_Y | x11::XK_y => Key::KeyY,
        x11::XK_Z | x11::XK_z => Key::KeyZ,

        x11::XK_KP_0 => Key::Num0,
        x11::XK_KP_1 => Key::Num1,
        x11::XK_KP_2 => Key::Num2,
        x11::XK_KP_3 => Key::Num3,
        x11::XK_KP_4 => Key::Num4,
        x11::XK_KP_5 => Key::Num5,
        x11::XK_KP_6 => Key::Num6,
        x11::XK_KP_7 => Key::Num7,
        x11::XK_KP_8 => Key::Num8,
        x11::XK_KP_9 => Key::Num9,

        _ => Key::Unknown,
    }
}
