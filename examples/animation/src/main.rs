#![allow(clippy::collapsible_match, clippy::collapsible_else_if, clippy::implicit_saturating_sub)]

use std::thread;
use std::time::Duration;

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::input::MouseWheelDirection;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::DrawableEnum;
use lemao_core::window::context::WindowContext;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Animation:
 Scroll - set speed";

pub fn main() -> Result<(), String> {
    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Animation", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer()?;
    renderer.set_swap_interval(1);

    let explosion_id = renderer.create_texture("./assets/explosion.bmp")?;
    let font_id = renderer.create_font("./assets/inconsolata.bff")?;

    let animation_id = renderer.create_tilemap(explosion_id).unwrap();
    let animation = renderer.tilemaps.get_mut(animation_id)?;
    animation.size = Vec2::new(128.0, 128.0);
    animation.anchor = Vec2::new(0.5, 0.5);
    animation.position = window_size / 2.0;
    animation.update();

    let description_text_id = renderer.create_text(font_id)?;
    let description_text = renderer.texts.get_mut(description_text_id)?;
    description_text.text = DESCRIPTION.to_string();
    description_text.anchor = Vec2::new(0.0, 1.0);
    description_text.line_height = 20;
    description_text.update();

    let mut sleep_duration = 10;
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
                    if direction == MouseWheelDirection::Up {
                        if sleep_duration < 1000 {
                            sleep_duration += 1;
                        }
                    } else {
                        if sleep_duration > 0 {
                            sleep_duration -= 1;
                        }
                    }
                }
                InputEvent::WindowSizeChanged(size) => {
                    renderer.texts.get_mut(description_text_id)?.position = Vec2::new(5.0, size.y - 0.0);
                    renderer.set_viewport_size(size)?;
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        thread::sleep(Duration::from_millis(sleep_duration));
        renderer.tilemaps.get_mut(animation_id)?.set_next_frame();
        renderer.tilemaps.get_mut(animation_id)?.update();

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(DrawableEnum::Tilemap, animation_id)?;
        renderer.draw(DrawableEnum::Text, description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
