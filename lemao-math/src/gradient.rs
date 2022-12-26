use crate::color::SolidColor;

#[derive(Clone, PartialEq)]
pub struct Gradient {
    pub r#type: GradientType,
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
    pub fn new(r#type: GradientType) -> Self {
        Self { r#type, steps: Default::default() }
    }
}

impl GradientStep {
    pub fn new(color: SolidColor, step: f32) -> Self {
        Self { color, step }
    }
}
