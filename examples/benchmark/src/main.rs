use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::DrawableEnum;
use lemao_core::utils::rand;
use lemao_core::window::context::WindowContext;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;
use lemao_ui::context::UiContext;
use std::time::Instant;

#[no_mangle]
#[cfg(windows)]
pub static NvOptimusEnablement: i32 = 1;

#[no_mangle]
#[cfg(windows)]
pub static AmdPowerXpressRequestHighPerformance: i32 = 1;

const CELLS_COUNT: usize = 20000;
const MAX_SPEED: f32 = 0.2;

pub struct CellData {
    pub sprite_id: usize,
    pub position: Vec2,
    pub velocity: Vec2,
}

pub fn main() -> Result<(), String> {
    let window_position = Default::default();
    let mut window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Audio", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer()?;
    let mut ui = UiContext::new(&mut renderer)?;
    renderer.set_swap_interval(0);

    let cell_texture_id = renderer.create_texture("./assets/cell.bmp")?;
    let font_id = renderer.create_font("./assets/inconsolata.bff")?;

    let fps_text_id = ui.create_label(&mut renderer, font_id)?;
    let fps_text = ui.get_component_and_cast_mut::<Label>(fps_text_id)?;
    fps_text.label_text = "FPS:0".to_string();
    fps_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    fps_text.offset = Vec2::new(5.0, 0.0);
    fps_text.anchor = Vec2::new(0.0, 1.0);
    ui.get_component_mut(ui.main_canvas_id)?.add_child(fps_text_id);

    let mut cells = Vec::new();

    for _ in 0..CELLS_COUNT {
        let sprite_id = renderer.create_rectangle()?;
        let sprite = renderer.rectangles.get_mut(sprite_id)?;
        let cell_texture = renderer.textures.get(cell_texture_id)?;

        sprite.anchor = Vec2::new(0.5, 0.5);
        sprite.size = cell_texture.size;
        sprite.set_texture(cell_texture);

        let position = Vec2::new(rand::i32(0..window_size.x as i32) as f32, rand::i32(0..window_size.y as i32) as f32);
        let velocity = Vec2::new(MAX_SPEED * (rand::i32(-100..100) as f32 / 100.0), MAX_SPEED * (rand::i32(-100..100)as f32 / 100.0) );

        cells.push(CellData { sprite_id, position, velocity });
    }

    let mut now = Instant::now();
    let mut frames = 0;
    let mut is_running = true;

    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::WindowSizeChanged(size) => {
                    window_size = size;
                    renderer.set_viewport_size(size)?;
                }
                InputEvent::WindowClosed => {
                    is_running = false;
                }
                _ => {}
            }

            ui.process_window_event(&mut renderer, &event)?;
        }

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));

        for cell in &mut cells {
            let sprite = renderer.rectangles.get_mut(cell.sprite_id)?;
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
            sprite.position = cell.position;
            sprite.update();

            renderer.batcher_add_drawable(DrawableEnum::Rectangle, cell.sprite_id)?;
        }

        if now.elapsed().as_millis() >= 1000 {
            let fps_text = ui.get_component_and_cast_mut::<Label>(fps_text_id)?;
            fps_text.label_text = format!("FPS:{frames}");
            fps_text.dirty = true;

            now = Instant::now();
            frames = 0;
        }

        ui.update(&mut renderer)?;

        frames += 1;
        renderer.batcher_draw()?;
        ui.draw(&mut renderer, fps_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
