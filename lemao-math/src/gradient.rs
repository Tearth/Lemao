use crate::color::SolidColor;
use crate::vec2::Vec2;

#[derive(Clone, PartialEq)]
pub struct Gradient {
    pub r#type: GradientType,
    pub offset: Vec2,
    pub steps: Vec<GradientStep>,
}

#[derive(Copy, Clone, PartialEq)]
pub enum GradientType {
    Horizontal,
    Vertical,
    Radial,
    Rectangular,
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct GradientStep {
    pub color: SolidColor,
    pub step: f32,
}

impl Gradient {
    pub fn new(r#type: GradientType, offset: Vec2) -> Self {
        Self { r#type, offset, steps: Default::default() }
    }
}

impl GradientStep {
    pub fn new(color: SolidColor, step: f32) -> Self {
        Self { color, step }
    }
}
