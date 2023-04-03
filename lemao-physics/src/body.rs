use lemao_math::vec2::Vec2;

#[derive(Debug)]
pub struct Body {
    pub id: usize,
    pub shape: BodyShape,
    pub position: Vec2,
    pub rotation: f32,
    pub size: Vec2,
    pub mass: f32,
    pub velocity_linear: Vec2,
    pub dynamic: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BodyShape {
    Box,
    Circle,
}

impl Body {
    pub fn new(shape: BodyShape, position: Vec2, rotation: f32, size: Vec2, mass: f32, dynamic: bool) -> Self {
        Self { id: 0, shape, position, rotation, size, mass, velocity_linear: Default::default(), dynamic }
    }
}
