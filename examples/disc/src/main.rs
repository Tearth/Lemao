#![allow(clippy::collapsible_match, clippy::collapsible_else_if)]

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_common_platform::input::MouseWheelDirection;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::disc::Disc;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::window::context::CoordinationSystem;
use lemao_core::window::context::WindowContext;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Disc:
 LMB - place
 Scroll - set sides";

pub fn main() -> Result<(), String> {
    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Disc", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer()?;
    renderer.set_swap_interval(1);

    let font_storage = renderer.get_fonts();
    let mut font_storage = font_storage.write().unwrap();
    let font_id = font_storage.store(Box::new(Font::new(&renderer, &bff::load("./assets/inconsolata.bff")?)?));

    drop(font_storage);

    let disc_id = renderer.create_disc(100.0, 32).unwrap();
    let description_text_id = renderer.create_text(font_id)?;

    let disc = renderer.get_drawable_and_cast_mut::<Disc>(disc_id)?;
    disc.set_anchor(Vec2::new(0.5, 0.5));
    disc.set_position(Vec2::new(400.0, 300.0));

    let description_text = renderer.get_drawable_and_cast_mut::<Text>(description_text_id)?;
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
                InputEvent::MouseWheelRotated(direction, _) => {
                    let disc = renderer.get_drawable_and_cast_mut::<Disc>(disc_id)?;
                    if direction == MouseWheelDirection::Up {
                        disc.set_sides(disc.get_sides() + 1);
                    } else {
                        if disc.get_sides() > 3 {
                            disc.set_sides(disc.get_sides() - 1);
                        }
                    }
                }
                InputEvent::WindowSizeChanged(size) => {
                    renderer.set_viewport_size(size);
                    renderer.get_active_camera_mut()?.set_size(size);
                    renderer.get_drawable_mut(description_text_id)?.set_position(Vec2::new(5.0, size.y - 0.0));
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        if window.is_mouse_button_pressed(MouseButton::Left) {
            let position = window.get_cursor_position(CoordinationSystem::Window);
            renderer.get_drawable_and_cast_mut::<Disc>(disc_id)?.set_position(position);
        }

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(disc_id)?;
        renderer.draw(description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
