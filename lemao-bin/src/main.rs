use lemao_core::window::context::WindowContext;
use lemao_core::window::input;
use lemao_core::window::input::InputEvent;
use lemao_core::window::input::Key;
use lemao_math::color::Color;

fn main() {
    let window = WindowContext::new("Test", 800, 600);
    let mut is_running = true;

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::KeyPressed(k) => match k {
                    Key::Escape => window.close(),
                    _ => println!("Pressed {:?}", k),
                },
                InputEvent::KeyReleased(k) => println!("Released {:?}", k),
                InputEvent::CharPressed(c) => println!("{:?}", c),
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        if input::is_key_pressed(Key::Space) {
            println!("Pressed Space");
        }

        window.clear(Color::new(0.0, 0.0, 0.1, 1.0));
        window.swap_buffers();
    }
}
