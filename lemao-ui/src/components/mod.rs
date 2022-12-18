use lemao_core::renderer::context::RendererContext;
use std::any::Any;

pub mod panel;

pub trait Component {
    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String>;

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
