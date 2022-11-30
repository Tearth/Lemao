use crate::renderer::context::RendererContext;
use crate::renderer::fonts::storage::FontStorage;
use crate::renderer::textures::storage::TextureStorage;
use lemao_common_platform::input::InputEvent;
use lemao_common_platform::input::Key;
use lemao_common_platform::input::MouseButton;
use lemao_common_platform::window::WindowPlatformSpecific;
use lemao_common_platform::window::WindowStyle;
use lemao_math::vec2::Vec2;
use lemao_windows_winapi::window::WindowsWinAPIWindow;
use std::sync::Arc;
use std::sync::Mutex;

pub struct WindowContext {
    window: Box<dyn WindowPlatformSpecific>,
}

impl WindowContext {
    pub fn new(title: &str, style: WindowStyle) -> Result<Self, String> {
        Ok(Self { window: WindowsWinAPIWindow::new(title, style)? })
    }

    pub fn poll_event(&mut self) -> Option<InputEvent> {
        self.window.poll_event()
    }

    pub fn create_renderer(&mut self, textures: Arc<Mutex<TextureStorage>>, fonts: Arc<Mutex<FontStorage>>) -> Result<RendererContext, String> {
        let renderer_platform_specific = self.window.create_renderer()?;
        let mut renderer = RendererContext::new(renderer_platform_specific, textures, fonts)?;
        renderer.init()?;

        Ok(renderer)
    }

    pub fn get_position(&self) -> Vec2 {
        self.window.get_position()
    }

    pub fn get_size(&self) -> Vec2 {
        self.window.get_size()
    }

    pub fn get_style(&self) -> WindowStyle {
        self.window.get_style()
    }

    pub fn set_style(&mut self, style: WindowStyle) -> Result<(), String> {
        self.window.set_style(style)
    }

    pub fn swap_buffers(&self) {
        self.window.swap_buffers();
    }

    pub fn close(&self) {
        self.window.close();
    }

    pub fn is_key_pressed(&self, key: Key) -> bool {
        self.window.is_key_pressed(key)
    }

    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.window.is_mouse_button_pressed(button)
    }

    pub fn get_cursor_position(&self) -> (i32, i32) {
        self.window.get_cursor_position()
    }

    pub fn set_cursor_visibility(&self, visible: bool) {
        self.window.set_cursor_visibility(visible)
    }

    pub fn is_cursor_visible(&self) -> bool {
        self.window.is_cursor_visible()
    }
}
