use crate::events::UiEvent;

use super::Component;
use super::ComponentMargin;
use super::ComponentPosition;
use super::ComponentSize;
use super::EventMask;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_common_platform::input::MouseWheelDirection;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::Color;
use std::any::Any;

pub struct Scrollbox {
    pub(crate) id: usize,

    position: ComponentPosition,
    screen_position: Vec2,
    size: ComponentSize,
    screen_size: Vec2,
    total_size: Vec2,
    min_size: Vec2,
    max_size: Vec2,
    anchor: Vec2,
    margin: ComponentMargin,
    offset: Vec2,
    scroll_offset: Vec2,
    scroll_difference: Vec2,
    scroll_delta: Vec2,
    scroll_speed: Vec2,
    children: Vec<usize>,
    dirty: bool,
    event_mask: Option<EventMask>,

    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_scroll: Option<fn(component: &mut Self, direction: MouseWheelDirection)>,
}

impl Scrollbox {
    pub fn new(id: usize) -> Result<Self, String> {
        Ok(Self {
            id,
            position: ComponentPosition::AbsoluteToParent(Default::default()),
            screen_position: Default::default(),
            size: ComponentSize::Absolute(Default::default()),
            screen_size: Default::default(),
            total_size: Default::default(),
            min_size: Vec2::new(f32::MIN, f32::MIN),
            max_size: Vec2::new(f32::MAX, f32::MAX),
            anchor: Default::default(),
            margin: Default::default(),
            offset: Default::default(),
            scroll_offset: Default::default(),
            scroll_difference: Default::default(),
            scroll_delta: Default::default(),
            scroll_speed: Vec2::new(5.0, 5.0),
            children: Default::default(),
            dirty: true,
            event_mask: None,

            on_cursor_enter: None,
            on_cursor_leave: None,
            on_mouse_button_pressed: None,
            on_mouse_button_released: None,
            on_scroll: None,
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_total_size(&self) -> Vec2 {
        self.total_size
    }

    pub fn set_total_size(&mut self, total_size: Vec2) {
        self.total_size = total_size;
        self.dirty = true;
    }

    pub fn get_scroll_difference(&self) -> Vec2 {
        self.scroll_difference
    }

    pub fn set_scroll_difference(&mut self, scroll_difference: Vec2) {
        self.scroll_difference = scroll_difference;
        self.dirty = true;
    }

    pub fn get_scroll_delta(&self) -> Vec2 {
        self.scroll_delta
    }

    pub fn set_scroll_delta(&mut self, scroll_delta: Vec2) {
        self.scroll_delta = scroll_delta;
        self.dirty = true;
    }

    pub fn get_scroll_speed(&self) -> Vec2 {
        self.scroll_speed
    }

    pub fn set_scroll_speed(&mut self, scroll_speed: Vec2) {
        self.scroll_speed = scroll_speed;
        self.dirty = true;
    }

    fn is_point_inside(&self, point: Vec2) -> bool {
        if let Some(event_mask) = self.event_mask {
            let event_mask_left_bottom = event_mask.position;
            let event_mask_right_top = event_mask.position + event_mask.size;

            if point.x < event_mask_left_bottom.x || point.y < event_mask_left_bottom.y || point.x > event_mask_right_top.x || point.y > event_mask_right_top.y
            {
                return false;
            }
        }

        let x1 = self.screen_position.x;
        let y1 = self.screen_position.y;
        let x2 = self.screen_position.x + self.screen_size.x;
        let y2 = self.screen_position.y + self.screen_size.y;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }
}

impl Component for Scrollbox {
    fn get_position(&self) -> ComponentPosition {
        self.position
    }

    fn get_work_area_position(&self) -> Vec2 {
        self.screen_position
    }

    fn set_position(&mut self, position: ComponentPosition) {
        self.position = position;
        self.dirty = true;
    }

    fn get_size(&self) -> ComponentSize {
        self.size
    }

    fn get_work_area_size(&self) -> Vec2 {
        self.screen_size
    }

    fn set_size(&mut self, size: ComponentSize) {
        self.size = size;
        self.dirty = true;
    }

    fn get_min_size(&self) -> Vec2 {
        self.min_size
    }

    fn set_min_size(&mut self, min_size: Vec2) {
        self.min_size = min_size;
        self.dirty = true;
    }

    fn get_max_size(&self) -> Vec2 {
        self.max_size
    }

    fn set_max_size(&mut self, max_size: Vec2) {
        self.max_size = max_size;
        self.dirty = true;
    }

    fn get_anchor(&self) -> Vec2 {
        self.anchor
    }

    fn set_anchor(&mut self, anchor: Vec2) {
        self.anchor = anchor;
        self.dirty = true;
    }

    fn get_margin(&self) -> ComponentMargin {
        self.margin
    }

    fn set_margin(&mut self, margin: ComponentMargin) {
        self.margin = margin;
        self.dirty = true;
    }

    fn get_offset(&self) -> Vec2 {
        self.offset
    }

    fn set_offset(&mut self, offset: Vec2) {
        self.offset = offset;
        self.dirty = true;
    }

    fn get_scroll_offset(&self) -> Vec2 {
        self.scroll_offset
    }

    fn set_scroll_offset(&mut self, scroll_offset: Vec2) {
        self.scroll_offset = scroll_offset;
        self.dirty = true;
    }

    fn get_color(&self) -> &Color {
        panic!("Not supported")
    }

    fn set_color(&mut self, color: Color) {
        panic!("Not supported")
    }

    fn add_child(&mut self, component_id: usize) {
        self.children.push(component_id);
    }

    fn remove_child(&mut self, component_id: usize) {
        self.children.retain(|&p| p != component_id);
    }

    fn get_children(&self) -> &Vec<usize> {
        &self.children
    }

    fn process_window_event(&mut self, renderer: &mut RendererContext, event: &InputEvent) -> Vec<UiEvent> {
        let mut events: Vec<UiEvent> = Default::default();

        match event {
            InputEvent::MouseMoved(cursor_position, previous_cursor_position) => {
                if self.is_point_inside(*cursor_position) {
                    if !self.is_point_inside(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_enter {
                            (f)(self, *cursor_position);
                            self.dirty = true;
                        };
                        events.push(UiEvent::CursorEnter(self.id, *cursor_position));
                    }
                } else {
                    if self.is_point_inside(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_leave {
                            (f)(self, *cursor_position);
                            self.dirty = true;
                        };
                        events.push(UiEvent::CursorLeave(self.id, *cursor_position));
                    }
                }
            }
            InputEvent::MouseButtonPressed(button, cursor_position) => {
                if self.is_point_inside(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_pressed {
                        (f)(self, *button, *cursor_position);
                        self.dirty = true;
                    };
                    events.push(UiEvent::MouseButtonPressed(self.id, *button));
                }
            }
            InputEvent::MouseButtonReleased(button, cursor_position) => {
                if self.is_point_inside(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_released {
                        (f)(self, *button, *cursor_position);
                        self.dirty = true;
                    };
                    events.push(UiEvent::MouseButtonReleased(self.id, *button));
                }
            }
            InputEvent::MouseWheelRotated(direction) => {
                match direction {
                    MouseWheelDirection::Up => self.scroll_delta -= Vec2::new(0.0, self.scroll_speed.y),
                    MouseWheelDirection::Down => self.scroll_delta += Vec2::new(0.0, self.scroll_speed.y),
                    _ => {}
                };

                self.scroll_delta = self.scroll_delta.clamp(Vec2::new(0.0, 0.0), self.scroll_difference);

                if let Some(f) = self.on_scroll {
                    (f)(self, *direction);
                    self.dirty = true;
                };
                events.push(UiEvent::ScrollboxScroll(self.id, *direction));
            }
            _ => {}
        }

        events
    }

    fn update(&mut self, _renderer: &mut RendererContext, area_position: Vec2, area_size: Vec2) -> Result<(), String> {
        if !self.dirty {
            return Ok(());
        }

        self.screen_size = match self.size {
            ComponentSize::Absolute(size) => size,
            ComponentSize::Relative(size) => area_size * size,
        };

        self.screen_size = self.screen_size.clamp(self.min_size, self.max_size);

        self.screen_position = match self.position {
            ComponentPosition::AbsoluteToParent(position) => area_position + position,
            ComponentPosition::RelativeToParent(position) => area_position + (position * area_size),
        };
        self.screen_position += Vec2::new(
            self.margin.left * (1.0 - self.anchor.x) - self.margin.right * self.anchor.x,
            self.margin.bottom * (1.0 - self.anchor.y) - self.margin.top * self.anchor.y,
        ) + self.offset;
        self.screen_size -= Vec2::new(self.margin.left + self.margin.right, self.margin.bottom + self.margin.top);
        self.screen_position -= self.screen_size * self.anchor;
        self.screen_position += self.scroll_offset;

        self.screen_size = self.screen_size.floor();
        self.screen_position = self.screen_position.floor();

        self.scroll_difference = (self.total_size - self.screen_size).clamp(Vec2::new(0.0, 0.0), Vec2::new(f32::MAX, f32::MAX));
        self.dirty = false;

        Ok(())
    }

    fn draw(&mut self, _renderer: &mut RendererContext) -> Result<(), String> {
        Ok(())
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty_flag(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn get_event_mask(&self) -> Option<EventMask> {
        self.event_mask
    }

    fn set_event_mask(&mut self, event_mask: Option<EventMask>) {
        self.event_mask = event_mask;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
