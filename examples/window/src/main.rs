use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::window::context::CoordinationSystem;
use lemao_core::window::context::WindowContext;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;
use lemao_ui::context::UiContext;

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
    let mut ui = UiContext::new(&mut renderer)?;

    renderer.set_swap_interval(1);

    let font_id = renderer.fonts.store(Font::new(&renderer, &bff::load("./assets/inconsolata.bff")?)?);

    let description_text_id = ui.components.store(Box::new(Label::new(&mut renderer, font_id)?));
    let description_text = ui.get_component_and_cast_mut::<Label>(description_text_id)?;
    description_text.label_text = DESCRIPTION.to_string();
    description_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    description_text.offset = Vec2::new(5.0, -30.0);
    description_text.anchor = Vec2::new(0.0, 1.0);
    description_text.label_line_height = 20;
    ui.get_component_mut(ui.main_canvas_id)?.add_child(description_text_id);

    let left_top_text_id = ui.components.store(Box::new(Label::new(&mut renderer, font_id)?));
    let left_top_text = ui.get_component_and_cast_mut::<Label>(left_top_text_id)?;
    left_top_text.label_text = "Left top".to_string();
    left_top_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    left_top_text.offset = Vec2::new(5.0, 0.0);
    left_top_text.anchor = Vec2::new(0.0, 1.0);
    ui.get_component_mut(ui.main_canvas_id)?.add_child(left_top_text_id);

    let right_top_text_id = ui.components.store(Box::new(Label::new(&mut renderer, font_id)?));
    let right_top_text = ui.get_component_and_cast_mut::<Label>(right_top_text_id)?;
    right_top_text.label_text = "Right top".to_string();
    right_top_text.position = ComponentPosition::RelativeToParent(Vec2::new(1.0, 1.0));
    right_top_text.offset = Vec2::new(-5.0, 0.0);
    right_top_text.anchor = Vec2::new(1.0, 1.0);
    ui.get_component_mut(ui.main_canvas_id)?.add_child(right_top_text_id);

    let left_bottom_text_id = ui.components.store(Box::new(Label::new(&mut renderer, font_id)?));
    let left_bottom_text = ui.get_component_and_cast_mut::<Label>(left_bottom_text_id)?;
    left_bottom_text.label_text = "Left bottom".to_string();
    left_bottom_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 0.0));
    left_bottom_text.offset = Vec2::new(5.0, 0.0);
    left_bottom_text.anchor = Vec2::new(0.0, 0.0);
    ui.get_component_mut(ui.main_canvas_id)?.add_child(left_bottom_text_id);

    let right_bottom_text_id = ui.components.store(Box::new(Label::new(&mut renderer, font_id)?));
    let right_bottom_text = ui.get_component_and_cast_mut::<Label>(right_bottom_text_id)?;
    right_bottom_text.label_text = "Right bottom".to_string();
    right_bottom_text.position = ComponentPosition::RelativeToParent(Vec2::new(1.0, 0.0));
    right_bottom_text.offset = Vec2::new(-5.0, 0.0);
    right_bottom_text.anchor = Vec2::new(1.0, 0.0);
    ui.get_component_mut(ui.main_canvas_id)?.add_child(right_bottom_text_id);

    let window_status_text_id = ui.components.store(Box::new(Label::new(&mut renderer, font_id)?));
    let window_status_text = ui.get_component_and_cast_mut::<Label>(window_status_text_id)?;
    window_status_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    window_status_text.offset = Vec2::new(5.0, -120.0);
    window_status_text.anchor = Vec2::new(0.0, 1.0);
    ui.get_component_mut(ui.main_canvas_id)?.add_child(window_status_text_id);

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
                    renderer.set_viewport_size(size)?;
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

            ui.process_window_event(&mut renderer, &event)?;
        }

        if style_changed {
            ui.get_component_and_cast_mut::<Label>(window_status_text_id)?.label_text = format!(
                "Window position: {:?}\nWindow size: {:?}\nWindow style: {:?}\nCursor position: {:?}",
                window.get_position(),
                window.get_size(),
                window.get_style(),
                window.get_cursor_position(CoordinationSystem::Window)
            );
            ui.get_component_and_cast_mut::<Label>(window_status_text_id)?.dirty = true;
        }

        ui.update(&mut renderer)?;

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        ui.draw(&mut renderer, description_text_id)?;
        ui.draw(&mut renderer, window_status_text_id)?;
        ui.draw(&mut renderer, left_top_text_id)?;
        ui.draw(&mut renderer, right_top_text_id)?;
        ui.draw(&mut renderer, left_bottom_text_id)?;
        ui.draw(&mut renderer, right_bottom_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
