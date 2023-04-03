use lemao_math::vec2::Vec2;

use crate::{body::Body, constants::*};

pub mod box2box;
pub mod box2circle;
pub mod circle2circle;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Collision {
    pub depth: f32,
    pub direction: Vec2,
}

impl Collision {
    pub fn new(depth: f32, direction: Vec2) -> Self {
        Self { depth, direction }
    }
}

pub fn separate(body1: &mut Body, body2: &mut Body, collision: &Collision) {
    let change = collision.direction * collision.depth.max(MIN_SEPARATION);
    let (b1_mass_ratio, b2_mass_ratio) = if body1.dynamic && body2.dynamic {
        (1.0 - body1.mass / (body1.mass + body2.mass), 1.0 - body2.mass / (body1.mass + body2.mass))
    } else if !body1.dynamic {
        (0.0, 1.0)
    } else {
        (1.0, 0.0)
    };

    let b1_position = body1.position - change * b1_mass_ratio;
    let b2_position = body2.position + change * b2_mass_ratio;

    body1.position = b1_position;
    body2.position = b2_position;
}
