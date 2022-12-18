use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use std::any::Any;

pub mod canvas;
pub mod panel;

#[derive(Copy, Clone, Debug)]
pub enum ComponentPosition {
    Absolute(Vec2),
}

#[derive(Copy, Clone, Debug)]
pub enum ComponentSize {
    Absolute(Vec2),
}

pub trait Component {
    fn get_position(&self) -> ComponentPosition;
    fn set_position(&mut self, position: ComponentPosition);

    fn get_size(&self) -> ComponentSize;
    fn set_size(&mut self, size: ComponentSize);

    fn get_anchor(&self) -> Vec2;
    fn set_anchor(&mut self, anchor: Vec2);

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
