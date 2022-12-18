use super::Component;
use super::ComponentPosition;
use super::ComponentSize;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use std::any::Any;

pub struct Panel {
    pub(crate) id: usize,

    position: ComponentPosition,
    size: ComponentSize,
    anchor: Vec2,
    rectangle_id: usize,
}

impl Panel {
    pub fn new(id: usize, renderer: &mut RendererContext) -> Result<Self, String> {
        Ok(Self {
            id,
            position: ComponentPosition::Absolute(Default::default()),
            size: ComponentSize::Absolute(Default::default()),
            anchor: Default::default(),
            rectangle_id: renderer.create_rectangle(Vec2::new(100.0, 100.0))?,
        })
    }
}

impl Component for Panel {
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

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        renderer.draw(self.rectangle_id)?;
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
