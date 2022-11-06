use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::fonts::storage::FontStorage;
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
    pub position: Vec2,
    pub velocity: Vec2,
}

pub fn main() {
    const DEFAULT_WINDOW_WIDTH: u32 = 800;
    const DEFAULT_WINDOW_HEIGHT: u32 = 600;
    const CELLS_COUNT: usize = 10000;
    const MAX_SPEED: f32 = 1.0;

    let textures = Arc::new(Mutex::new(TextureStorage::default()));
    let fonts = Arc::new(Mutex::new(FontStorage::default()));

    let mut window = match WindowContext::new("Benchmark", DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT) {
        Ok(window) => window,
        Err(message) => panic!("{}", message),
    };

    let mut renderer = match window.create_renderer(textures.clone(), fonts.clone()) {
        Ok(renderer) => renderer,
        Err(message) => panic!("{}", message),
    };

    let mut cells = Vec::new();
    let cell_texture_id = textures.lock().unwrap().load("./assets/cell.bmp").unwrap();
    let cell_sprite_id = renderer.create_sprite(cell_texture_id).unwrap();
    let font_id = fonts.lock().unwrap().load("./assets/inconsolata.bff").unwrap();
    let text_id = renderer.create_text(font_id).unwrap();

    renderer.get_drawable_with_type_mut::<Text>(text_id).unwrap().set_text("FPS:0");
    renderer.get_drawable_with_type_mut::<Text>(text_id).unwrap().set_anchor(Vec2::new(0.0, 1.0));

    for _ in 0..CELLS_COUNT {
        cells.push(CellData {
            sprite_id: cell_sprite_id,
            position: Vec2::new(fastrand::f32() * 800.0, fastrand::f32() * 600.0),
            velocity: Vec2 { x: MAX_SPEED * (fastrand::f32() * 2.0 - 1.0), y: MAX_SPEED * (fastrand::f32() * 2.0 - 1.0) },
        });
    }

    let mut window_size = Vec2::new(DEFAULT_WINDOW_WIDTH as f32, DEFAULT_WINDOW_HEIGHT as f32);
    let mut now = Instant::now();
    let mut is_running = true;
    let mut frames = 0;

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowSizeChanged(width, height) => {
                    window_size = Vec2::new(width as f32, height as f32);

                    renderer.set_viewport(width, height);
                    renderer.get_camera_mut(0).unwrap().set_viewport(window_size);
                    renderer.get_drawable_mut(text_id).unwrap().set_position(Vec2::new(5.0, window_size.y - 0.0));
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
            if cell.position.x <= 0.0 {
                cell.velocity = Vec2::new(cell.velocity.x.abs(), cell.velocity.y);
            }
            if cell.position.x >= window_size.x {
                cell.velocity = Vec2::new(-cell.velocity.x.abs(), cell.velocity.y);
            }
            if cell.position.y <= 0.0 {
                cell.velocity = Vec2::new(cell.velocity.x, cell.velocity.y.abs());
            }
            if cell.position.y >= window_size.y {
                cell.velocity = Vec2::new(cell.velocity.x, -cell.velocity.y.abs());
            }

            cell.position += cell.velocity;
            sprite.set_position(cell.position);
            renderer.draw(cell.sprite_id);
        }

        if now.elapsed().as_millis() >= 1000 {
            renderer.get_drawable_with_type_mut::<Text>(text_id).unwrap().set_text(&format!("FPS:{}", frames));
            now = Instant::now();
            frames = 0;
        }

        frames += 1;
        renderer.draw(text_id);
        window.swap_buffers();
    }
}
