use lemao_math::vec2::Vec2;

use crate::input::InputEvent;
use crate::input::Key;
use crate::input::MouseButton;
use crate::renderer::RendererPlatformSpecific;

#[derive(Copy, Clone, Debug)]
pub enum WindowStyle {
    Window { position: Vec2, size: Vec2 },
    Borderless,
    Fullscreen,
}

pub trait WindowPlatformSpecific {
    fn poll_event(&mut self) -> Option<InputEvent>;
    fn create_renderer(&mut self) -> Result<Box<dyn RendererPlatformSpecific>, String>;
    fn get_position(&self) -> Vec2;
    fn get_size(&self) -> Vec2;
    fn get_style(&self) -> WindowStyle;
    fn set_style(&mut self, style: WindowStyle) -> Result<(), String>;
    fn swap_buffers(&self);
    fn close(&self);

    fn is_key_pressed(&self, key: Key) -> bool;
    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool;
    fn get_cursor_position(&self) -> (i32, i32);
    fn set_cursor_visibility(&mut self, visible: bool);
    fn is_cursor_visible(&self) -> bool;
}
