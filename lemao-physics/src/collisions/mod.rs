use lemao_math::vec2::Vec2;

use crate::body::Body;

pub mod box2box;
pub mod box2circle;
pub mod circle2circle;

#[derive(Copy, Clone, Debug, Default)]
pub struct Collision {
    pub depth: f32,
    pub normal: Vec2,
}

impl Collision {
    pub fn new(depth: f32, normal: Vec2) -> Self {
        Self { depth, normal }
    }
}

pub fn separate(body1: &mut Body, body2: &mut Body, collision: &Collision) {
    let change = collision.normal * collision.depth;
    let b1_mass_ratio = 1.0 - body1.mass / (body1.mass + body2.mass);
    let b2_mass_ratio = 1.0 - body2.mass / (body1.mass + body2.mass);

    let b1_position = body1.position - change * b1_mass_ratio;
    let b2_position = body2.position + change * b2_mass_ratio;

    body1.position = b1_position;
    body2.position = b2_position;
}
