use lemao_core::audio::context::AudioContext;
use lemao_core::audio::samples::wav;
use lemao_core::audio::sounds::Sound;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::window::context::WindowContext;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;
use lemao_ui::context::UiContext;

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
    let mut ui = UiContext::new(&mut renderer)?;
    let mut audio = AudioContext::new()?;

    renderer.set_swap_interval(1);

    let font_id = renderer.fonts.store(Font::new(&renderer, &bff::load("./assets/inconsolata.bff")?)?);
    let chopin_sound_id = audio.sounds.store(Sound::new(&wav::load("./assets/chopin.wav")?)?);

    let description_text_id = ui.components.store(Label::new(&mut renderer, font_id)?);
    let description_text = ui.components.get_and_cast_mut::<Label>(description_text_id)?;
    description_text.label_text = DESCRIPTION.to_string();
    description_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    description_text.offset = Vec2::new(5.0, 0.0);
    description_text.anchor = Vec2::new(0.0, 1.0);
    description_text.label_line_height = 20;
    ui.components.get_mut(ui.main_canvas_id)?.add_child(description_text_id);

    let status_text_id = ui.components.store(Label::new(&mut renderer, font_id)?);
    let status_text = ui.components.get_and_cast_mut::<Label>(status_text_id)?;
    status_text.label_text = "Status: stopped".to_string();
    status_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    status_text.offset = Vec2::new(5.0, -200.0);
    status_text.anchor = Vec2::new(0.0, 1.0);
    status_text.label_line_height = 20;
    ui.components.get_mut(ui.main_canvas_id)?.add_child(status_text_id);

    let chopin_sound = audio.sounds.get_mut(chopin_sound_id)?;
    let mut is_running = true;

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowSizeChanged(size) => {
                    renderer.set_viewport_size(size)?;
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

            ui.process_window_event(&mut renderer, &event)?;
        }

        let status_text = ui.components.get_and_cast_mut::<Label>(status_text_id)?;
        if chopin_sound.is_playing()? {
            status_text.label_text = "Status: playing".to_string();
            status_text.dirty = true;
        } else {
            status_text.label_text = "Status: stopped".to_string();
            status_text.dirty = true;
        }

        ui.update(&mut renderer)?;

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        ui.draw(&mut renderer, description_text_id)?;
        ui.draw(&mut renderer, status_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
