use lemao_math::vec2::Vec2;

#[derive(Debug)]
pub struct Body {
    pub id: usize,
    pub position: Vec2,
    pub rotation: f32,
    pub size: Vec2,
    pub mass: f32,

    pub shape: BodyShape,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BodyShape {
    Box,
    Circle,
}
