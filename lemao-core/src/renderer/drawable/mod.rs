use super::shaders::Shader;
use lemao_math::vec2::Vec2;

pub mod sprite;
pub mod storage;

pub trait Drawable {
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);

    fn draw(&self, shader: &Shader);

    fn get_position(&self) -> Vec2<f32>;
    fn set_position(&mut self, position: Vec2<f32>);
    fn move_toward(&mut self, delta: Vec2<f32>);

    fn get_scale(&self) -> Vec2<f32>;
    fn set_scale(&mut self, scale: Vec2<f32>);

    fn get_rotation(&self) -> f32;
    fn set_rotation(&mut self, rotation: f32);
}
