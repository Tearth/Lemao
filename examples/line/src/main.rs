use lemao_core::renderer::drawable::line::Line;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::storage::FontStorage;
use lemao_core::renderer::textures::bmp;
use lemao_core::renderer::textures::storage::TextureStorage;
use lemao_core::window::context::WindowContext;
use lemao_core::window::context::WindowStyle;
use lemao_core::window::input;
use lemao_core::window::input::InputEvent;
use lemao_core::window::input::Key;
use lemao_core::window::input::MouseButton;
use lemao_math::color::Color;
use lemao_math::vec2::Vec2;
use std::sync::Arc;
use std::sync::Mutex;

pub fn main() -> Result<(), String> {
    let textures = Arc::new(Mutex::new(TextureStorage::default()));
    let fonts = Arc::new(Mutex::new(FontStorage::default()));

    let mut window = match WindowContext::new("Test", WindowStyle::Window(Vec2::new(0.0, 0.0), Vec2::new(800.0, 600.0))) {
        Ok(window) => window,
        Err(message) => panic!("{}", message),
    };

    let mut renderer = match window.create_renderer(textures.clone(), fonts.clone()) {
        Ok(renderer) => renderer,
        Err(message) => panic!("{}", message),
    };
    let font_id = fonts.lock().unwrap().store(bff::load(&renderer, "./assets/inconsolata.bff")?);

    let mut is_running = true;
    let line_id = renderer.create_line(Vec2::new(200.0, 200.0), Vec2::new(400.0, 400.0)).unwrap();
    // renderer.get_drawable_mut(line_id).unwrap().set_scale(Vec2::new(1.0, 2.0));
    // renderer.get_drawable_mut(line_id).unwrap().set_anchor(Vec2::new(0.0, 0.0));
    renderer.get_drawable_with_type_mut::<Line>(line_id).unwrap().set_thickness(10.0);

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::KeyPressed(k) => match k {
                    Key::Escape => window.close(),
                    _ => {}
                },
                InputEvent::CharPressed(c) => println!("{:?}", c),
                InputEvent::WindowSizeChanged(width, height) => {
                    let window_size = Vec2::new(width as f32, height as f32);

                    renderer.set_viewport(width, height);
                    renderer.get_camera_mut(0)?.set_size(window_size);
                    renderer.get_camera_mut(0).unwrap().set_size(Vec2::new(width as f32, height as f32));
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        if input::is_mouse_button_pressed(MouseButton::Left) {
            let window_size = window.get_size();
            let position = input::get_cursor_position(&window);
            renderer.get_drawable_with_type_mut::<Line>(line_id).unwrap().set_to(Vec2::new(position.0 as f32, window_size.y - position.1 as f32));
        }

        if input::is_mouse_button_pressed(MouseButton::Right) {
            let window_size = window.get_size();
            let position = input::get_cursor_position(&window);
            renderer.get_drawable_with_type_mut::<Line>(line_id).unwrap().set_from(Vec2::new(position.0 as f32, window_size.y - position.1 as f32));
        }

        renderer.clear(Color::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(line_id)?;
        window.swap_buffers();
    }

    Ok(())
}
