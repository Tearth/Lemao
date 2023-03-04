use crate::renderer::cameras::Camera;
use crate::renderer::context::RendererContext;
use lemao_common_platform::input::InputEvent;
use lemao_common_platform::input::Key;
use lemao_common_platform::input::MouseButton;
use lemao_common_platform::window::WindowPlatformSpecific;
use lemao_common_platform::window::WindowStyle;
use lemao_math::vec2::Vec2;
use std::collections::VecDeque;

pub struct WindowContext {
    window: Box<dyn WindowPlatformSpecific>,
    events: VecDeque<InputEvent>,
}

pub enum CoordinationSystem<'a> {
    Window,
    Camera(&'a Camera),
}

impl WindowContext {
    pub fn new(title: &str, style: WindowStyle) -> Result<Self, String> {
        #[cfg(windows)]
        return Ok(Self { window: lemao_windows_winapi::window::WindowWinAPI::new(title, style)?, events: VecDeque::new() });

        #[cfg(unix)]
        return Ok(Self { window: lemao_linux_x11::window::WindowX11::new(title, style)?, events: VecDeque::new() });
    }

    pub fn poll_event(&mut self) -> Option<InputEvent> {
        self.events.extend(self.window.poll_event());
        self.events.pop_front()
    }

    pub fn create_renderer(&mut self) -> Result<RendererContext, String> {
        let renderer_platform_specific = self.window.create_renderer()?;
        let mut renderer = RendererContext::new(renderer_platform_specific, self.window.get_size())?;
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

    pub fn get_cursor_position(&self, coordination_system: CoordinationSystem) -> Vec2 {
        let cursor_position = self.window.get_cursor_position();

        match coordination_system {
            CoordinationSystem::Window => Vec2::new(cursor_position.x, cursor_position.y),
            CoordinationSystem::Camera(camera) => Vec2::new(cursor_position.x, cursor_position.y) + camera.position,
        }
    }

    pub fn set_cursor_visibility(&mut self, visible: bool) {
        self.window.set_cursor_visibility(visible)
    }

    pub fn is_cursor_visible(&self) -> bool {
        self.window.is_cursor_visible()
    }
}
