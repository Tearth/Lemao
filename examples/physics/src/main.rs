#![allow(clippy::collapsible_match)]

use std::time::SystemTime;

use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::window::WindowStyle;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::window::context::WindowContext;
use lemao_physics::body::Body;
use lemao_physics::body::BodyShape;
use lemao_physics::context::PhysicsContext;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;
use lemao_ui::context::UiContext;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Physics test";

pub fn main() -> Result<(), String> {
    let window_position = Default::default();
    let window_size = Vec2::new(1366.0, 768.0);

    let mut window = WindowContext::new("Physics", WindowStyle::Window { position: window_position, size: window_size })?;
    let mut renderer = window.create_renderer()?;
    let mut physics = PhysicsContext::new(&mut renderer)?;
    let mut ui = UiContext::new(&mut renderer)?;
    renderer.set_swap_interval(1);

    let font_id = renderer.fonts.store(Font::new(&renderer, &bff::load("./assets/inconsolata.bff")?)?);

    let description_text_id = ui.components.store(Label::new(&mut renderer, font_id)?);
    let description_text = ui.components.get_and_cast_mut::<Label>(description_text_id)?;
    description_text.label_text = DESCRIPTION.to_string();
    description_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
    description_text.offset = Vec2::new(5.0, 0.0);
    description_text.anchor = Vec2::new(0.0, 1.0);
    description_text.label_line_height = 20;
    ui.components.get_mut(ui.main_canvas_id)?.add_child(description_text_id);

    let world_center = window_size / 2.0 / physics.pixels_per_meter;
    physics.bodies.store(Body { id: 0, shape: BodyShape::Circle, position: world_center + Vec2::new(0.0, 1.0), rotation: 0.0, size: Vec2::new(1.0, 1.0), mass: 1.0 })?;
    physics.bodies.store(Body { id: 0, shape: BodyShape::Circle, position: world_center + Vec2::new(-2.0, 1.0), rotation: 0.0, size: Vec2::new(1.0, 1.0), mass: 1.0 })?;
    physics.bodies.store(Body { id: 0, shape: BodyShape::Box, position: Vec2::new(0.5, 0.5), rotation: 0.0, size: Vec2::new(1.0, 1.0), mass: 1.0 })?;
    physics.bodies.store(Body { id: 0, shape: BodyShape::Circle, position: world_center + Vec2::new(2.0, 1.0), rotation: 0.0, size: Vec2::new(1.0, 1.0), mass: 1.0 })?;
    physics.bodies.store(Body {
        id: 0,
        shape: BodyShape::Box,
        position: world_center + Vec2::new(4.0, 1.0),
        rotation: std::f32::consts::PI / 4.0,
        size: Vec2::new(1.0, 1.0),
        mass: 1.0,
    })?;
    physics.bodies.store(Body {
        id: 0,
        shape: BodyShape::Circle,
        position: world_center + Vec2::new(-3.0, -1.0),
        rotation: 0.0,
        size: Vec2::new(2.0, 2.0),
        mass: 10.0,
    })?;
    physics.bodies.store(Body {
        id: 0,
        shape: BodyShape::Circle,
        position: world_center + Vec2::new(3.0, -1.0),
        rotation: 0.0,
        size: Vec2::new(2.0, 2.0),
        mass: 10.0,
    })?;

    let mut is_running = true;
    let mut dt_timestamp = SystemTime::now();
    while is_running {
        while let Some(event) = window.poll_event() {
            match event {
                InputEvent::MouseMoved(cursor_position, _) => {
                    physics.bodies.get_mut(0)?.position = cursor_position / physics.pixels_per_meter;
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

        let delta_time = dt_timestamp.elapsed().unwrap().as_secs_f32();
        dt_timestamp = SystemTime::now();

        physics.update(delta_time)?;
        ui.update(&mut renderer)?;

        renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        physics.draw(&mut renderer)?;
        ui.draw(&mut renderer, description_text_id)?;
        window.swap_buffers();
    }

    Ok(())
}
