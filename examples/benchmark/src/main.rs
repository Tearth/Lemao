use lemao_core::renderer::textures::storage::TextureStorage;
use lemao_core::window::context::WindowContext;
use lemao_core::window::input::InputEvent;
use lemao_math::color::Color;
use lemao_math::vec2::Vec2;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

pub struct CellData {
    pub sprite_id: usize,
    pub velocity: Vec2,
}

pub fn main() {
    const WINDOW_WIDTH: u32 = 800;
    const WINDOW_HEIGHT: u32 = 600;
    const CELLS_COUNT: usize = 1000;
    const MAX_SPEED: f32 = 1.0;

    let textures = Arc::new(Mutex::new(TextureStorage::default()));
    let mut window = match WindowContext::new("Benchmark", WINDOW_WIDTH, WINDOW_HEIGHT) {
        Ok(window) => window,
        Err(message) => panic!("{}", message),
    };

    let mut renderer = match window.create_renderer(textures.clone()) {
        Ok(renderer) => renderer,
        Err(message) => panic!("{}", message),
    };

    let mut cells = Vec::new();
    let cell_texture_id = textures.lock().unwrap().load("./assets/cell.bmp").unwrap();

    for _ in 0..CELLS_COUNT {
        let cell_sprite_id = renderer.create_sprite(cell_texture_id).unwrap();
        let cell_sprite = renderer.get_drawable_mut(cell_sprite_id).unwrap();
        cell_sprite.set_position(Vec2::new(fastrand::f32() * 800.0, fastrand::f32() * 600.0));

        cells.push(CellData {
            sprite_id: cell_sprite_id,
            velocity: Vec2 { x: MAX_SPEED * (fastrand::f32() * 2.0 - 1.0), y: MAX_SPEED * (fastrand::f32() * 2.0 - 1.0) },
        });
    }

    let mut width = WINDOW_WIDTH;
    let mut height = WINDOW_HEIGHT;

    let mut is_running = true;
    let mut frames = 0;
    let mut now = Instant::now();

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowSizeChanged(w, h) => {
                    renderer.set_viewport(w, h);
                    renderer.get_camera_mut(0).unwrap().set_viewport(Vec2::new(w as f32, h as f32));
                    width = w;
                    height = h;
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }
        }

        renderer.clear(Color::new(0.5, 0.5, 0.5, 1.0));

        for cell in &mut cells {
            let sprite = renderer.get_drawable_mut(cell.sprite_id).unwrap();
            let position = sprite.get_position();

            if position.x <= 0.0 {
                cell.velocity = Vec2::new(cell.velocity.x.abs(), cell.velocity.y);
            }
            if position.x >= width as f32 {
                cell.velocity = Vec2::new(-cell.velocity.x.abs(), cell.velocity.y);
            }
            if position.y <= 0.0 {
                cell.velocity = Vec2::new(cell.velocity.x, cell.velocity.y.abs());
            }
            if position.y >= height as f32 {
                cell.velocity = Vec2::new(cell.velocity.x, -cell.velocity.y.abs());
            }

            sprite.move_toward(cell.velocity);
            renderer.draw(cell.sprite_id);
        }

        if now.elapsed().as_millis() >= 1000 {
            now = Instant::now();

            println!("[{}] FPS: {}", chrono::Utc::now(), frames);
            frames = 0;
        }

        frames += 1;
        window.swap_buffers();
    }
}
