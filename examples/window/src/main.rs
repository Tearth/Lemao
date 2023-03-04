use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::window::context::CoordinationSystem;
use lemao_core::window::context::WindowContext;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Window mode:
 1 - windowed
 2 - borderless
 3 - fullscreen";

pub fn main() -> Result<(), String> {
    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = match WindowContext::new("Window", WindowStyle::Window { position: window_position, size: window_size }) {
        Ok(window) => window,
        Err(message) => panic!("{}", message),
    };

    let mut renderer = match window.create_renderer() {
        Ok(renderer) => renderer,
        Err(message) => panic!("{}", message),
    };
    renderer.set_swap_interval(1);

    let font_id = renderer.create_font("./assets/inconsolata.bff")?;

    let window_status_text_id = renderer.create_text(font_id)?;
    let left_top_text_id = renderer.create_text(font_id)?;
    let right_top_text_id = renderer.create_text(font_id)?;
    let left_bottom_text_id = renderer.create_text(font_id)?;
    let right_bottom_text_id = renderer.create_text(font_id)?;

    let description_text_id = renderer.create_text(font_id)?;
    let description_text = renderer.texts.get_mut(description_text_id)?;
    description_text.text = (DESCRIPTION.to_string());
    description_text.anchor = (Vec2::new(0.0, 1.0));
    description_text.line_height = (20);

    let left_top_text = renderer.texts.get_mut(left_top_text_id)?;
    left_top_text.anchor = (Vec2::new(0.0, 1.0));
    left_top_text.text = ("Left top".to_string());
    left_top_text.update();

    let right_top_text = renderer.texts.get_mut(right_top_text_id)?;
    right_top_text.anchor = (Vec2::new(1.0, 1.0));
    right_top_text.text = ("Right top".to_string());
    right_top_text.update();

    let left_bottom_text = renderer.texts.get_mut(left_bottom_text_id)?;
    left_bottom_text.anchor = (Vec2::new(0.0, 0.0));
    left_bottom_text.text = ("Left bottom".to_string());
    left_bottom_text.update();

    let right_bottom_text = renderer.texts.get_mut(right_bottom_text_id)?;
    right_bottom_text.anchor = (Vec2::new(1.0, 0.0));
    right_bottom_text.text = ("Right bottom".to_string());
    right_bottom_text.update();

    let window_status_text = renderer.texts.get_mut(window_status_text_id)?;
    window_status_text.anchor = (Vec2::new(0.0, 1.0));
    window_status_text.text = ("".to_string());
    window_status_text.update();

    let mut old_position = window_position;
    let mut old_size = window_size;
    let mut is_running = true;

    while is_running {
        let mut style_changed = false;
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowMoved(_) => {
                    style_changed = true;
                }

                InputEvent::WindowSizeChanged(size) => {
                    let left_top_text_size = renderer.get_drawable_and_cast_mut::<Text>(left_top_text_id)?.get_size();
                    let description_text_size = renderer.get_drawable_and_cast_mut::<Text>(description_text_id)?.get_size();

                    let description_text_margin = left_top_text_size.y - 0.0;
                    let window_status_text_margin = left_top_text_size.y + description_text_size.y + 20.0;

                    renderer.set_viewport_size(size);

                    style_changed = true;
                }
                InputEvent::KeyPressed(Key::Key1) => {
                    window.set_style(WindowStyle::Window { position: old_position, size: old_size })?;
                    style_changed = true;
                }
                InputEvent::KeyPressed(Key::Key2) => {
                    if let WindowStyle::Window { position: _, size: _ } = window.get_style() {
                        old_position = window.get_position();
                        old_size = window.get_size();
                    }
                    style_changed = true;

                    window.set_style(WindowStyle::Borderless)?;
                }
                InputEvent::KeyPressed(Key::Key3) => {
                    if let WindowStyle::Window { position: _, size: _ } = window.get_style() {
                        old_position = window.get_position();
                        old_size = window.get_size();
                    }
                    style_changed = true;

                    window.set_style(WindowStyle::Fullscreen)?;
                }
                InputEvent::MouseMoved(_, _) => {
                    style_changed = true;
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        if style_changed {
            renderer.get_drawable_and_cast_mut::<Text>(window_status_text_id)?.set_text(&format!(
                "Window position: {:?}\nWindow size: {:?}\nWindow style: {:?}\nCursor position: {:?}",
                window.get_position(),
                window.get_size(),
                window.get_style(),
                window.get_cursor_position(CoordinationSystem::Window)
            ));
        }

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(description_text_id)?;
        renderer.draw(window_status_text_id)?;
        renderer.draw(left_top_text_id)?;
        renderer.draw(right_top_text_id)?;
        renderer.draw(left_bottom_text_id)?;
        renderer.draw(right_bottom_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
