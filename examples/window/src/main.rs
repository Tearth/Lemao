use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::storage::FontStorage;
use lemao_core::renderer::textures::storage::TextureStorage;
use lemao_core::window::context::WindowContext;
use lemao_core::window::context::WindowStyle;
use lemao_core::window::input::InputEvent;
use lemao_core::window::input::Key;
use lemao_math::color::Color;
use lemao_math::vec2::Vec2;
use std::sync::Arc;
use std::sync::Mutex;

pub fn main() -> Result<(), String> {
    let default_position = Vec2::new(0.0, 0.0);
    let default_size = Vec2::new(800.0, 600.0);
    let textures = Arc::new(Mutex::new(TextureStorage::default()));
    let fonts = Arc::new(Mutex::new(FontStorage::default()));

    let mut window = match WindowContext::new("Window", WindowStyle::Window(default_position, default_size)) {
        Ok(window) => window,
        Err(message) => panic!("{}", message),
    };

    let mut renderer = match window.create_renderer(textures, fonts.clone()) {
        Ok(renderer) => renderer,
        Err(message) => panic!("{}", message),
    };

    let font_id = fonts.lock().unwrap().store(bff::load(&renderer, "./assets/inconsolata.bff")?);
    let left_top_text_id = renderer.create_text(font_id)?;
    let right_top_text_id = renderer.create_text(font_id)?;
    let left_bottom_text_id = renderer.create_text(font_id)?;
    let right_bottom_text_id = renderer.create_text(font_id)?;
    let window_status_text_id = renderer.create_text(font_id)?;

    renderer.get_drawable_with_type_mut::<Text>(left_top_text_id)?.set_anchor(Vec2::new(0.0, 1.0))?;
    renderer.get_drawable_with_type_mut::<Text>(right_top_text_id)?.set_anchor(Vec2::new(1.0, 1.0))?;
    renderer.get_drawable_with_type_mut::<Text>(left_bottom_text_id)?.set_anchor(Vec2::new(0.0, 0.0))?;
    renderer.get_drawable_with_type_mut::<Text>(right_bottom_text_id)?.set_anchor(Vec2::new(1.0, 0.0))?;
    renderer.get_drawable_with_type_mut::<Text>(window_status_text_id)?.set_anchor(Vec2::new(0.0, 1.0))?;

    renderer.get_drawable_with_type_mut::<Text>(left_top_text_id)?.set_text("Left top");
    renderer.get_drawable_with_type_mut::<Text>(right_top_text_id)?.set_text("Right top");
    renderer.get_drawable_with_type_mut::<Text>(left_bottom_text_id)?.set_text("Left bottom");
    renderer.get_drawable_with_type_mut::<Text>(right_bottom_text_id)?.set_text("Right bottom");
    renderer.get_drawable_with_type_mut::<Text>(window_status_text_id)?.set_text("");

    let mut old_position = default_position;
    let mut old_size = default_size;
    let mut is_running = true;

    while is_running {
        let mut style_changed = false;
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowMoved(_, _) => {
                    style_changed = true;
                }

                InputEvent::WindowSizeChanged(width, height) => {
                    renderer.set_viewport(width, height);
                    renderer.get_camera_mut(0)?.set_size(Vec2::new(width as f32, height as f32));

                    renderer.get_drawable_mut(left_top_text_id)?.set_position(Vec2::new(5.0, height as f32));
                    renderer.get_drawable_mut(right_top_text_id)?.set_position(Vec2::new(width as f32 - 5.0, height as f32));
                    renderer.get_drawable_mut(left_bottom_text_id)?.set_position(Vec2::new(5.0, 0.0));
                    renderer.get_drawable_mut(right_bottom_text_id)?.set_position(Vec2::new(width as f32 - 5.0, 0.0));
                    renderer.get_drawable_mut(window_status_text_id)?.set_position(Vec2::new(5.0, height as f32 - 40.0));

                    style_changed = true;
                }
                InputEvent::KeyPressed(Key::Key1) => {
                    window.set_style(WindowStyle::Window(old_position, old_size))?;
                    style_changed = true;
                }
                InputEvent::KeyPressed(Key::Key2) => {
                    if let WindowStyle::Window(_, _) = window.get_style() {
                        old_position = window.get_position();
                        old_size = window.get_size();
                    }
                    style_changed = true;

                    window.set_style(WindowStyle::Borderless)?;
                }
                InputEvent::KeyPressed(Key::Key3) => {
                    if let WindowStyle::Window(_, _) = window.get_style() {
                        old_position = window.get_position();
                        old_size = window.get_size();
                    }
                    style_changed = true;

                    window.set_style(WindowStyle::Fullscreen)?;
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        if style_changed {
            renderer.get_drawable_with_type_mut::<Text>(window_status_text_id)?.set_text(&format!(
                "Window position: {:?}\nWindow size: {:?}\nWindow style: {:?}",
                window.get_position(),
                window.get_size(),
                window.get_style()
            ));
        }

        renderer.clear(Color::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(left_top_text_id)?;
        renderer.draw(right_top_text_id)?;
        renderer.draw(left_bottom_text_id)?;
        renderer.draw(right_bottom_text_id)?;
        renderer.draw(window_status_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
