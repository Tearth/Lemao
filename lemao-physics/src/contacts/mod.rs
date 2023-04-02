use lemao_math::vec2::Vec2;

pub mod box2box;
pub mod box2circle;
pub mod circle2circle;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Contact {
    pub position: Vec2,
}

impl Contact {
    pub fn new(position: Vec2) -> Self {
        Self { position }
    }
}
