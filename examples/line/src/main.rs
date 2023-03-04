#![allow(clippy::collapsible_match)]

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_common_platform::input::MouseWheelDirection;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::line::Line;
use lemao_core::renderer::drawable::DrawableEnum;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::window::context::CoordinationSystem;
use lemao_core::window::context::WindowContext;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Line:
 LMB - set from
 RMB - set to
 Scroll - set thickness";

pub fn main() -> Result<(), String> {
    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Line", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer()?;
    renderer.set_swap_interval(1);

    let font_id = renderer.create_font("./assets/inconsolata.bff")?;

    let line_id = renderer.create_line().unwrap();
    let line = renderer.lines.get_mut(line_id)?;
    line.from = (Vec2::new(200.0, 200.0));
    line.to = (Vec2::new(400.0, 400.0));

    let description_text_id = renderer.create_text(font_id)?;
    let description_text = renderer.texts.get_mut(description_text_id)?;
    description_text.text = (DESCRIPTION.to_string());
    description_text.anchor = (Vec2::new(0.0, 1.0));
    description_text.line_height = (20);
    description_text.update();

    let mut is_running = true;
    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::KeyPressed(k) => {
                    if let Key::Escape = k {
                        window.close();
                    }
                }
                InputEvent::MouseWheelRotated(direction, _) => {
                    let line = renderer.lines.get_mut(line_id)?;
                    if direction == MouseWheelDirection::Up {
                        line.thickness = (line.thickness + 1.0);
                    } else {
                        line.thickness = ((line.thickness - 1.0).max(1.0));
                    }

                    line.update();
                }
                InputEvent::WindowSizeChanged(size) => {
                    renderer.set_viewport_size(size)?;
                    renderer.texts.get_mut(description_text_id)?.position = (Vec2::new(5.0, size.y - 0.0));
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        if window.is_mouse_button_pressed(MouseButton::Left) {
            let position = window.get_cursor_position(CoordinationSystem::Window);
            renderer.lines.get_mut(line_id)?.from = position;
            renderer.lines.get_mut(line_id)?.update();
        }

        if window.is_mouse_button_pressed(MouseButton::Right) {
            let position = window.get_cursor_position(CoordinationSystem::Window);
            renderer.lines.get_mut(line_id)?.to = (position);
            renderer.lines.get_mut(line_id)?.update();
        }

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(DrawableEnum::Line, line_id)?;
        renderer.draw(DrawableEnum::Text, description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
