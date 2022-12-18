use super::Component;
use super::ComponentPosition;
use super::ComponentSize;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use std::any::Any;

pub struct Canvas {
    pub(crate) id: usize,

    position: ComponentPosition,
    size: ComponentSize,
    anchor: Vec2,
}

impl Canvas {
    pub fn new(id: usize) -> Result<Self, String> {
        Ok(Self {
            id,
            position: ComponentPosition::Absolute(Default::default()),
            size: ComponentSize::Absolute(Default::default()),
            anchor: Default::default(),
        })
    }
}

impl Component for Canvas {
    fn get_position(&self) -> ComponentPosition {
        self.position
    }

    fn set_position(&mut self, position: ComponentPosition) {
        self.position = position;
    }

    fn get_size(&self) -> ComponentSize {
        self.size
    }

    fn set_size(&mut self, size: ComponentSize) {
        self.size = size;
    }

    fn get_anchor(&self) -> Vec2 {
        self.anchor
    }

    fn set_anchor(&mut self, anchor: Vec2) {
        self.anchor = anchor;
    }

    fn draw(&mut self, _renderer: &mut RendererContext) -> Result<(), String> {
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
