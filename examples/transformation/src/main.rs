#![allow(clippy::collapsible_match)]

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::bmp;
use lemao_core::renderer::textures::Texture;
use lemao_core::window::context::WindowContext;
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
    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Transformation", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer()?;

    let texture_storage = renderer.get_textures();
    let mut texture_storage = texture_storage.write().unwrap();
    let kaela_rgb = texture_storage.store(Box::new(Texture::new(&renderer, &bmp::load("./assets/disc.bmp")?)));

    drop(texture_storage);

    let font_storage = renderer.get_fonts();
    let mut font_storage = font_storage.write().unwrap();
    let font_id = font_storage.store(Box::new(Font::new(&renderer, &bff::load("./assets/inconsolata.bff")?)));

    drop(font_storage);

    let gui_camera_id = renderer.create_camera(Default::default(), window_size)?;
    let sprite_id = renderer.create_rectangle()?;
    let description_text_id = renderer.create_text(font_id)?;

    let texture_storage = renderer.get_textures();
    let texture_storage = texture_storage.read().unwrap();

    let sprite = renderer.get_drawable_with_type_mut::<Rectangle>(sprite_id)?;
    sprite.set_texture(texture_storage.get_and_cast::<Texture>(kaela_rgb)?);
    sprite.set_anchor(Vec2::new(0.5, 0.5));
    sprite.set_position(Vec2::new(400.0, 300.0));

    drop(texture_storage);

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
                InputEvent::WindowSizeChanged(size) => {
                    renderer.set_viewport_size(size);
                    renderer.get_active_camera_mut()?.set_size(size);
                    renderer.get_camera_mut(gui_camera_id)?.set_size(size);
                    renderer.get_drawable_mut(description_text_id)?.set_position(Vec2::new(5.0, size.y - 0.0));
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        let camera = renderer.get_active_camera_mut()?;
        if window.is_key_pressed(Key::ArrowUp) {
            camera.move_toward(Vec2::new(0.0, 200.0 * delta));
        }
        if window.is_key_pressed(Key::ArrowDown) {
            camera.move_toward(Vec2::new(0.0, -200.0 * delta));
        }
        if window.is_key_pressed(Key::ArrowLeft) {
            camera.move_toward(Vec2::new(-200.0 * delta, 0.0));
        }
        if window.is_key_pressed(Key::ArrowRight) {
            camera.move_toward(Vec2::new(200.0 * delta, 0.0));
        }

        let sprite = renderer.get_drawable_mut(sprite_id)?;
        if window.is_key_pressed(Key::KeyW) {
            sprite.move_delta(Vec2::new(0.0, 200.0 * delta));
        }
        if window.is_key_pressed(Key::KeyS) {
            sprite.move_delta(Vec2::new(0.0, -200.0 * delta));
        }
        if window.is_key_pressed(Key::KeyA) {
            sprite.move_delta(Vec2::new(-200.0 * delta, 0.0));
        }
        if window.is_key_pressed(Key::KeyD) {
            sprite.move_delta(Vec2::new(200.0 * delta, 0.0));
        }
        if window.is_key_pressed(Key::KeyE) {
            sprite.rotate(-2.0 * delta);
        }
        if window.is_key_pressed(Key::KeyQ) {
            sprite.rotate(2.0 * delta);
        }
        if window.is_key_pressed(Key::Space) {
            sprite.move_delta(Vec2::new_from_angle(sprite.get_rotation()) * 200.0 * Vec2::new(delta, delta));
        }

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(sprite_id)?;
        renderer.set_camera_as_active(gui_camera_id)?;
        renderer.draw(description_text_id)?;
        renderer.set_default_camera()?;
        window.swap_buffers();
    }

    Ok(())
}
