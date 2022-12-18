use super::Component;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use std::any::Any;

pub struct Panel {
    pub(crate) id: usize,

    rectangle_id: usize,
}

impl Panel {
    pub fn new(id: usize, renderer: &mut RendererContext) -> Result<Self, String> {
        Ok(Self { id, rectangle_id: renderer.create_rectangle(Vec2::new(100.0, 100.0))? })
    }
}

impl Component for Panel {
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
