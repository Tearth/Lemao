use crate::components::button::Button;
use crate::components::canvas::Canvas;
use crate::components::label::Label;
use crate::components::panel::Panel;
use crate::components::Component;
use crate::components::ComponentPosition;
use crate::components::ComponentShape;
use crate::components::ComponentSize;
use crate::events::UiEvent;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use std::collections::VecDeque;

pub struct UiContext {
    ui_camera_id: usize,
    main_canvas_id: usize,
    last_cursor_position: Vec2,
    components: Vec<Option<Box<dyn Component>>>,
    events: VecDeque<UiEvent>,
}

impl UiContext {
    pub fn new(renderer: &mut RendererContext) -> Result<Self, String> {
        let main_camera = renderer.get_active_camera()?;
        let ui_camera_id = renderer.create_camera(main_camera.get_position(), main_camera.get_size())?;

        let mut ui =
            Self { main_canvas_id: 0, last_cursor_position: Default::default(), ui_camera_id, components: Default::default(), events: Default::default() };
        ui.main_canvas_id = ui.create_canvas(renderer)?;

        let main_canvas = ui.get_component_mut(ui.main_canvas_id)?;
        main_canvas.set_size(ComponentSize::Absolute(renderer.get_viewport_size()));

        Ok(ui)
    }

    pub fn process_window_event(&mut self, renderer: &mut RendererContext, event: &InputEvent) -> Result<(), String> {
        match event {
            InputEvent::WindowSizeChanged(size) => {
                let ui_camera = renderer.get_camera_mut(self.ui_camera_id)?;
                let main_canvas = self.get_component_mut(self.main_canvas_id)?;

                ui_camera.set_size(*size);
                main_canvas.set_size(ComponentSize::Absolute(*size));
            }
            InputEvent::MouseMoved(cursor_position) => {
                for (component_id, component) in self.components.iter().enumerate() {
                    if component_id == self.main_canvas_id {
                        continue;
                    }

                    if component.is_none() {
                        continue;
                    }

                    let component = component.as_ref().unwrap();

                    if component.is_point_inside(*cursor_position) {
                        if !component.is_point_inside(self.last_cursor_position) {
                            self.events.push_back(UiEvent::CursorEnter(component_id, *cursor_position));
                        }

                        self.events.push_back(UiEvent::CursorOver(component_id, *cursor_position));
                    } else {
                        if component.is_point_inside(self.last_cursor_position) {
                            self.events.push_back(UiEvent::CursorLeave(component_id, *cursor_position));
                        }
                    }
                }

                self.last_cursor_position = *cursor_position;
            }
            InputEvent::MouseButtonPressed(button) => {
                for (component_id, component) in self.components.iter().enumerate() {
                    if component_id == self.main_canvas_id {
                        continue;
                    }

                    if component.is_none() {
                        continue;
                    }

                    let component = component.as_ref().unwrap();

                    if component.is_point_inside(self.last_cursor_position) {
                        self.events.push_back(UiEvent::MouseButtonPressed(component_id, *button));
                    }
                }
            }
            InputEvent::MouseButtonReleased(button) => {
                for (component_id, component) in self.components.iter().enumerate() {
                    if component_id == self.main_canvas_id {
                        continue;
                    }

                    if component.is_none() {
                        continue;
                    }

                    let component = component.as_ref().unwrap();

                    if component.is_point_inside(self.last_cursor_position) {
                        self.events.push_back(UiEvent::MouseButtonReleased(component_id, *button));
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn poll_event(&mut self) -> Option<UiEvent> {
        self.events.pop_front()
    }

    pub fn create_button(&mut self, renderer: &mut RendererContext, shape: ComponentShape, label_font_id: usize) -> Result<usize, String> {
        let id = self.components.len();
        let button = Box::new(Button::new(id, renderer, shape, label_font_id)?);
        self.components.push(Some(button));

        Ok(id)
    }

    pub fn create_canvas(&mut self, renderer: &mut RendererContext) -> Result<usize, String> {
        let id = self.components.len();
        let canvas = Box::new(Canvas::new(id)?);
        self.components.push(Some(canvas));

        Ok(id)
    }

    pub fn create_label(&mut self, renderer: &mut RendererContext, label_font_id: usize) -> Result<usize, String> {
        let id = self.components.len();
        let label = Box::new(Label::new(id, renderer, label_font_id)?);
        self.components.push(Some(label));

        Ok(id)
    }

    pub fn create_panel(&mut self, renderer: &mut RendererContext, shape: ComponentShape) -> Result<usize, String> {
        let id = self.components.len();
        let panel = Box::new(Panel::new(id, renderer, shape)?);
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
        let active_camera_id = renderer.get_active_camera()?.get_id();
        renderer.set_camera_as_active(self.ui_camera_id)?;

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

        renderer.set_camera_as_active(active_camera_id)?;
        Ok(())
    }

    fn update(&mut self, renderer: &mut RendererContext, component_id: usize, area_position: Vec2, area_size: Vec2) -> Result<(), String> {
        let component = self.get_component_mut(component_id)?;
        component.update(renderer, area_position, area_size)?;

        let component_area_position = component.get_work_area_position();
        let component_area_size = component.get_work_area_size();

        for child_id in component.get_children().clone() {
            self.update(renderer, child_id, component_area_position, component_area_size)?;
        }

        Ok(())
    }
}
