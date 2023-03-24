#![allow(clippy::collapsible_match, clippy::identity_op)]

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::RawTexture;
use lemao_core::renderer::textures::Texture;
use lemao_core::window::context::CoordinationSystem;
use lemao_core::window::context::WindowContext;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;
use lemao_ui::context::UiContext;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Paint:
 LMB - put white pixel";

pub fn main() -> Result<(), String> {
    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Paint", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer()?;
    let mut ui = UiContext::new(&mut renderer)?;
    renderer.set_swap_interval(1);

    let mut raw_texture = RawTexture::new(window_size, vec![0; (window_size.x * window_size.y * 4.0) as usize]);

    let texture_id = renderer.textures.store(Texture::new(&renderer, &raw_texture)?);
    let font_id = renderer.fonts.store(Font::new(&renderer, &bff::load("./assets/inconsolata.bff")?)?);

    let mut sprite = renderer.create_rectangle()?;
    sprite.set_texture(renderer.textures.get(texture_id)?);
    sprite.size = window_size;

    let description_text_id = ui.components.store(Box::new(Label::new(&mut renderer, font_id)?));
    let description_text = ui.get_component_and_cast_mut::<Label>(description_text_id)?;
    description_text.label_text = DESCRIPTION.to_string();
    description_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    description_text.offset = Vec2::new(5.0, 0.0);
    description_text.anchor = Vec2::new(0.0, 1.0);
    description_text.label_line_height = 20;
    ui.get_component_mut(ui.main_canvas_id)?.add_child(description_text_id);

    let mut is_running = true;

    while is_running {
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

        if window.is_mouse_button_pressed(MouseButton::Left) {
            let cursor_position = window.get_cursor_position(CoordinationSystem::Window);
            let index = (cursor_position.x + cursor_position.y * window_size.x) as usize;

            if cursor_position.x >= 0.0 && cursor_position.x < window_size.x && cursor_position.y >= 0.0 && cursor_position.y < window_size.y {
                raw_texture.data[index * 4 + 0] = 255;
                raw_texture.data[index * 4 + 1] = 255;
                raw_texture.data[index * 4 + 2] = 255;
                raw_texture.data[index * 4 + 3] = 255;

                let texture = renderer.textures.get_mut(texture_id).unwrap();
                texture.set_data(&raw_texture);
            }
        }

        ui.update(&mut renderer)?;

        renderer.clear(SolidColor::new(0.0, 0.0, 0.0, 1.0));
        renderer.draw(&mut sprite)?;
        ui.draw(&mut renderer, description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
