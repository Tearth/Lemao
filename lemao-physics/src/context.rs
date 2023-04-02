use crate::body::BodyShape;
use crate::collisions;
use crate::storage::PhysicsStorage;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::circle::Circle;
use lemao_core::renderer::drawable::frame::Frame;
use lemao_core::renderer::drawable::line::Line;
use lemao_core::renderer::drawable::Color;
use lemao_math::color::SolidColor;
use lemao_math::vec2::Vec2;
use std::f32::consts;

pub struct PhysicsContext {
    pub debug_box: Frame,
    pub debug_circle: Circle,
    pub debug_line: Line,
    pub pixels_per_meter: f32,
    pub bodies: PhysicsStorage,
}

impl PhysicsContext {
    pub fn new(renderer: &mut RendererContext) -> Result<Self, String> {
        let physics = Self {
            debug_circle: renderer.create_circle()?,
            debug_box: renderer.create_frame()?,
            debug_line: renderer.create_line()?,
            pixels_per_meter: 100.0,
            bodies: Default::default(),
        };

        Ok(physics)
    }

    pub fn update(&mut self, delta_time: f32) -> Result<(), String> {
        for body1_id in 0..self.bodies.len() {
            for body2_id in 0..self.bodies.len() {
                if body1_id == body2_id {
                    continue;
                }

                let body1 = self.bodies.get(body1_id)?;
                let body2 = self.bodies.get(body2_id)?;

                let (collision, invert) = if body1.shape == BodyShape::Box && body2.shape == BodyShape::Box {
                    (collisions::box2box::process(body1, body2), false)
                } else if body1.shape == BodyShape::Circle && body2.shape == BodyShape::Circle {
                    (collisions::circle2circle::process(body1, body2), false)
                } else if body1.shape == BodyShape::Box && body2.shape == BodyShape::Circle {
                    (collisions::box2circle::process(body1, body2), false)
                } else if body1.shape == BodyShape::Circle && body2.shape == BodyShape::Box {
                    (collisions::box2circle::process(body2, body1), true)
                } else {
                    (None, false)
                };

                if let Some(collision) = collision {
                    let (body1, body2) = self.bodies.get_mut_2(body1_id, body2_id)?;

                    // Separator assumes that the collision normal goes from body1 to body2
                    let (body1, body2) = if invert { (body2, body1) } else { (body1, body2) };

                    collisions::separate(body1, body2, &collision);
                }
            }
        }

        Ok(())
    }

    pub fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        for body in self.bodies.iter() {
            match body.shape {
                BodyShape::Box => {
                    self.debug_box.position = body.position * self.pixels_per_meter;
                    self.debug_box.rotation = body.rotation;
                    self.debug_box.size = body.size * self.pixels_per_meter;
                    self.debug_box.anchor = Vec2::new(0.5, 0.5);
                    self.debug_box.color = Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0));
                    self.debug_box.update();

                    self.debug_line.from = body.position * self.pixels_per_meter;
                    self.debug_line.to = (body.position + (Vec2::new_from_angle(body.rotation) * body.size.y / 2.0)) * self.pixels_per_meter;
                    self.debug_line.color = Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0));
                    self.debug_line.update();

                    renderer.draw(&mut self.debug_box)?;
                    renderer.draw(&mut self.debug_line)?;
                }
                BodyShape::Circle => {
                    self.debug_circle.position = body.position * self.pixels_per_meter;
                    self.debug_circle.rotation = body.rotation;
                    self.debug_circle.size = body.size * self.pixels_per_meter;
                    self.debug_circle.anchor = Vec2::new(0.5, 0.5);
                    self.debug_circle.color = Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0));
                    self.debug_circle.update();

                    self.debug_line.from = body.position * self.pixels_per_meter;
                    self.debug_line.to = (body.position + (Vec2::new_from_angle(body.rotation) * body.size.y / 2.0)) * self.pixels_per_meter;
                    self.debug_line.color = Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0));
                    self.debug_line.update();

                    renderer.draw(&mut self.debug_circle)?;
                    renderer.draw(&mut self.debug_line)?;
                }
            }
        }

        Ok(())
    }
}
