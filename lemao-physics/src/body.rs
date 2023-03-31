use lemao_math::vec2::Vec2;

#[derive(Debug, Default)]
pub struct Body {
    pub id: usize,
    pub position: Vec2,
    pub size: Vec2,
    pub mass: f32,
}
