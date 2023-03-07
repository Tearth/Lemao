use crate::events::UiEvent;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::circle::Circle;
use lemao_core::renderer::drawable::disc::Disc;
use lemao_core::renderer::drawable::frame::{Frame, FrameThickness};
use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_core::renderer::drawable::CornerRounding;
use std::any::Any;

pub mod button;
pub mod canvas;
pub mod checkbox;
pub mod label;
pub mod panel;
pub mod progressbar;
pub mod scrollbox;
pub mod slider;
pub mod textbox;
pub mod wire;

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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ComponentShape {
    Rectangle,
    Disc,
}

pub enum ComponentFillingShape {
    Rectangle(Rectangle),
    Disc(Disc),
}

pub enum ComponentBorderShape {
    Frame(Frame),
    Circle(Circle),
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

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct ComponentCornerRounding {
    pub left_bottom: f32,
    pub right_bottom: f32,
    pub right_top: f32,
    pub left_top: f32,
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

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct EventMask {
    pub position: Vec2,
    pub size: Vec2,
}

pub trait Component {
    fn get_position(&self) -> ComponentPosition;
    fn get_work_area_position(&self) -> Vec2;
    fn set_position(&mut self, position: ComponentPosition);

    fn get_size(&self) -> ComponentSize;
    fn get_work_area_size(&self) -> Vec2;
    fn set_size(&mut self, size: ComponentSize);

    fn get_min_size(&self) -> Vec2;
    fn set_min_size(&mut self, min_size: Vec2);
    fn get_max_size(&self) -> Vec2;
    fn set_max_size(&mut self, max_size: Vec2);

    fn get_anchor(&self) -> Vec2;
    fn set_anchor(&mut self, anchor: Vec2);

    fn get_margin(&self) -> ComponentMargin;
    fn set_margin(&mut self, margin: ComponentMargin);

    fn get_offset(&self) -> Vec2;
    fn set_offset(&mut self, offset: Vec2);

    fn get_scroll_offset(&self) -> Vec2;
    fn set_scroll_offset(&mut self, scroll_offset: Vec2);

    fn add_child(&mut self, component_id: usize);
    fn remove_child(&mut self, component_id: usize);
    fn get_children(&self) -> &Vec<usize>;

    fn process_window_event(&mut self, event: &InputEvent) -> Vec<UiEvent>;
    fn update(&mut self, renderer: &mut RendererContext, area_position: Vec2, area_size: Vec2) -> Result<(), String>;
    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String>;

    fn is_active(&self) -> bool;
    fn set_active_flag(&mut self, active: bool);

    fn is_dirty(&self) -> bool;
    fn set_dirty_flag(&mut self, dirty: bool);

    fn get_event_mask(&self) -> Option<EventMask>;
    fn set_event_mask(&mut self, event_mask: Option<EventMask>);

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

impl ComponentCornerRounding {
    pub fn new(left_bottom: f32, right_bottom: f32, right_top: f32, left_top: f32) -> Self {
        Self { left_bottom, right_bottom, right_top, left_top }
    }
}

impl From<ComponentCornerRounding> for CornerRounding {
    fn from(corner_rounding: ComponentCornerRounding) -> Self {
        Self {
            left_bottom: corner_rounding.left_bottom,
            right_bottom: corner_rounding.right_bottom,
            right_top: corner_rounding.right_top,
            left_top: corner_rounding.left_top,
        }
    }
}

impl EventMask {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }
}
