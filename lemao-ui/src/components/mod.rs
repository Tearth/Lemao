use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use std::any::Any;

pub mod canvas;
pub mod panel;

#[derive(Copy, Clone, Debug)]
pub enum ComponentPosition {
    AbsoluteToParent(Vec2),
    RelativeToParent(Vec2),
}

#[derive(Copy, Clone, Debug)]
pub enum ComponentSize {
    Absolute(Vec2),
    Relative(Vec2),
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ComponentMargin {
    pub top: f32,
    pub bottom: f32,
    pub right: f32,
    pub left: f32,
}

pub trait Component {
    fn get_position(&self) -> ComponentPosition;
    fn get_screen_position(&self) -> Vec2;
    fn set_position(&mut self, position: ComponentPosition);

    fn get_size(&self) -> ComponentSize;
    fn get_screen_size(&self) -> Vec2;
    fn set_size(&mut self, size: ComponentSize);

    fn get_anchor(&self) -> Vec2;
    fn set_anchor(&mut self, anchor: Vec2);

    fn get_margin(&self) -> ComponentMargin;
    fn set_margin(&mut self, margin: ComponentMargin);

    fn add_child(&mut self, component_id: usize);
    fn remove_child(&mut self, component_id: usize);
    fn get_children(&self) -> &Vec<usize>;

    fn update(&mut self, renderer: &mut RendererContext, area_position: Vec2, area_size: Vec2) -> Result<(), String>;
    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl ComponentMargin {
    pub fn new(top: f32, bottom: f32, right: f32, left: f32) -> Self {
        ComponentMargin { top, bottom, right, left }
    }
}
