use lemao_core::audio::context::AudioContext;
use lemao_core::audio::samples::storage::SampleStorage;
use lemao_core::audio::samples::wav;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::fonts::storage::FontStorage;
use lemao_core::renderer::textures::storage::TextureStorage;
use lemao_core::window::context::WindowContext;
use lemao_core::window::context::WindowStyle;
use lemao_core::window::input::InputEvent;
use lemao_core::window::input::Key;
use lemao_math::color::Color;
use lemao_math::vec2::Vec2;
use std::sync::Arc;
use std::sync::Mutex;

pub fn main() {
    const DEFAULT_WINDOW_WIDTH: u32 = 800;
    const DEFAULT_WINDOW_HEIGHT: u32 = 600;

    let samples = Arc::new(Mutex::new(SampleStorage::default()));
    let mut audio = AudioContext::new(samples.clone()).unwrap();

    let chopin_sample_id = samples.lock().unwrap().store(wav::load("./assets/chopin.wav").unwrap()).unwrap();
    let chopin_sound_id = audio.create_sound(chopin_sample_id).unwrap();

    let textures = Arc::new(Mutex::new(TextureStorage::default()));
    let fonts = Arc::new(Mutex::new(FontStorage::default()));

    let mut window =
        match WindowContext::new("Audio", WindowStyle::Window(Vec2::new(0.0, 0.0), Vec2::new(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32))) {
            Ok(window) => window,
            Err(message) => panic!("{}", message),
        };

    let mut renderer = match window.create_renderer(textures, fonts.clone()) {
        Ok(renderer) => renderer,
        Err(message) => panic!("{}", message),
    };

    let font_id = fonts.lock().unwrap().load("./assets/inconsolata.bff").unwrap();
    let description_text_id = renderer.create_text(font_id).unwrap();
    let status_text_id = renderer.create_text(font_id).unwrap();

    renderer.get_drawable_with_type_mut::<Text>(description_text_id).unwrap().set_text(
        "Music:
 W - play
 S - stop
 A - pause
 D - rewind
 
Volume:
 R - up
 F - down",
    );
    renderer.get_drawable_with_type_mut::<Text>(description_text_id).unwrap().set_anchor(Vec2::new(0.0, 1.0));
    renderer.get_drawable_with_type_mut::<Text>(description_text_id).unwrap().set_line_height(20);

    let description_text_size = renderer.get_drawable_with_type_mut::<Text>(description_text_id).unwrap().get_size();
    renderer.get_drawable_with_type_mut::<Text>(status_text_id).unwrap().set_text("Status: stopped");
    renderer.get_drawable_with_type_mut::<Text>(status_text_id).unwrap().set_anchor(Vec2::new(0.0, 1.0));

    let mut is_running = true;

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowSizeChanged(width, height) => {
                    renderer.set_viewport(width, height);
                    renderer.get_camera_mut(0).unwrap().set_viewport(Vec2::new(width as f32, height as f32));
                    renderer.get_drawable_mut(description_text_id).unwrap().set_position(Vec2::new(5.0, height as f32 - 0.0));
                    renderer.get_drawable_mut(status_text_id).unwrap().set_position(Vec2::new(5.0, height as f32 - description_text_size.y - 20.0));
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                InputEvent::KeyPressed(Key::KeyW) => {
                    audio.play(chopin_sound_id);
                }
                InputEvent::KeyPressed(Key::KeyS) => {
                    audio.stop(chopin_sound_id);
                }
                InputEvent::KeyPressed(Key::KeyA) => {
                    audio.pause(chopin_sound_id);
                }
                InputEvent::KeyPressed(Key::KeyD) => {
                    audio.rewind(chopin_sound_id);
                }
                InputEvent::KeyPressed(Key::KeyR) => {
                    audio.set_volume(chopin_sound_id, audio.get_volume(chopin_sound_id) + 0.1);
                }
                InputEvent::KeyPressed(Key::KeyF) => {
                    audio.set_volume(chopin_sound_id, audio.get_volume(chopin_sound_id) - 0.1);
                }
                _ => {}
            }
        }

        if audio.is_playing(chopin_sound_id) {
            renderer.get_drawable_with_type_mut::<Text>(status_text_id).unwrap().set_text("Status: playing");
        } else {
            renderer.get_drawable_with_type_mut::<Text>(status_text_id).unwrap().set_text("Status: stopped");
        }

        renderer.clear(Color::new(0.5, 0.5, 0.5, 1.0));
        renderer.draw(description_text_id);
        renderer.draw(status_text_id);
        window.swap_buffers();
    }
}
