use lemao_core::audio::context::AudioContext;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;

use lemao_core::renderer::drawable::DrawableEnum;
use lemao_core::window::context::WindowContext;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Music:
 W - play
 S - stop
 A - pause
 D - rewind
 
Volume:
 R - up
 F - down";

pub fn main() -> Result<(), String> {
    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Audio", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer()?;
    let mut audio = AudioContext::new()?;

    renderer.set_swap_interval(1);

    let font_id = renderer.create_font("./assets/inconsolata.bff")?;
    let chopin_sample_id = audio.create_sample("./assets/chopin.wav")?;
    let chopin_sound_id = audio.create_sound(chopin_sample_id)?;

    let description_text_id = renderer.create_text(font_id)?;
    let description_text = renderer.texts.get_mut(description_text_id)?;
    description_text.text = DESCRIPTION.to_string();
    description_text.anchor = Vec2::new(0.0, 1.0);
    description_text.line_height = 20;
    description_text.update();

    let status_text_id = renderer.create_text(font_id)?;
    let status_text = renderer.texts.get_mut(status_text_id)?;
    status_text.text = "Status: stopped".to_string();
    status_text.anchor = Vec2::new(0.0, 1.0);
    status_text.update();

    let chopin_sound = audio.sounds.get_mut(chopin_sound_id)?;
    let mut is_running = true;

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowSizeChanged(size) => {
                    let description_text_size = renderer.texts.get_mut(description_text_id)?.size;

                    renderer.set_viewport_size(size)?;

                    renderer.texts.get_mut(description_text_id)?.position = Vec2::new(5.0, size.y - 0.0);
                    renderer.texts.get_mut(description_text_id)?.update();

                    renderer.texts.get_mut(status_text_id)?.position = Vec2::new(5.0, size.y - description_text_size.y - 20.0);
                    renderer.texts.get_mut(status_text_id)?.update();
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                InputEvent::KeyPressed(Key::KeyW) => chopin_sound.play()?,
                InputEvent::KeyPressed(Key::KeyS) => chopin_sound.stop()?,
                InputEvent::KeyPressed(Key::KeyA) => chopin_sound.pause()?,
                InputEvent::KeyPressed(Key::KeyD) => chopin_sound.rewind()?,
                InputEvent::KeyPressed(Key::KeyR) => chopin_sound.set_volume((chopin_sound.get_volume()? + 0.1).clamp(0.0, 1.0))?,
                InputEvent::KeyPressed(Key::KeyF) => chopin_sound.set_volume((chopin_sound.get_volume()? - 0.1).clamp(0.0, 1.0))?,
                _ => {}
            }
        }

        let status_text = renderer.texts.get_mut(status_text_id)?;
        if chopin_sound.is_playing()? {
            status_text.text = "Status: playing".to_string();
        } else {
            status_text.text = "Status: stopped".to_string();
        }

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(DrawableEnum::Text, description_text_id)?;
        renderer.draw(DrawableEnum::Text, status_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
