use super::batcher::Batch;
use super::shaders::Shader;
use lemao_math::color::SolidColor;
use lemao_math::gradient::Gradient;
use lemao_math::mat4x4::Mat4x4;
use lemao_math::vec2::Vec2;
use std::any::Any;

pub mod animation;
pub mod circle;
pub mod disc;
pub mod frame;
pub mod line;
pub mod rectangle;
pub mod sprite;
pub mod storage;
pub mod text;

#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    SolidColor(SolidColor),
    Gradient(Gradient),
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

    fn get_anchor(&self) -> Vec2;
    fn set_anchor(&mut self, anchor: Vec2);

    fn get_color(&self) -> Color;
    fn set_color(&mut self, color: Color);

    fn get_transformation_matrix(&self) -> Mat4x4;
    fn get_batch(&self) -> Batch;

    fn draw(&self, shader: &Shader) -> Result<(), String>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
