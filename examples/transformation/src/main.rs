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
    let kaela_rgb = textures.lock().unwrap().store(bmp::load(&renderer, "./assets/circle.bmp").unwrap());
    let font_id = fonts.lock().unwrap().store(bff::load(&renderer, "./assets/inconsolata.bff")?);
    let sprite_id = renderer.create_text(font_id)?;

    let mut is_running = true;
    let sprite_id = renderer.create_sprite(kaela_rgb).unwrap();
    // renderer.get_drawable_mut(sprite_id).unwrap().set_anchor(Vec2::new(1.0, 1.0));
    // renderer.get_drawable_with_type_mut::<Text>(sprite_id)?.set_text("TEST 123\nTEST 123\nTEST 123");

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::KeyPressed(k) => match k {
                    Key::Escape => window.close(),
                    _ => println!("Pressed {:?}", k),
                },
                InputEvent::KeyReleased(k) => println!("Released {:?}", k),
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

        let camera = renderer.get_active_camera_mut().unwrap();
        if input::is_key_pressed(Key::ArrowUp) {
            camera.move_toward(Vec2::new(0.0, 0.2));
        }
        if input::is_key_pressed(Key::ArrowDown) {
            camera.move_toward(Vec2::new(0.0, -0.2));
        }
        if input::is_key_pressed(Key::ArrowLeft) {
            camera.move_toward(Vec2::new(-0.2, 0.0));
        }
        if input::is_key_pressed(Key::ArrowRight) {
            camera.move_toward(Vec2::new(0.2, 0.0));
        }

        let sprite = renderer.get_drawable_mut(sprite_id).unwrap();
        if input::is_key_pressed(Key::KeyW) {
            sprite.move_delta(Vec2::new(0.0, 0.2));
        }
        if input::is_key_pressed(Key::KeyS) {
            sprite.move_delta(Vec2::new(0.0, -0.2));
        }
        if input::is_key_pressed(Key::KeyA) {
            sprite.move_delta(Vec2::new(-0.2, 0.0));
        }
        if input::is_key_pressed(Key::KeyD) {
            sprite.move_delta(Vec2::new(0.2, 0.0));
        }
        if input::is_key_pressed(Key::KeyE) {
            sprite.rotate(-0.001);
        }
        if input::is_key_pressed(Key::KeyQ) {
            sprite.rotate(0.001);
        }
        if input::is_key_pressed(Key::Space) {
            sprite.move_delta(Vec2::new_from_angle(sprite.get_rotation()) / 10.0);
        }

        renderer.clear(Color::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(sprite_id)?;
        window.swap_buffers();
    }

    Ok(())
}
