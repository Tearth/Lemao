#![allow(clippy::collapsible_match)]

use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::storage::FontStorage;
use lemao_core::renderer::textures::storage::TextureStorage;
use lemao_core::window::context::WindowContext;
use lemao_core::window::context::WindowStyle;
use lemao_core::window::input;
use lemao_core::window::input::InputEvent;
use lemao_core::window::input::Key;
use lemao_core::window::input::MouseButton;
use lemao_math::color::Color;
use lemao_math::vec2::Vec2;
use std::sync::Arc;
use std::sync::Mutex;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Rectangle:
 LMB - place
 Scroll - set size";

pub fn main() -> Result<(), String> {
    let textures = Arc::new(Mutex::new(TextureStorage::default()));
    let fonts = Arc::new(Mutex::new(FontStorage::default()));

    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Rectangle", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer(textures, fonts.clone())?;

    let font_id = fonts.lock().unwrap().store(bff::load(&renderer, "./assets/inconsolata.bff")?);

    let rectangle_id = renderer.create_rectangle(Vec2::new(100.0, 100.0)).unwrap();
    let description_text_id = renderer.create_text(font_id)?;

    let rectangle = renderer.get_drawable_with_type_mut::<Rectangle>(rectangle_id)?;
    rectangle.set_anchor(Vec2::new(0.5, 0.5));
    rectangle.set_position(Vec2::new(400.0, 300.0));

    let description_text = renderer.get_drawable_with_type_mut::<Text>(description_text_id)?;
    description_text.set_text(DESCRIPTION);
    description_text.set_anchor(Vec2::new(0.0, 1.0));
    description_text.set_line_height(20);

    let mut is_running = true;
    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::KeyPressed(k) => {
                    if let Key::Escape = k {
                        window.close()
                    }
                }
                InputEvent::MouseWheelRotated(delta) => {
                    let rectangle = renderer.get_drawable_with_type_mut::<Rectangle>(rectangle_id)?;
                    if delta > 0 {
                        rectangle.set_size(rectangle.get_size() + Vec2::new(1.0, 1.0));
                    } else {
                        rectangle.set_size(rectangle.get_size() - Vec2::new(1.0, 1.0));
                    }
                }
                InputEvent::WindowSizeChanged(width, height) => {
                    renderer.set_viewport(width, height);
                    renderer.get_active_camera_mut()?.set_size(Vec2::new(width as f32, height as f32));
                    renderer.get_drawable_mut(description_text_id)?.set_position(Vec2::new(5.0, height as f32 - 0.0));
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        if input::is_mouse_button_pressed(MouseButton::Left) {
            let window_size = window.get_size();
            let position = input::get_cursor_position(&window);
            renderer.get_drawable_with_type_mut::<Rectangle>(rectangle_id)?.set_position(Vec2::new(position.0 as f32, window_size.y - position.1 as f32));
        }

        renderer.clear(Color::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(rectangle_id)?;
        renderer.draw(description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
