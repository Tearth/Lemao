use crate::storage::PhysicsStorage;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::circle::Circle;
use lemao_core::renderer::drawable::Color;
use lemao_math::color::SolidColor;
use lemao_math::vec2::Vec2;

pub struct PhysicsContext {
    pub debug_circle: Circle,
    pub bodies: PhysicsStorage,
}

impl PhysicsContext {
    pub fn new(renderer: &mut RendererContext) -> Result<Self, String> {
        let mut physics = Self { debug_circle: renderer.create_circle()?, bodies: Default::default() };

        Ok(physics)
    }

    pub fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        for body in self.bodies.iter() {
            self.debug_circle.position = body.position;
            self.debug_circle.size = body.size;
            self.debug_circle.anchor = Vec2::new(0.5, 0.5);
            self.debug_circle.color = Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0));
            self.debug_circle.update();
            renderer.draw(&mut self.debug_circle)?;
        }

        Ok(())
    }

    pub fn update(&mut self, delta_time: f32) -> Result<(), String> {
        Ok(())
    }
}
