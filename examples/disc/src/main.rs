#![allow(clippy::collapsible_match, clippy::collapsible_else_if)]

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_common_platform::input::MouseWheelDirection;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::DrawableEnum;
use lemao_core::window::context::CoordinationSystem;
use lemao_core::window::context::WindowContext;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;
use lemao_ui::context::UiContext;

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
    let mut ui = UiContext::new(&mut renderer)?;
    renderer.set_swap_interval(1);

    let font_id = renderer.create_font("./assets/inconsolata.bff")?;

    let disc_id = renderer.create_disc()?;
    let disc = renderer.discs.get_mut(disc_id)?;
    disc.size = Vec2::new(100.0, 100.0);
    disc.sides = 32;
    disc.anchor = Vec2::new(0.5, 0.5);
    disc.position = Vec2::new(400.0, 300.0);
    disc.update();

    let description_text_id = ui.create_label(&mut renderer, font_id)?;
    let description_text = ui.get_component_and_cast_mut::<Label>(description_text_id)?;
    description_text.label_text = DESCRIPTION.to_string();
    description_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    description_text.offset = Vec2::new(5.0, 0.0);
    description_text.anchor = Vec2::new(0.0, 1.0);
    description_text.label_line_height = 20;
    ui.get_component_mut(ui.main_canvas_id)?.add_child(description_text_id);

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
                    let disc = renderer.discs.get_mut(disc_id)?;
                    if direction == MouseWheelDirection::Up {
                        disc.sides += 1;
                    } else {
                        if disc.sides > 3 {
                            disc.sides -= 1;
                        }
                    }

                    disc.update();
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

        if window.is_mouse_button_pressed(MouseButton::Left) {
            let position = window.get_cursor_position(CoordinationSystem::Window);
            renderer.discs.get_mut(disc_id)?.position = position;
            renderer.discs.get_mut(disc_id)?.update()
        }

        ui.update(&mut renderer)?;

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(DrawableEnum::Disc, disc_id)?;
        ui.draw(&mut renderer, description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
