use crate::components::canvas::Canvas;
use crate::components::panel::Panel;
use crate::components::Component;
use crate::components::ComponentPosition;
use crate::components::ComponentSize;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;

pub struct UiContext {
    main_canvas_id: usize,
    components: Vec<Option<Box<dyn Component>>>,
}

impl UiContext {
    pub fn new(renderer: &mut RendererContext) -> Result<Self, String> {
        let mut ui = Self { main_canvas_id: 0, components: Default::default() };
        ui.main_canvas_id = ui.create_canvas(renderer)?;

        let main_canvas = ui.get_component_mut(ui.main_canvas_id)?;
        main_canvas.set_size(ComponentSize::Absolute(renderer.get_viewport_size()));

        Ok(ui)
    }

    pub fn process_event(&mut self, event: &InputEvent) {
        match event {
            InputEvent::WindowSizeChanged(size) => {
                let main_canvas = self.get_component_mut(self.main_canvas_id).unwrap();
                main_canvas.set_size(ComponentSize::Absolute(*size));
            }
            _ => {}
        }
    }

    pub fn create_canvas(&mut self, renderer: &mut RendererContext) -> Result<usize, String> {
        let id = self.components.len();
        let canvas = Box::new(Canvas::new(id)?);
        self.components.push(Some(canvas));

        Ok(id)
    }

    pub fn create_panel(&mut self, renderer: &mut RendererContext) -> Result<usize, String> {
        let id = self.components.len();
        let panel = Box::new(Panel::new(id, renderer)?);
        self.components.push(Some(panel));

        Ok(id)
    }

    pub fn get_component(&self, component_id: usize) -> Result<&dyn Component, String> {
        if component_id >= self.components.len() {
            return Err(format!("Component with id {} not found", component_id));
        }

        match &self.components[component_id] {
            Some(component) => Ok(component.as_ref()),
            None => return Err(format!("Component with id {} not found", component_id)),
        }
    }

    pub fn get_component_with_type<T: 'static>(&self, component_id: usize) -> Result<&T, String> {
        self.get_component(component_id)?.as_any().downcast_ref::<T>().ok_or(format!("Component with id {} cannot be downcasted", component_id))
    }

    pub fn get_component_mut(&mut self, component_id: usize) -> Result<&mut dyn Component, String> {
        if component_id >= self.components.len() {
            return Err(format!("Component with id {} not found", component_id));
        }

        match &mut self.components[component_id] {
            Some(drawable) => Ok(drawable.as_mut()),
            None => return Err(format!("Component with id {} not found", component_id)),
        }
    }

    pub fn get_component_with_type_mut<T: 'static>(&mut self, component_id: usize) -> Result<&mut T, String> {
        self.get_component_mut(component_id)?.as_any_mut().downcast_mut::<T>().ok_or(format!("Component with id {} cannot be downcasted", component_id))
    }

    pub fn get_main_canvas(&self) -> Result<&dyn Component, String> {
        self.get_component(self.main_canvas_id)
    }

    pub fn get_main_canvas_mut(&mut self) -> Result<&mut dyn Component, String> {
        self.get_component_mut(self.main_canvas_id)
    }

    pub fn draw(&mut self, renderer: &mut RendererContext, component_id: usize) -> Result<(), String> {
        let main_canvas = self.get_main_canvas()?;
        let area_position = match main_canvas.get_position() {
            ComponentPosition::AbsoluteToParent(position) => position,
            _ => return Err("Invalid canvas".to_string()),
        };
        let area_size = match main_canvas.get_size() {
            ComponentSize::Absolute(size) => size,
            _ => return Err("Invalid canvas".to_string()),
        };
        self.update(renderer, self.main_canvas_id, area_position, area_size)?;

        let component = self.get_component_mut(component_id)?;
        component.draw(renderer)?;

        Ok(())
    }

    fn update(&mut self, renderer: &mut RendererContext, component_id: usize, area_position: Vec2, area_size: Vec2) -> Result<(), String> {
        let component = self.get_component_mut(component_id)?;
        component.update(renderer, area_position, area_size)?;

        let component_area_position = component.get_screen_position() - (component.get_screen_size() * component.get_anchor());
        let component_area_size = component.get_screen_size();

        for child_id in component.get_children().clone() {
            self.update(renderer, child_id, component_area_position, component_area_size)?;
        }

        Ok(())
    }
}
