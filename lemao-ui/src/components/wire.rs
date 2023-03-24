use super::Component;
use super::ComponentMargin;
use super::ComponentPosition;
use super::ComponentSize;
use super::EventMask;
use crate::events::UiEvent;
use crate::utils::storage::UiStorageItem;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::line::Line;
use lemao_core::renderer::drawable::Color;
use std::any::Any;

pub struct Wire {
    pub id: usize,

    // Common properties
    pub position: ComponentPosition,
    pub screen_position: Vec2,
    pub size: ComponentSize,
    pub screen_size: Vec2,
    pub min_size: Vec2,
    pub max_size: Vec2,
    pub anchor: Vec2,
    pub margin: ComponentMargin,
    pub offset: Vec2,
    pub scroll_offset: Vec2,
    pub active: bool,
    pub dirty: bool,
    pub children: Vec<usize>,
    pub event_mask: Option<EventMask>,

    // Line properties
    pub chunks: Vec<WireChunk>,
    pub data: Vec<WireChunkData>,

    // Event handlers
    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
}

pub struct WireChunkData {
    color: Color,
    from: Vec2,
    to: Vec2,
    thickness: f32,
}

pub struct WireChunk {
    // Shape properties
    line: Line,
}

impl Wire {
    pub fn new(_renderer: &mut RendererContext) -> Result<Self, String> {
        Ok(Self {
            id: 0,

            // Common properties
            position: ComponentPosition::AbsoluteToParent(Default::default()),
            screen_position: Default::default(),
            size: ComponentSize::Absolute(Default::default()),
            screen_size: Default::default(),
            min_size: Vec2::new(f32::MIN, f32::MIN),
            max_size: Vec2::new(f32::MAX, f32::MAX),
            anchor: Default::default(),
            margin: Default::default(),
            offset: Default::default(),
            scroll_offset: Default::default(),
            active: true,
            dirty: true,
            children: Default::default(),
            event_mask: None,

            // Chunk properties
            data: Default::default(),
            chunks: Default::default(),

            // Event handlers
            on_cursor_enter: None,
            on_cursor_leave: None,
            on_mouse_button_pressed: None,
            on_mouse_button_released: None,
        })
    }

    fn is_point_inside(&self, point: Vec2) -> bool {
        if !self.active {
            return false;
        }

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

impl Component for Wire {
    /* #region Common properties */
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

    fn get_scroll_offset(&self) -> Vec2 {
        self.scroll_offset
    }

    fn set_scroll_offset(&mut self, scroll_offset: Vec2) {
        self.scroll_offset = scroll_offset;
        self.dirty = true;
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty_flag(&mut self, dirty: bool) {
        self.dirty = dirty;
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

    fn get_event_mask(&self) -> Option<EventMask> {
        self.event_mask
    }

    fn set_event_mask(&mut self, event_mask: Option<EventMask>) {
        self.event_mask = event_mask;
    }
    /* #endregion */

    fn process_window_event(&mut self, event: &InputEvent) -> Vec<UiEvent> {
        let mut events: Vec<UiEvent> = Default::default();

        if !self.active {
            return events;
        }

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
            _ => {}
        }

        events
    }

    fn update(&mut self, renderer: &mut RendererContext, area_position: Vec2, area_size: Vec2) -> Result<(), String> {
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

        self.chunks.clear();

        for chunk_data in &mut self.data {
            let mut chunk = WireChunk::new(renderer)?;
            chunk.line.from = self.screen_position + chunk_data.from * self.screen_size;
            chunk.line.to = self.screen_position + chunk_data.to * self.screen_size;
            chunk.line.color = chunk_data.color.clone();
            chunk.line.thickness = chunk_data.thickness;
            chunk.line.update();
            self.chunks.push(chunk);
        }

        self.dirty = false;
        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        for chunk in &mut self.chunks {
            renderer.draw(&mut chunk.line)?;
        }

        Ok(())
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active_flag(&mut self, active: bool) {
        self.active = active;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl WireChunk {
    pub fn new(renderer: &mut RendererContext) -> Result<Self, String> {
        Ok(Self { line: renderer.create_line()? })
    }
}

impl WireChunkData {
    pub fn new(color: Color, from: Vec2, to: Vec2, thickness: f32) -> Self {
        Self { color, from, to, thickness }
    }
}

impl UiStorageItem for Wire {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_component(&self) -> Option<&dyn Component> {
        Some(self)
    }

    fn as_component_mut(&mut self) -> Option<&mut dyn Component> {
        Some(self)
    }
}
