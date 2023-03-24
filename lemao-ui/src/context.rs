use crate::components::canvas::Canvas;
use crate::components::scrollbox::Scrollbox;
use crate::components::Component;
use crate::components::ComponentPosition;
use crate::components::ComponentSize;
use crate::components::EventMask;
use crate::events::UiEvent;
use crate::utils::storage::UiStorage;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::cameras::Camera;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::frame::Frame;
use lemao_core::renderer::drawable::Color;
use lemao_math::color::SolidColor;
use std::collections::VecDeque;

pub struct UiContext {
    pub ui_camera_id: usize,
    pub main_canvas_id: usize,
    pub debug_frame: Frame,
    pub debug: bool,

    pub components: UiStorage,
    events: VecDeque<UiEvent>,
}

impl UiContext {
    pub fn new(renderer: &mut RendererContext) -> Result<Self, String> {
        let main_camera = renderer.cameras.get(renderer.active_camera_id)?;
        let ui_camera_id = renderer.cameras.store(Camera::new(main_camera.position, main_camera.size));

        let mut ui = Self {
            main_canvas_id: 0,
            ui_camera_id,
            debug_frame: renderer.create_frame()?,
            debug: false,

            components: Default::default(),
            events: Default::default(),
        };
        ui.main_canvas_id = ui.components.store(Box::new(Canvas::new()?));

        let main_canvas = ui.components.get_mut(ui.main_canvas_id)?;
        main_canvas.set_size(ComponentSize::Absolute(renderer.viewport_size));

        Ok(ui)
    }

    pub fn process_window_event(&mut self, renderer: &mut RendererContext, event: &InputEvent) -> Result<(), String> {
        if let InputEvent::WindowSizeChanged(size) = event {
            let ui_camera = renderer.cameras.get_mut(self.ui_camera_id)?;
            let main_canvas = self.components.get_mut(self.main_canvas_id)?;

            ui_camera.size = *size;
            ui_camera.dirty = true;
            main_canvas.set_size(ComponentSize::Absolute(*size));

            for component in self.components.iter_mut() {
                component.set_dirty_flag(true);
            }
        } else {
            for (component_id, component) in self.components.iter_mut().enumerate() {
                if component_id == self.main_canvas_id {
                    continue;
                }

                self.events.extend(component.process_window_event(event));
            }
        }

        Ok(())
    }

    pub fn poll_event(&mut self) -> Option<UiEvent> {
        self.events.pop_front()
    }

    pub fn begin_scrollbox(&self, scrollbox_id: usize, renderer: &RendererContext) -> Result<(), String> {
        let scrollbox = self.components.get(scrollbox_id)?;
        renderer.enable_scissor(scrollbox.get_work_area_position(), scrollbox.get_work_area_size());

        Ok(())
    }

    pub fn end_scrollbox(&self, renderer: &RendererContext) {
        renderer.disable_scissor();
    }

    pub fn set_active_flag_for_tree(&mut self, root_component: usize, active: bool) -> Result<(), String> {
        let component = self.components.get_mut(root_component)?;
        component.set_active_flag(active);

        for child_id in component.get_children().clone() {
            self.set_active_flag_for_tree(child_id, active)?;
        }

        Ok(())
    }

    pub fn update(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        let main_canvas = self.components.get_mut(self.main_canvas_id)?;
        let area_position = match main_canvas.get_position() {
            ComponentPosition::AbsoluteToParent(position) => position,
            _ => return Err("Invalid canvas".to_string()),
        };
        let area_size = match main_canvas.get_size() {
            ComponentSize::Absolute(size) => size,
            _ => return Err("Invalid canvas".to_string()),
        };
        let updated_components = self.update_internal(renderer, self.main_canvas_id, area_position, area_size, None, Default::default(), false)?;

        if self.debug && updated_components > 0 {
            println!("{} components updated", updated_components);
        }

        Ok(())
    }

    fn update_internal(
        &mut self,
        renderer: &mut RendererContext,
        component_id: usize,
        area_position: Vec2,
        area_size: Vec2,
        event_mask: Option<EventMask>,
        scroll_offset: Option<Vec2>,
        force: bool,
    ) -> Result<u32, String> {
        let component = self.components.get_mut(component_id)?;
        let update = force || component.is_dirty();
        let mut updated_components = 0;

        if let Some(scroll_offset) = scroll_offset {
            component.set_scroll_offset(scroll_offset);
        }

        if update {
            component.update(renderer, area_position, area_size)?;
            updated_components += 1;
        }

        let component_area_position = component.get_work_area_position();
        let component_area_size = component.get_work_area_size();
        let (event_mask, scroll_offset) = if let Ok(scrollbox) = self.components.get_and_cast::<Scrollbox>(component_id) {
            (Some(EventMask::new(component_area_position, component_area_size)), if update { Some(scrollbox.get_scroll_delta()) } else { None })
        } else {
            (event_mask, Default::default())
        };

        self.components.get_mut(component_id)?.set_event_mask(event_mask);

        for child_id in self.components.get_mut(component_id)?.get_children().clone() {
            updated_components +=
                self.update_internal(renderer, child_id, component_area_position, component_area_size, event_mask, scroll_offset, force || update)?;
        }

        // Scrollbox needs to be updated second time, after all children are refreshed
        if self.components.get_and_cast::<Scrollbox>(component_id).is_ok() && updated_components > 1 {
            let mut left_bottom_corner: Vec2 = Vec2::new(f32::MAX, f32::MAX);
            let mut right_top_corner: Vec2 = Vec2::new(f32::MIN, f32::MIN);

            for child_id in self.components.get(component_id)?.get_children().clone() {
                let child = self.components.get(child_id)?;
                let child_area_position = child.get_work_area_position();
                let child_area_size = child.get_work_area_size();

                left_bottom_corner.x = f32::min(left_bottom_corner.x, child_area_position.x);
                left_bottom_corner.y = f32::min(left_bottom_corner.y, child_area_position.y);
                right_top_corner.x = f32::max(right_top_corner.x, child_area_position.x + child_area_size.x);
                right_top_corner.y = f32::max(right_top_corner.y, child_area_position.y + child_area_size.y);
            }

            self.components.get_and_cast_mut::<Scrollbox>(component_id)?.set_total_size(right_top_corner - left_bottom_corner);
            self.components.get_and_cast_mut::<Scrollbox>(component_id)?.update(renderer, area_position, area_size)?;
        }

        Ok(updated_components)
    }

    pub fn draw(&mut self, renderer: &mut RendererContext, component_id: usize) -> Result<(), String> {
        let active_camera_id = renderer.active_camera_id;
        renderer.set_camera_as_active(self.ui_camera_id)?;

        let component = self.components.get_mut(component_id)?;
        let component_position = component.get_work_area_position();
        let component_size = component.get_work_area_size();
        let component_is_active = component.is_active();
        component.draw(renderer)?;

        if self.debug {
            self.debug_frame.position = component_position;
            self.debug_frame.size = component_size;
            self.debug_frame.color = Color::SolidColor(match component_is_active {
                true => SolidColor::new(1.0, 0.0, 0.0, 1.0),
                false => SolidColor::new(0.2, 0.2, 0.2, 1.0),
            });
            self.debug_frame.update();
            renderer.draw(&mut self.debug_frame)?;
        }

        renderer.set_camera_as_active(active_camera_id)?;
        Ok(())
    }
}
