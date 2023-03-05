#![allow(clippy::collapsible_match)]

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::DrawableEnum;

use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::bmp;
use lemao_core::renderer::textures::Texture;
use lemao_core::window::context::WindowContext;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;
use lemao_ui::context::UiContext;
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
    let mut ui = UiContext::new(&mut renderer)?;
    renderer.set_swap_interval(1);

    let texture_id = renderer.textures.store(Texture::new(&renderer, &bmp::load("./cell/disc.bmp")?)?);
    let font_id = renderer.fonts.store(Font::new(&renderer, &bff::load("./assets/inconsolata.bff")?)?);

    let sprite_id = renderer.create_rectangle()?;
    let sprite = renderer.rectangles.get_mut(sprite_id)?;
    sprite.set_texture(renderer.textures.get(texture_id)?);
    sprite.anchor = Vec2::new(0.5, 0.5);
    sprite.position = Vec2::new(400.0, 300.0);
    sprite.size = renderer.textures.get(texture_id)?.size;

    let description_text_id = ui.create_label(&mut renderer, font_id)?;
    let description_text = ui.get_component_and_cast_mut::<Label>(description_text_id)?;
    description_text.label_text = DESCRIPTION.to_string();
    description_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    description_text.offset = Vec2::new(5.0, 0.0);
    description_text.anchor = Vec2::new(0.0, 1.0);
    description_text.label_line_height = 20;
    ui.get_component_mut(ui.main_canvas_id)?.add_child(description_text_id);

    let mut last_update = Instant::now();
    let mut is_running = true;

    while is_running {
        let delta = last_update.elapsed().as_secs_f32();
        last_update = Instant::now();

        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::KeyPressed(key) => {
                    if let Key::Escape = key {
                        window.close()
                    }
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

        let camera = renderer.cameras.get_mut(renderer.active_camera_id)?;
        if window.is_key_pressed(Key::ArrowUp) {
            camera.position += Vec2::new(0.0, 200.0 * delta);
            camera.dirty = true;
        }
        if window.is_key_pressed(Key::ArrowDown) {
            camera.position += Vec2::new(0.0, -200.0 * delta);
            camera.dirty = true;
        }
        if window.is_key_pressed(Key::ArrowLeft) {
            camera.position += Vec2::new(-200.0 * delta, 0.0);
            camera.dirty = true;
        }
        if window.is_key_pressed(Key::ArrowRight) {
            camera.position += Vec2::new(200.0 * delta, 0.0);
            camera.dirty = true;
        }

        let sprite = renderer.rectangles.get_mut(sprite_id)?;
        if window.is_key_pressed(Key::KeyW) {
            sprite.position += Vec2::new(0.0, 200.0 * delta);
        }
        if window.is_key_pressed(Key::KeyS) {
            sprite.position += Vec2::new(0.0, -200.0 * delta);
        }
        if window.is_key_pressed(Key::KeyA) {
            sprite.position += Vec2::new(-200.0 * delta, 0.0);
        }
        if window.is_key_pressed(Key::KeyD) {
            sprite.position += Vec2::new(200.0 * delta, 0.0);
        }
        if window.is_key_pressed(Key::KeyE) {
            sprite.rotation += -2.0 * delta;
        }
        if window.is_key_pressed(Key::KeyQ) {
            sprite.rotation += 2.0 * delta;
        }
        if window.is_key_pressed(Key::Space) {
            sprite.position += Vec2::new_from_angle(sprite.rotation) * 200.0 * Vec2::new(delta, delta);
        }

        ui.update(&mut renderer)?;

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(DrawableEnum::Rectangle, sprite_id)?;
        ui.draw(&mut renderer, description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
