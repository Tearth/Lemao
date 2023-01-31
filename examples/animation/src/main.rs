#![allow(clippy::collapsible_match, clippy::collapsible_else_if, clippy::implicit_saturating_sub)]

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::input::MouseWheelDirection;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::animation::Animation;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::bmp;
use lemao_core::renderer::textures::Texture;
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

    let texture_storage = renderer.get_textures();
    let mut texture_storage = texture_storage.write().unwrap();
    let explosion_id = texture_storage.store(Texture::new(&renderer, &bmp::load("./assets/explosion.bmp")?));

    drop(texture_storage);

    let font_storage = renderer.get_fonts();
    let mut font_storage = font_storage.write().unwrap();
    let font_id = font_storage.store(Font::new(&renderer, &bff::load("./assets/inconsolata.bff")?));
    drop(font_storage);

    let animation_id = renderer.create_animation(explosion_id, Vec2::new(128.0, 128.0)).unwrap();
    let description_text_id = renderer.create_text(font_id)?;

    let animation = renderer.get_drawable_with_type_mut::<Animation>(animation_id)?;
    animation.set_anchor(Vec2::new(0.5, 0.5));
    animation.set_position(window_size / 2.0);

    let description_text = renderer.get_drawable_with_type_mut::<Text>(description_text_id)?;
    description_text.set_text(DESCRIPTION);
    description_text.set_anchor(Vec2::new(0.0, 1.0));
    description_text.set_line_height(20);

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

        std::thread::sleep(std::time::Duration::from_millis(sleep_duration));
        renderer.get_drawable_with_type_mut::<Animation>(animation_id)?.set_next_frame();

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(animation_id)?;
        renderer.draw(description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
