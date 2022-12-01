use lemao_common_platform::window::WindowPlatformSpecific;
use lemao_common_platform::window::WindowStyle;
use lemao_math::vec2::Vec2;

pub struct WindowX11 {
    style: WindowStyle,
    position: Vec2,
    size: Vec2,
}

impl WindowX11 {
    pub fn new(title: &str, style: WindowStyle) -> Result<Box<Self>, String> {
        Ok(Box::new(Self { style, position: Default::default(), size: Default::default() }))
    }
}

impl WindowPlatformSpecific for WindowX11 {
    fn poll_event(&mut self) -> Option<lemao_common_platform::input::InputEvent> {
        todo!()
    }

    fn create_renderer(&mut self) -> Result<Box<dyn lemao_common_platform::renderer::RendererPlatformSpecific>, String> {
        todo!()
    }

    fn get_position(&self) -> Vec2 {
        todo!()
    }

    fn get_size(&self) -> Vec2 {
        todo!()
    }

    fn get_style(&self) -> WindowStyle {
        todo!()
    }

    fn set_style(&mut self, style: WindowStyle) -> Result<(), String> {
        todo!()
    }

    fn swap_buffers(&self) {
        todo!()
    }

    fn close(&self) {
        todo!()
    }

    fn is_key_pressed(&self, key: lemao_common_platform::input::Key) -> bool {
        todo!()
    }

    fn is_mouse_button_pressed(&self, button: lemao_common_platform::input::MouseButton) -> bool {
        todo!()
    }

    fn get_cursor_position(&self) -> (i32, i32) {
        todo!()
    }

    fn set_cursor_visibility(&self, visible: bool) {
        todo!()
    }

    fn is_cursor_visible(&self) -> bool {
        todo!()
    }
}
