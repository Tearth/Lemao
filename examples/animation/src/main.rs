#![allow(clippy::collapsible_match, clippy::collapsible_else_if, clippy::implicit_saturating_sub)]

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::input::MouseWheelDirection;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::bmp;
use lemao_core::renderer::textures::Texture;
use lemao_core::window::context::WindowContext;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;
use lemao_ui::context::UiContext;
use std::thread;
use std::time::Duration;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Animation:
 Scroll - set speed";

pub fn main() -> Result<(), String> {
    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Animation", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer()?;
    let mut ui = UiContext::new(&mut renderer)?;
    renderer.set_swap_interval(1);

    let explosion_id = renderer.textures.store(Texture::new(&renderer, &bmp::load("./assets/explosion.bmp")?)?);
    let font_id = renderer.fonts.store(Font::new(&renderer, &bff::load("./assets/inconsolata.bff")?)?);

    let mut animation = renderer.create_tilemap(explosion_id).unwrap();
    animation.size = Vec2::new(128.0, 128.0);
    animation.anchor = Vec2::new(0.5, 0.5);
    animation.position = window_size / 2.0;
    animation.update();

    let description_text_id = ui.components.store(Box::new(Label::new(&mut renderer, font_id)?));
    let description_text = ui.get_component_and_cast_mut::<Label>(description_text_id)?;
    description_text.label_text = DESCRIPTION.to_string();
    description_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    description_text.offset = Vec2::new(5.0, 0.0);
    description_text.anchor = Vec2::new(0.0, 1.0);
    description_text.label_line_height = 20;
    ui.get_component_mut(ui.main_canvas_id)?.add_child(description_text_id);

    let mut sleep_duration = 10;
    let mut is_running = true;
    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::KeyPressed(key) => {
                    if let Key::Escape = key {
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
                    renderer.set_viewport_size(size)?;
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }

            ui.process_window_event(&mut renderer, &event)?;
        }

        thread::sleep(Duration::from_millis(sleep_duration));

        animation.set_next_frame();
        animation.update();

        ui.update(&mut renderer)?;

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(&mut animation)?;
        ui.draw(&mut renderer, description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
