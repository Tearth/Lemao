use lemao_core::audio::context::AudioContext;
use lemao_core::audio::samples::wav;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
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

    let font_storage = renderer.get_fonts();
    let mut font_storage = font_storage.write().unwrap();
    let font_id = font_storage.store(Font::new(&renderer, &bff::load("./assets/inconsolata.bff")?));

    drop(font_storage);

    let sample_storage = audio.get_samples();
    let mut sample_storage = sample_storage.write().unwrap();
    let chopin_sample_id = sample_storage.store(wav::load(&audio, "./assets/chopin.wav")?);

    drop(sample_storage);

    let description_text_id = renderer.create_text(font_id)?;
    let status_text_id = renderer.create_text(font_id)?;
    let chopin_sound_id = audio.create_sound(chopin_sample_id)?;

    let description_text = renderer.get_drawable_with_type_mut::<Text>(description_text_id)?;
    description_text.set_text(DESCRIPTION);
    description_text.set_anchor(Vec2::new(0.0, 1.0));
    description_text.set_line_height(20);

    let status_text = renderer.get_drawable_with_type_mut::<Text>(status_text_id)?;
    status_text.set_text("Status: stopped");
    status_text.set_anchor(Vec2::new(0.0, 1.0));

    let chopin_sound = audio.get_sound_mut(chopin_sound_id)?;
    let mut is_running = true;

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowSizeChanged(size) => {
                    let description_text_size = renderer.get_drawable_with_type_mut::<Text>(description_text_id)?.get_size();

                    renderer.set_viewport_size(size);
                    renderer.get_active_camera_mut()?.set_size(size);
                    renderer.get_drawable_mut(description_text_id)?.set_position(Vec2::new(5.0, size.y - 0.0));
                    renderer.get_drawable_mut(status_text_id)?.set_position(Vec2::new(5.0, size.y - description_text_size.y - 20.0));
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

        let status_text = renderer.get_drawable_with_type_mut::<Text>(status_text_id)?;
        if chopin_sound.is_playing()? {
            status_text.set_text("Status: playing");
        } else {
            status_text.set_text("Status: stopped");
        }

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(description_text_id)?;
        renderer.draw(status_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
