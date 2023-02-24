use super::Component;
use super::ComponentMargin;
use super::ComponentPosition;
use super::ComponentSize;
use super::EventMask;
use crate::events::UiEvent;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::fonts::Font;
use std::any::Any;

pub struct Label {
    pub(crate) id: usize,

    // Common properties
    position: ComponentPosition,
    screen_position: Vec2,
    size: ComponentSize,
    screen_size: Vec2,
    min_size: Vec2,
    max_size: Vec2,
    anchor: Vec2,
    margin: ComponentMargin,
    offset: Vec2,
    scroll_offset: Vec2,
    active: bool,
    dirty: bool,
    children: Vec<usize>,
    event_mask: Option<EventMask>,

    // Label properties
    label_id: usize,
    label_font_id: usize,
    label_text: String,
    label_color: Color,
    multiline: bool,
    max_multiline_width: f32,

    // Shadow properties
    shadow_enabled: bool,
    shadow_offset: Vec2,
    shadow_color: Color,

    // Event handlers
    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
}

impl Label {
    pub fn new(id: usize, renderer: &mut RendererContext, label_font_id: usize) -> Result<Self, String> {
        Ok(Self {
            id,

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

            // Label properties
            label_id: renderer.create_text(label_font_id)?,
            label_font_id,
            label_text: Default::default(),
            label_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            multiline: false,
            max_multiline_width: 0.0,

            // Shadow properties
            shadow_enabled: false,
            shadow_offset: Default::default(),
            shadow_color: Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)),

            // Event handlers
            on_cursor_enter: None,
            on_cursor_leave: None,
            on_mouse_button_pressed: None,
            on_mouse_button_released: None,
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    /* #region Label properties */
    pub fn get_font_id(&self) -> usize {
        self.label_font_id
    }

    pub fn set_font_id(&mut self, font_id: usize) {
        self.label_font_id = font_id;
        self.dirty = true;
    }

    pub fn get_text(&self) -> &str {
        &self.label_text
    }

    pub fn set_text(&mut self, text: String) {
        self.label_text = text;
        self.multiline = false;
        self.dirty = true;
    }

    pub fn set_multiline_text(&mut self, text: String, width: f32) {
        self.label_text = text;
        self.max_multiline_width = width;
        self.multiline = true;
        self.dirty = true;
    }

    pub fn get_color(&self) -> &Color {
        &self.label_color
    }

    pub fn set_color(&mut self, color: Color) {
        self.label_color = color;
        self.dirty = true;
    }
    /* #endregion */

    /* #region Shadow properties */
    pub fn is_shadow_enabled(&self) -> bool {
        self.shadow_enabled
    }

    pub fn set_shadow_enabled_flag(&mut self, shadow_enabled: bool) {
        self.shadow_enabled = shadow_enabled;
    }

    pub fn get_shadow_offset(&self) -> Vec2 {
        self.shadow_offset
    }

    pub fn set_shadow_offset(&mut self, shadow_offset: Vec2) {
        self.shadow_offset = shadow_offset;
    }

    pub fn get_shadow_color(&self) -> &Color {
        &self.shadow_color
    }

    pub fn set_shadow_color(&mut self, get_shadow_color: Color) {
        self.shadow_color = get_shadow_color;
    }
    /* #endregion */

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

impl Component for Label {
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

    fn set_size(&mut self, _size: ComponentSize) {
        // Can't be set explicitly
        // self.size = size;
    }

    fn get_min_size(&self) -> Vec2 {
        self.min_size
    }

    fn set_min_size(&mut self, _min_size: Vec2) {
        // Can't be set explicitly
        // self.min_size = min_size;
    }

    fn get_max_size(&self) -> Vec2 {
        self.max_size
    }

    fn set_max_size(&mut self, _max_size: Vec2) {
        // Can't be set explicitly
        // self.max_size = max_size;
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
        // We have to set text first, to get the size used later
        let label = renderer.get_drawable_and_cast_mut::<Text>(self.label_id)?;
        let mut label_text_processed = self.label_text.clone();
        if self.multiline {
            let mut line = String::new();
            let mut result = String::new();

            for token in label_text_processed.split(' ') {
                if label.calculate_text_size(line.clone() + token).x > self.max_multiline_width {
                    result += &(line.trim().to_string() + "\n");
                    line.clear();
                }

                line += token;
                line += " ";
            }

            label_text_processed = result + &line;
        }

        let font_storage = renderer.get_fonts();
        let font_storage = font_storage.read().unwrap();
        let font = font_storage.get_and_cast::<Font>(self.label_font_id)?;
        renderer.get_drawable_and_cast_mut::<Text>(self.label_id)?.set_font(font);
        renderer.get_drawable_and_cast_mut::<Text>(self.label_id)?.set_text(&label_text_processed);
        renderer.get_drawable_and_cast_mut::<Text>(self.label_id)?.update();

        self.screen_size = renderer.get_drawable_and_cast_mut::<Text>(self.label_id)?.get_size();
        self.size = ComponentSize::Absolute(self.screen_size);

        self.screen_position = match self.position {
            ComponentPosition::AbsoluteToParent(position) => area_position + position,
            ComponentPosition::RelativeToParent(position) => area_position + (position * area_size),
        };
        self.screen_position += Vec2::new(
            self.margin.left * (1.0 - self.anchor.x) - self.margin.right * self.anchor.x,
            self.margin.bottom * (1.0 - self.anchor.y) - self.margin.top * self.anchor.y,
        ) + self.offset;
        self.screen_position -= self.screen_size * self.anchor;
        self.screen_position += self.scroll_offset;

        self.screen_size = self.screen_size.floor();
        self.screen_position = self.screen_position.floor();

        let label = renderer.get_drawable_and_cast_mut::<Text>(self.label_id)?;
        label.set_position(self.screen_position);
        label.set_color(self.label_color.clone());

        self.dirty = false;

        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        if self.shadow_enabled {
            let drawable = renderer.get_drawable_mut(self.label_id)?;
            let original_position = drawable.get_position();
            let original_color = drawable.get_color().clone();

            let drawable = renderer.get_drawable_mut(self.label_id)?;
            drawable.set_position(original_position + self.shadow_offset);
            drawable.set_color(self.shadow_color.clone());
            renderer.draw(self.label_id)?;

            let drawable = renderer.get_drawable_mut(self.label_id)?;
            drawable.set_position(original_position);
            drawable.set_color(original_color);
        }

        renderer.draw(self.label_id)?;
        Ok(())
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn set_active_flag(&mut self, active: bool) {
        self.active = active;
    }

    fn release_internal_resources(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        renderer.remove_drawable(self.label_id)?;

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
