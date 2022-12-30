use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::frame::FrameThickness;
use lemao_core::renderer::drawable::Color;
use std::any::Any;

pub mod button;
pub mod canvas;
pub mod image;
pub mod label;
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

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ComponentBorderThickness {
    pub top: f32,
    pub bottom: f32,
    pub right: f32,
    pub left: f32,
}

#[derive(Copy, Clone, Debug)]
pub enum HorizontalAlignment {
    Left,
    Middle,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub enum VerticalAlignment {
    Top,
    Middle,
    Bottom,
}

pub trait Component {
    fn get_position(&self) -> ComponentPosition;
    fn get_work_area_position(&self) -> Vec2;
    fn set_position(&mut self, position: ComponentPosition);

    fn get_size(&self) -> ComponentSize;
    fn get_work_area_size(&self) -> Vec2;
    fn set_size(&mut self, size: ComponentSize);

    fn get_anchor(&self) -> Vec2;
    fn set_anchor(&mut self, anchor: Vec2);

    fn get_margin(&self) -> ComponentMargin;
    fn set_margin(&mut self, margin: ComponentMargin);

    fn get_offset(&self) -> Vec2;
    fn set_offset(&mut self, offset: Vec2);

    fn get_color(&self) -> &Color;
    fn set_color(&mut self, color: Color);

    fn add_child(&mut self, component_id: usize);
    fn remove_child(&mut self, component_id: usize);
    fn get_children(&self) -> &Vec<usize>;

    fn update(&mut self, renderer: &mut RendererContext, area_position: Vec2, area_size: Vec2) -> Result<(), String>;
    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String>;

    fn is_point_inside(&self, point: Vec2) -> bool {
        let component_position = self.get_work_area_position();
        let component_size = self.get_work_area_size();

        let x1 = component_position.x;
        let y1 = component_position.y;
        let x2 = component_position.x + component_size.x;
        let y2 = component_position.y + component_size.y;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl ComponentMargin {
    pub fn new(top: f32, bottom: f32, right: f32, left: f32) -> Self {
        Self { top, bottom, right, left }
    }
}

impl ComponentBorderThickness {
    pub fn new(top: f32, bottom: f32, right: f32, left: f32) -> Self {
        Self { top, bottom, right, left }
    }

    pub fn is_axially_uniform(&self) -> bool {
        self.top == self.bottom && self.left == self.right
    }
}

impl From<ComponentBorderThickness> for FrameThickness {
    fn from(thickness: ComponentBorderThickness) -> Self {
        Self { top: thickness.top, bottom: thickness.bottom, right: thickness.right, left: thickness.left }
    }
}
