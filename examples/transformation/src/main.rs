#![allow(clippy::collapsible_match)]

use lemao_core::renderer::drawable::sprite::Sprite;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Drawable;
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
use std::time::Instant;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Transformation:
 WASD - move sprite
 Arrows - move camera
 Q - rotate left
 E - rotate right
 Space - move forward";

pub fn main() -> Result<(), String> {
    let textures = Arc::new(Mutex::new(TextureStorage::default()));
    let fonts = Arc::new(Mutex::new(FontStorage::default()));

    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Transformation", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer(textures.clone(), fonts.clone())?;

    let kaela_rgb = textures.lock().unwrap().store(bmp::load(&renderer, "./assets/circle.bmp")?);
    let font_id = fonts.lock().unwrap().store(bff::load(&renderer, "./assets/inconsolata.bff")?);

    let gui_camera_id = renderer.create_camera(Default::default(), window_size)?;
    let sprite_id = renderer.create_sprite(kaela_rgb)?;
    let description_text_id = renderer.create_text(font_id)?;

    let sprite = renderer.get_drawable_with_type_mut::<Sprite>(sprite_id)?;
    sprite.set_anchor(Vec2::new(0.5, 0.5));
    sprite.set_position(Vec2::new(400.0, 300.0));

    let description_text = renderer.get_drawable_with_type_mut::<Text>(description_text_id)?;
    description_text.set_text(DESCRIPTION);
    description_text.set_anchor(Vec2::new(0.0, 1.0));
    description_text.set_line_height(20);

    let mut last_update = Instant::now();
    let mut is_running = true;
    while is_running {
        let delta = last_update.elapsed().as_secs_f32();
        last_update = Instant::now();

        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::KeyPressed(k) => {
                    if let Key::Escape = k {
                        window.close()
                    }
                }
                InputEvent::WindowSizeChanged(width, height) => {
                    renderer.set_viewport(width, height);
                    renderer.get_active_camera_mut()?.set_size(Vec2::new(width as f32, height as f32));
                    renderer.get_camera_mut(gui_camera_id)?.set_size(Vec2::new(width as f32, height as f32));
                    renderer.get_drawable_mut(description_text_id)?.set_position(Vec2::new(5.0, height as f32 - 0.0));
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        let camera = renderer.get_active_camera_mut()?;
        if input::is_key_pressed(Key::ArrowUp) {
            camera.move_toward(Vec2::new(0.0, 200.0 * delta));
        }
        if input::is_key_pressed(Key::ArrowDown) {
            camera.move_toward(Vec2::new(0.0, -200.0 * delta));
        }
        if input::is_key_pressed(Key::ArrowLeft) {
            camera.move_toward(Vec2::new(-200.0 * delta, 0.0));
        }
        if input::is_key_pressed(Key::ArrowRight) {
            camera.move_toward(Vec2::new(200.0 * delta, 0.0));
        }

        let sprite = renderer.get_drawable_mut(sprite_id)?;
        if input::is_key_pressed(Key::KeyW) {
            sprite.move_delta(Vec2::new(0.0, 200.0 * delta));
        }
        if input::is_key_pressed(Key::KeyS) {
            sprite.move_delta(Vec2::new(0.0, -200.0 * delta));
        }
        if input::is_key_pressed(Key::KeyA) {
            sprite.move_delta(Vec2::new(-200.0 * delta, 0.0));
        }
        if input::is_key_pressed(Key::KeyD) {
            sprite.move_delta(Vec2::new(200.0 * delta, 0.0));
        }
        if input::is_key_pressed(Key::KeyE) {
            sprite.rotate(-2.0 * delta);
        }
        if input::is_key_pressed(Key::KeyQ) {
            sprite.rotate(2.0 * delta);
        }
        if input::is_key_pressed(Key::Space) {
            sprite.move_delta(Vec2::new_from_angle(sprite.get_rotation()) * 200.0 * Vec2::new(delta, delta));
        }

        renderer.clear(Color::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(sprite_id)?;
        renderer.set_camera_as_active(gui_camera_id)?;
        renderer.draw(description_text_id)?;
        renderer.set_default_camera()?;
        window.swap_buffers();
    }

    Ok(())
}
