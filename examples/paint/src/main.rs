#![allow(clippy::collapsible_match, clippy::identity_op)]

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::DrawableEnum;
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
    renderer.set_swap_interval(1);

    let mut texture_data = vec![0; (window_size.x * window_size.y * 4.0) as usize];

    let texture_id = renderer.textures.store(Texture::new(&renderer, &RawTexture::new(window_size, texture_data.clone()))?);
    renderer.textures.get_mut(texture_id)?.id = texture_id;

    let font_id = renderer.create_font("./assets/inconsolata.bff")?;

    let sprite_id = renderer.create_rectangle()?;
    let sprite = renderer.rectangles.get_mut(sprite_id)?;
    sprite.set_texture(renderer.textures.get(texture_id)?);
    sprite.size = window_size;

    let description_text_id = renderer.create_text(font_id)?;
    let description_text = renderer.texts.get_mut(description_text_id)?;
    description_text.text = (DESCRIPTION.to_string());
    description_text.anchor = (Vec2::new(0.0, 1.0));
    description_text.line_height = (20);
    description_text.update();

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
                    renderer.set_viewport_size(size)?;
                    renderer.texts.get_mut(description_text_id)?.position = (Vec2::new(5.0, size.y - 0.0));
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

                let texture = renderer.textures.get_mut(texture_id).unwrap();
                texture.set_data(window_size, &texture_data);
            }
        }

        renderer.clear(SolidColor::new(0.0, 0.0, 0.0, 1.0));
        renderer.draw(DrawableEnum::Rectangle, sprite_id)?;
        renderer.draw(DrawableEnum::Text, description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
