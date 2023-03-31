use crate::storage::PhysicsStorage;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::circle::Circle;
use lemao_core::renderer::drawable::Color;
use lemao_math::color::SolidColor;
use lemao_math::vec2::Vec2;

pub struct PhysicsContext {
    pub debug_circle: Circle,
    pub pixels_per_meter: f32,
    pub bodies: PhysicsStorage,
}

impl PhysicsContext {
    pub fn new(renderer: &mut RendererContext) -> Result<Self, String> {
        let mut physics = Self { debug_circle: renderer.create_circle()?, pixels_per_meter: 100.0, bodies: Default::default() };

        Ok(physics)
    }

    pub fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        for body in self.bodies.iter() {
            self.debug_circle.position = body.position * self.pixels_per_meter;
            self.debug_circle.size = body.size * self.pixels_per_meter;
            self.debug_circle.anchor = Vec2::new(0.5, 0.5);
            self.debug_circle.color = Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0));
            self.debug_circle.update();
            renderer.draw(&mut self.debug_circle)?;
        }

        Ok(())
    }

    pub fn update(&mut self, delta_time: f32) -> Result<(), String> {
        for i in 0..self.bodies.len() {
            for j in 0..self.bodies.len() {
                if i == j {
                    continue;
                }

                let b1 = self.bodies.get(i)?;
                let b2 = self.bodies.get(j)?;

                let r1 = b1.size.x / 2.0;
                let r2 = b2.size.y / 2.0;
                let distance = b1.position.distance(b2.position);
                let depth = distance - (r1 + r2);

                if depth < 0.0 {
                    let direction = b2.position - b1.position;
                    let change = direction * depth;
                    let b1_mass_ratio = 1.0 - b1.mass / (b1.mass + b2.mass);
                    let b2_mass_ratio = 1.0 - b2.mass / (b1.mass + b2.mass);

                    let b1_position = b1.position + change * b1_mass_ratio;
                    let b2_position = b2.position - change * b2_mass_ratio;

                    self.bodies.get_mut(i)?.position = b1_position;
                    self.bodies.get_mut(j)?.position = b2_position;
                }
            }
        }

        Ok(())
    }
}
