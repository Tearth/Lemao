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

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Window mode:
 1 - windowed
 2 - borderless
 3 - fullscreen";

pub fn main() -> Result<(), String> {
    let textures = Arc::new(Mutex::new(TextureStorage::default()));
    let fonts = Arc::new(Mutex::new(FontStorage::default()));

    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = match WindowContext::new("Window", WindowStyle::Window { position: window_position, size: window_size }) {
        Ok(window) => window,
        Err(message) => panic!("{}", message),
    };

    let mut renderer = match window.create_renderer(textures, fonts.clone()) {
        Ok(renderer) => renderer,
        Err(message) => panic!("{}", message),
    };

    let font_id = fonts.lock().unwrap().store(bff::load(&renderer, "./assets/inconsolata.bff")?);
    let description_text_id = renderer.create_text(font_id)?;
    let window_status_text_id = renderer.create_text(font_id)?;
    let left_top_text_id = renderer.create_text(font_id)?;
    let right_top_text_id = renderer.create_text(font_id)?;
    let left_bottom_text_id = renderer.create_text(font_id)?;
    let right_bottom_text_id = renderer.create_text(font_id)?;

    let description_text = renderer.get_drawable_with_type_mut::<Text>(description_text_id)?;
    description_text.set_text(DESCRIPTION);
    description_text.set_anchor(Vec2::new(0.0, 1.0));
    description_text.set_line_height(20);

    renderer.get_drawable_with_type_mut::<Text>(left_top_text_id)?.set_anchor(Vec2::new(0.0, 1.0));
    renderer.get_drawable_with_type_mut::<Text>(right_top_text_id)?.set_anchor(Vec2::new(1.0, 1.0));
    renderer.get_drawable_with_type_mut::<Text>(left_bottom_text_id)?.set_anchor(Vec2::new(0.0, 0.0));
    renderer.get_drawable_with_type_mut::<Text>(right_bottom_text_id)?.set_anchor(Vec2::new(1.0, 0.0));
    renderer.get_drawable_with_type_mut::<Text>(window_status_text_id)?.set_anchor(Vec2::new(0.0, 1.0));

    renderer.get_drawable_with_type_mut::<Text>(left_top_text_id)?.set_text("Left top");
    renderer.get_drawable_with_type_mut::<Text>(right_top_text_id)?.set_text("Right top");
    renderer.get_drawable_with_type_mut::<Text>(left_bottom_text_id)?.set_text("Left bottom");
    renderer.get_drawable_with_type_mut::<Text>(right_bottom_text_id)?.set_text("Right bottom");
    renderer.get_drawable_with_type_mut::<Text>(window_status_text_id)?.set_text("");

    let mut old_position = window_position;
    let mut old_size = window_size;
    let mut is_running = true;

    while is_running {
        let mut style_changed = false;
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowMoved(_, _) => {
                    style_changed = true;
                }

                InputEvent::WindowSizeChanged(width, height) => {
                    let left_top_text_size = renderer.get_drawable_with_type_mut::<Text>(left_top_text_id)?.get_size();
                    let description_text_size = renderer.get_drawable_with_type_mut::<Text>(description_text_id)?.get_size();

                    let description_text_margin = left_top_text_size.y - 0.0;
                    let window_status_text_margin = left_top_text_size.y + description_text_size.y + 20.0;

                    renderer.set_viewport(width, height);
                    renderer.get_active_camera_mut()?.set_size(Vec2::new(width as f32, height as f32));

                    renderer.get_drawable_mut(description_text_id)?.set_position(Vec2::new(5.0, height as f32 - description_text_margin));
                    renderer.get_drawable_mut(window_status_text_id)?.set_position(Vec2::new(5.0, height as f32 - window_status_text_margin));
                    renderer.get_drawable_mut(left_top_text_id)?.set_position(Vec2::new(5.0, height as f32));
                    renderer.get_drawable_mut(right_top_text_id)?.set_position(Vec2::new(width as f32 - 5.0, height as f32));
                    renderer.get_drawable_mut(left_bottom_text_id)?.set_position(Vec2::new(5.0, 0.0));
                    renderer.get_drawable_mut(right_bottom_text_id)?.set_position(Vec2::new(width as f32 - 5.0, 0.0));

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
