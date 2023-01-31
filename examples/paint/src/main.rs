#![allow(clippy::collapsible_match, clippy::identity_op)]

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::RawTexture;
use lemao_core::renderer::textures::Texture;
use lemao_core::window::context::CoordinationSystem;
use lemao_core::window::context::WindowContext;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Paint:
 LMB - put white pixel";

pub fn main() -> Result<(), String> {
    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Paint", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer()?;

    let mut texture_data = vec![0; (window_size.x * window_size.y * 4.0) as usize];

    let texture_storage = renderer.get_textures();
    let mut texture_storage = texture_storage.write().unwrap();
    let texture_id = texture_storage.store(Texture::new(&renderer, &RawTexture::new(window_size, texture_data.clone())));

    drop(texture_storage);

    let font_storage = renderer.get_fonts();
    let mut font_storage = font_storage.write().unwrap();
    let font_id = font_storage.store(Font::new(&renderer, &bff::load("./assets/inconsolata.bff")?));

    drop(font_storage);

    let sprite_id = renderer.create_rectangle()?;
    let description_text_id = renderer.create_text(font_id)?;

    let texture_storage = renderer.get_textures();
    let texture_storage = texture_storage.read().unwrap();

    let sprite = renderer.get_drawable_with_type_mut::<Rectangle>(sprite_id)?;
    sprite.set_texture(texture_storage.get(texture_id)?);

    drop(texture_storage);

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
                InputEvent::WindowSizeChanged(size) => {
                    renderer.set_viewport_size(size);
                    renderer.get_active_camera_mut()?.set_size(size);
                    renderer.get_drawable_mut(description_text_id)?.set_position(Vec2::new(5.0, size.y - 0.0));
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        if window.is_mouse_button_pressed(MouseButton::Left) {
            let cursor_position = window.get_cursor_position(CoordinationSystem::Window);
            let index = (cursor_position.x + cursor_position.y * window_size.x) as usize;

            if cursor_position.x >= 0.0 && cursor_position.x < window_size.x && cursor_position.y >= 0.0 && cursor_position.y < window_size.y {
                texture_data[index * 4 + 0] = 255;
                texture_data[index * 4 + 1] = 255;
                texture_data[index * 4 + 2] = 255;
                texture_data[index * 4 + 3] = 255;

                let texture_storage = renderer.get_textures();
                let texture_storage = texture_storage.read().unwrap();
                let texture = texture_storage.get(texture_id).unwrap();
                texture.set_data(window_size, &texture_data);
            }
        }

        renderer.clear(SolidColor::new(0.0, 0.0, 0.0, 1.0));
        renderer.draw(sprite_id)?;
        renderer.draw(description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
