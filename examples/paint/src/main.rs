#![allow(clippy::collapsible_match, clippy::identity_op)]

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::Color;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::storage::FontStorage;
use lemao_core::renderer::textures::storage::TextureStorage;
use lemao_core::renderer::textures::Texture;
use lemao_core::window::context::WindowContext;
use std::sync::Arc;
use std::sync::Mutex;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Paint:
 LMB - put white pixel";

pub fn main() -> Result<(), String> {
    let textures = Arc::new(Mutex::new(TextureStorage::default()));
    let fonts = Arc::new(Mutex::new(FontStorage::default()));

    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Paint", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer(textures.clone(), fonts.clone())?;

    let mut texture_data = vec![0; (window_size.x * window_size.y * 4.0) as usize];
    let texture_id = textures.lock().unwrap().store(Texture::new(&renderer, window_size, texture_data.clone()));
    let font_id = fonts.lock().unwrap().store(bff::load(&renderer, "./assets/inconsolata.bff")?);

    let sprite_id = renderer.create_sprite(texture_id)?;
    let description_text_id = renderer.create_text(font_id)?;

    let description_text = renderer.get_drawable_with_type_mut::<Text>(description_text_id)?;
    description_text.set_text(DESCRIPTION);
    description_text.set_anchor(Vec2::new(0.0, 1.0));
    description_text.set_line_height(20);

    let mut is_running = true;

    while is_running {
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
                    renderer.get_drawable_mut(description_text_id)?.set_position(Vec2::new(5.0, height as f32 - 0.0));
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        if window.is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = window.get_cursor_position();
            let index = (mouse_position.0 as usize) + ((window_size.y as usize) - (mouse_position.1 as usize)) * (window_size.x as usize);

            if index < texture_data.len() {
                texture_data[index * 4 + 0] = 255;
                texture_data[index * 4 + 1] = 255;
                texture_data[index * 4 + 2] = 255;
                texture_data[index * 4 + 3] = 255;

                let texture_storage = textures.lock().unwrap();
                let texture = texture_storage.get(texture_id).unwrap();
                texture.set_data(window_size, texture_data.clone());
            }
        }

        renderer.clear(Color::new(0.0, 0.0, 0.0, 1.0));
        renderer.draw(sprite_id)?;
        renderer.draw(description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}