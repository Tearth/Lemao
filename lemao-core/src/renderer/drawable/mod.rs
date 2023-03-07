use super::batcher::Batch;
use super::shaders::Shader;
use lemao_math::color::SolidColor;
use lemao_math::gradient::Gradient;
use lemao_math::mat4x4::Mat4x4;

pub mod circle;
pub mod disc;
pub mod frame;
pub mod line;
pub mod rectangle;
pub mod text;
pub mod tilemap;

#[derive(Clone, PartialEq)]
pub enum Color {
    SolidColor(SolidColor),
    Gradient(Gradient),
}

#[derive(Copy, Clone, Default, PartialEq)]
pub struct CornerRounding {
    pub left_bottom: f32,
    pub right_bottom: f32,
    pub right_top: f32,
    pub left_top: f32,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DrawableEnum {
    Circle,
    Disc,
    Frame,
    Line,
    Rectangle,
    Text,
    Tilemap,
}

pub trait Drawable {
    fn get_transformation_matrix(&self) -> Mat4x4;
    fn get_batch(&self) -> Batch;
    fn get_color(&self) -> &Color;
    fn draw(&mut self, shader: &Shader) -> Result<(), String>;
}

impl Color {
    pub fn set_alpha(self, alpha: f32) -> Color {
        match self {
            Self::SolidColor(solid) => Color::SolidColor(SolidColor::new(solid.r, solid.g, solid.b, alpha)),
            Self::Gradient(gradient) => {
                let mut gradient = gradient;
                for step in &mut gradient.steps {
                    step.color = SolidColor::new(step.color.r, step.color.g, step.color.b, alpha);
                }

                Color::Gradient(gradient)
            }
        }
    }
}

impl CornerRounding {
    pub fn new(left_bottom: f32, right_bottom: f32, right_top: f32, left_top: f32) -> Self {
        Self { left_bottom, right_bottom, right_top, left_top }
    }
}
