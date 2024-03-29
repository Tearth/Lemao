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
use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::Texture;
use std::any::Any;

pub struct Checkbox {
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

    // Box properties
    pub r#box: Rectangle,
    pub box_color: Color,
    pub box_offset: Vec2,
    pub box_size: Vec2,
    pub box_checked_texture_id: usize,
    pub box_unchecked_texture_id: usize,

    // Label properties
    pub label: Text,
    pub label_font_id: usize,
    pub label_text: String,
    pub label_offset: Vec2,
    pub label_color: Color,

    // Label shadow properties
    pub label_shadow_enabled: bool,
    pub label_shadow_offset: Vec2,
    pub label_shadow_color: Color,

    // Component-specific properties
    pub pressed: bool,
    pub checked: bool,

    // Event handlers
    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_checkbox_checked: Option<fn(component: &mut Self, mouse_button: MouseButton)>,
    pub on_checkbox_unchecked: Option<fn(component: &mut Self, mouse_button: MouseButton)>,
    pub on_checkbox_changed: Option<fn(component: &mut Self, mouse_button: MouseButton, checked: bool)>,
}

impl Checkbox {
    pub fn new(renderer: &mut RendererContext, label_font_id: usize, box_checked_texture_id: usize, box_unchecked_texture_id: usize) -> Result<Self, String> {
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

            // Box properties
            r#box: renderer.create_rectangle()?,
            box_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            box_offset: Default::default(),
            box_size: Default::default(),
            box_checked_texture_id,
            box_unchecked_texture_id,

            // Label properties
            label: renderer.create_text(label_font_id)?,
            label_font_id,
            label_text: Default::default(),
            label_offset: Default::default(),
            label_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Label shadow properties
            label_shadow_enabled: false,
            label_shadow_offset: Default::default(),
            label_shadow_color: Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)),

            // Component-specific properties
            pressed: false,
            checked: false,

            // Event handlers
            on_cursor_enter: None,
            on_cursor_leave: None,
            on_mouse_button_pressed: None,
            on_mouse_button_released: None,
            on_checkbox_checked: None,
            on_checkbox_unchecked: None,
            on_checkbox_changed: None,
        })
    }

    pub fn set_label_font(&mut self, font: &Font) {
        self.label_font_id = font.id;
        self.dirty = true;
    }

    pub fn set_box_checked_texture(&mut self, box_checked_texture: &Texture) {
        self.box_checked_texture_id = box_checked_texture.id;
        self.box_size = box_checked_texture.size;
        self.dirty = true;
    }

    pub fn set_box_unchecked_texture(&mut self, box_unchecked_texture: &Texture) {
        self.box_unchecked_texture_id = box_unchecked_texture.id;
        self.box_size = box_unchecked_texture.size;
        self.dirty = true;
    }

    fn is_point_inside(&self, point: Vec2) -> bool {
        if !self.active {
            return false;
        }

        if let Some(event_mask) = self.event_mask {
            let event_mask_left_bottom = event_mask.position;
            let event_mask_right_top = event_mask.position + event_mask.size;

            if point.x < event_mask_left_bottom.x || point.y < event_mask_left_bottom.y || point.x > event_mask_right_top.x || point.y > event_mask_right_top.y {
                return false;
            }
        }

        let x1 = self.screen_position.x;
        let y1 = self.screen_position.y;
        let x2 = self.screen_position.x + self.label_offset.x + self.screen_size.x;
        let y2 = self.screen_position.y + self.label_offset.y + self.screen_size.y;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }
}

impl Component for Checkbox {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

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
                            (f)(self, *cursor_position)
                        };
                        events.push(UiEvent::CursorEnter(self.id, *cursor_position));
                    }
                } else {
                    if self.is_point_inside(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_leave {
                            (f)(self, *cursor_position)
                        };
                        events.push(UiEvent::CursorLeave(self.id, *cursor_position));
                    }
                }
            }
            InputEvent::MouseButtonPressed(button, cursor_position) => {
                if self.is_point_inside(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_pressed {
                        (f)(self, *button, *cursor_position)
                    };

                    events.push(UiEvent::MouseButtonPressed(self.id, *button));
                    self.pressed = true;
                }
            }
            InputEvent::MouseButtonReleased(button, cursor_position) => {
                if self.is_point_inside(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_released {
                        (f)(self, *button, *cursor_position)
                    };
                    events.push(UiEvent::MouseButtonReleased(self.id, *button));

                    if self.pressed {
                        if self.checked {
                            if let Some(f) = self.on_checkbox_unchecked {
                                (f)(self, *button);
                                self.dirty = true;
                            };
                            events.push(UiEvent::CheckboxUnchecked(self.id, *button));
                        } else {
                            if let Some(f) = self.on_checkbox_checked {
                                (f)(self, *button);
                                self.dirty = true;
                            };
                            events.push(UiEvent::CheckboxChecked(self.id, *button));
                        }

                        self.checked = !self.checked;
                        self.dirty = true;
                        self.pressed = false;

                        if let Some(f) = self.on_checkbox_changed {
                            (f)(self, *button, self.checked);
                            self.dirty = true;
                        };
                        events.push(UiEvent::CheckboxChanged(self.id, *button, self.checked));
                    }
                }
            }
            _ => {}
        }

        events
    }

    fn update(&mut self, renderer: &mut RendererContext, area_position: Vec2, area_size: Vec2) -> Result<(), String> {
        // We have to set text first, to get the size used later
        let font = renderer.fonts.get(self.label_font_id)?;
        self.label.set_font(font);
        self.label.text = self.label_text.clone();
        self.label.update();

        self.screen_size = self.label.size + self.label_offset;
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

        self.r#box.position = self.screen_position + self.box_offset;
        self.r#box.color = self.box_color.clone();

        let texture_id = if self.checked { self.box_checked_texture_id } else { self.box_unchecked_texture_id };
        let texture = renderer.textures.get(texture_id)?;
        self.box_size = texture.size;
        self.r#box.set_texture(texture);

        self.r#box.size = self.box_size;
        self.r#box.update();

        self.label.position = self.screen_position + self.label_offset;
        self.label.color = self.label_color.clone();
        self.label.update();

        self.dirty = false;

        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        if self.label_shadow_enabled {
            let original_position = self.label.position;
            let original_color = self.label.color.clone();

            self.label.position = original_position + self.label_shadow_offset;
            self.label.color = self.label_shadow_color.clone();
            renderer.draw(&mut self.label)?;

            self.label.position = original_position;
            self.label.color = original_color;
        }

        renderer.draw(&mut self.r#box)?;
        renderer.draw(&mut self.label)?;
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
