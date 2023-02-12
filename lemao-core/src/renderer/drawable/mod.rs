use super::batcher::Batch;
use super::shaders::Shader;
use lemao_math::color::SolidColor;
use lemao_math::gradient::Gradient;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use std::any::Any;

pub mod circle;
pub mod disc;
pub mod frame;
pub mod line;
pub mod rectangle;
pub mod storage;
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

pub trait Drawable {
    fn get_position(&self) -> Vec2;
    fn set_position(&mut self, position: Vec2);
    fn move_delta(&mut self, delta: Vec2);

    fn get_scale(&self) -> Vec2;
    fn set_scale(&mut self, scale: Vec2);

    fn get_rotation(&self) -> f32;
    fn set_rotation(&mut self, rotation: f32);
    fn rotate(&mut self, delta: f32);

    fn get_size(&self) -> Vec2;
    fn set_size(&mut self, size: Vec2);

    fn get_anchor(&self) -> Vec2;
    fn set_anchor(&mut self, anchor: Vec2);

    fn get_color(&self) -> &Color;
    fn set_color(&mut self, color: Color);

    fn get_transformation_matrix(&self) -> Mat4x4;
    fn get_batch(&self) -> Batch;

    fn draw(&self, shader: &Shader) -> Result<(), String>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
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
