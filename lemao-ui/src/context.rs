use crate::components::button::Button;
use crate::components::canvas::Canvas;
use crate::components::checkbox::Checkbox;
use crate::components::label::Label;
use crate::components::panel::Panel;
use crate::components::scrollbox::Scrollbox;
use crate::components::Component;
use crate::components::ComponentPosition;
use crate::components::ComponentShape;
use crate::components::ComponentSize;
use crate::components::EventMask;
use crate::events::UiEvent;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use std::collections::VecDeque;

pub struct UiContext {
    ui_camera_id: usize,
    main_canvas_id: usize,
    components: Vec<Option<Box<dyn Component>>>,
    events: VecDeque<UiEvent>,
}

impl UiContext {
    pub fn new(renderer: &mut RendererContext) -> Result<Self, String> {
        let main_camera = renderer.get_active_camera()?;
        let ui_camera_id = renderer.create_camera(main_camera.get_position(), main_camera.get_size())?;

        let mut ui = Self { main_canvas_id: 0, ui_camera_id, components: Default::default(), events: Default::default() };
        ui.main_canvas_id = ui.create_canvas(renderer)?;

        let main_canvas = ui.get_component_mut(ui.main_canvas_id)?;
        main_canvas.set_size(ComponentSize::Absolute(renderer.get_viewport_size()));

        Ok(ui)
    }

    pub fn process_window_event(&mut self, renderer: &mut RendererContext, event: &InputEvent) -> Result<(), String> {
        if let InputEvent::WindowSizeChanged(size) = event {
            let ui_camera = renderer.get_camera_mut(self.ui_camera_id)?;
            let main_canvas = self.get_component_mut(self.main_canvas_id)?;

            ui_camera.set_size(*size);
            main_canvas.set_size(ComponentSize::Absolute(*size));

            for component in self.components.iter_mut().flatten() {
                component.set_dirty_flag(true);
            }
        } else {
            for (component_id, component) in self.components.iter_mut().enumerate() {
                if component_id == self.main_canvas_id {
                    continue;
                }

                if component.is_none() {
                    continue;
                }

                self.events.extend(component.as_mut().unwrap().process_window_event(event));
            }
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

    pub fn create_canvas(&mut self, _renderer: &mut RendererContext) -> Result<usize, String> {
        let id = self.components.len();
        let canvas = Box::new(Canvas::new(id)?);
        self.components.push(Some(canvas));

        Ok(id)
    }

    pub fn create_checkbox(
        &mut self,
        renderer: &mut RendererContext,
        label_font_id: usize,
        tick_on_texture_id: usize,
        tick_off_texture_id: usize,
    ) -> Result<usize, String> {
        let id = self.components.len();
        let checkbox = Box::new(Checkbox::new(id, renderer, label_font_id, tick_on_texture_id, tick_off_texture_id)?);
        self.components.push(Some(checkbox));

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

    pub fn create_scrollbox(&mut self, renderer: &mut RendererContext, scroll_shape: ComponentShape) -> Result<usize, String> {
        let id = self.components.len();
        let scrollbox = Box::new(Scrollbox::new(id, renderer, scroll_shape)?);
        self.components.push(Some(scrollbox));

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

    pub fn begin_scrollbox(&self, scrollbox_id: usize, renderer: &RendererContext) -> Result<(), String> {
        let scrollbox = self.get_component(scrollbox_id)?;
        renderer.enable_scissor(scrollbox.get_work_area_position(), scrollbox.get_work_area_size());

        Ok(())
    }

    pub fn end_scrollbox(&self, renderer: &RendererContext) {
        renderer.disable_scissor();
    }

    pub fn update(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        let main_canvas = self.get_main_canvas()?;
        let area_position = match main_canvas.get_position() {
            ComponentPosition::AbsoluteToParent(position) => position,
            _ => return Err("Invalid canvas".to_string()),
        };
        let area_size = match main_canvas.get_size() {
            ComponentSize::Absolute(size) => size,
            _ => return Err("Invalid canvas".to_string()),
        };
        self.update_internal(renderer, self.main_canvas_id, area_position, area_size, None, Default::default(), false)?;

        Ok(())
    }

    fn update_internal(
        &mut self,
        renderer: &mut RendererContext,
        component_id: usize,
        area_position: Vec2,
        area_size: Vec2,
        event_mask: Option<EventMask>,
        scroll_offset: Vec2,
        force: bool,
    ) -> Result<(), String> {
        let component = self.get_component_mut(component_id)?;
        let update_children = component.is_dirty();

        if force {
            component.set_dirty_flag(true);
        }

        component.set_scroll_offset(scroll_offset);
        component.update(renderer, area_position, area_size)?;

        let component_area_position = component.get_work_area_position();
        let component_area_size = component.get_work_area_size();
        let (event_mask, scroll_offset) = if let Ok(scrollbox) = self.get_component_with_type::<Scrollbox>(component_id) {
            (Some(EventMask::new(component_area_position, component_area_size)), scrollbox.get_scroll_delta())
        } else {
            (event_mask, Default::default())
        };

        let component = self.get_component_mut(component_id)?;
        component.set_event_mask(event_mask);

        for child_id in self.get_component_mut(component_id)?.get_children().clone() {
            self.update_internal(renderer, child_id, component_area_position, component_area_size, event_mask, scroll_offset, force || update_children)?;
        }

        // Scrollbox needs to be updated second time, after all children are refreshed
        if self.get_component_with_type::<Scrollbox>(component_id).is_ok() {
            let mut left_bottom_corner: Vec2 = Vec2::new(f32::MAX, f32::MAX);
            let mut right_top_corner: Vec2 = Vec2::new(f32::MIN, f32::MIN);

            for child_id in self.get_component(component_id)?.get_children().clone() {
                let child = self.get_component(child_id)?;
                let child_area_position = child.get_work_area_position();
                let child_area_size = child.get_work_area_size();

                left_bottom_corner.x = f32::min(left_bottom_corner.x, child_area_position.x);
                left_bottom_corner.y = f32::min(left_bottom_corner.y, child_area_position.y);
                right_top_corner.x = f32::max(right_top_corner.x, child_area_position.x + child_area_size.x);
                right_top_corner.y = f32::max(right_top_corner.y, child_area_position.y + child_area_size.y);
            }

            self.get_component_with_type_mut::<Scrollbox>(component_id)?.set_total_size(right_top_corner - left_bottom_corner);
            self.get_component_with_type_mut::<Scrollbox>(component_id)?.update(renderer, area_position, area_size)?;
        }

        Ok(())
    }

    pub fn draw(&mut self, renderer: &mut RendererContext, component_id: usize) -> Result<(), String> {
        let active_camera_id = renderer.get_active_camera()?.get_id();
        renderer.set_camera_as_active(self.ui_camera_id)?;

        let component = self.get_component_mut(component_id)?;
        component.draw(renderer)?;

        renderer.set_camera_as_active(active_camera_id)?;
        Ok(())
    }
}
