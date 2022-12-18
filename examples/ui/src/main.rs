use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::Color;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::fonts::storage::FontStorage;
use lemao_core::renderer::textures::storage::TextureStorage;
use lemao_core::window::context::WindowContext;
use lemao_ui::context::UiContext;
use std::sync::Arc;
use std::sync::Mutex;

pub fn main() -> Result<(), String> {
    let textures = Arc::new(Mutex::new(TextureStorage::default()));
    let fonts = Arc::new(Mutex::new(FontStorage::default()));

    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = match WindowContext::new("UI", WindowStyle::Window { position: window_position, size: window_size }) {
        Ok(window) => window,
        Err(message) => panic!("{}", message),
    };

    let mut renderer = match window.create_renderer(textures, fonts) {
        Ok(renderer) => renderer,
        Err(message) => panic!("{}", message),
    };

    let mut ui = UiContext::new()?;
    let panel_id = ui.create_panel(&mut renderer)?;
    let mut is_running = true;

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowSizeChanged(size) => {
                    renderer.set_viewport(size.x as u32, size.y as u32);
                    renderer.get_active_camera_mut()?.set_size(size);
                }

                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }

            ui.process_event(&event);
        }

        renderer.clear(Color::new(0.5, 0.5, 0.5, 1.0));
        ui.draw(&mut renderer, panel_id)?;
        window.swap_buffers();
    }

    Ok(())
}
