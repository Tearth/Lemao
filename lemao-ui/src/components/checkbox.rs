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
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::textures::Texture;
use std::any::Any;

pub struct Checkbox {
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
    dirty: bool,
    children: Vec<usize>,
    event_mask: Option<EventMask>,

    // Box properties
    box_id: usize,
    box_color: Color,
    box_offset: Vec2,
    box_size: Vec2,
    box_checked_texture_id: usize,
    box_unchecked_texture_id: usize,

    // Label properties
    label_id: usize,
    label_font_id: usize,
    label_text: String,
    label_offset: Vec2,
    label_color: Color,

    // Shadow
    shadow_enabled: bool,
    shadow_offset: Vec2,
    shadow_color: Color,

    // Component-specific properties
    pressed: bool,
    checked: bool,

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
    pub fn new(
        id: usize,
        renderer: &mut RendererContext,
        label_font_id: usize,
        box_checked_texture_id: usize,
        box_unchecked_texture_id: usize,
    ) -> Result<Self, String> {
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
            dirty: true,
            children: Default::default(),
            event_mask: None,

            // Box properties
            box_id: renderer.create_rectangle()?,
            box_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            box_offset: Default::default(),
            box_size: Default::default(),
            box_checked_texture_id,
            box_unchecked_texture_id,

            // Label properties
            label_id: renderer.create_text(label_font_id)?,
            label_font_id,
            label_text: Default::default(),
            label_offset: Default::default(),
            label_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Shadow
            shadow_enabled: false,
            shadow_offset: Default::default(),
            shadow_color: Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)),

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

    pub fn get_id(&self) -> usize {
        self.id
    }

    /* #region Box properties */
    pub fn get_box_checked_texture_id(&self) -> usize {
        self.box_checked_texture_id
    }

    pub fn set_box_checked_texture(&mut self, box_checked_texture: &Texture) {
        self.box_checked_texture_id = box_checked_texture.get_id();
        self.box_size = box_checked_texture.get_size();
        self.dirty = true;
    }

    pub fn get_box_unchecked_texture_id(&self) -> usize {
        self.box_unchecked_texture_id
    }

    pub fn set_box_unchecked_texture(&mut self, box_unchecked_texture: &Texture) {
        self.box_unchecked_texture_id = box_unchecked_texture.get_id();
        self.box_size = box_unchecked_texture.get_size();
        self.dirty = true;
    }

    pub fn get_box_color(&self) -> &Color {
        &self.box_color
    }

    pub fn set_box_color(&mut self, box_color: Color) {
        self.label_color = box_color;
        self.dirty = true;
    }

    pub fn get_box_offset(&self) -> &Vec2 {
        &self.box_offset
    }

    pub fn set_box_offset(&mut self, box_offset: Vec2) {
        self.box_offset = box_offset;
        self.dirty = true;
    }
    /* #endregion */

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
        self.dirty = true;
    }

    pub fn get_label_offset(&self) -> Vec2 {
        self.label_offset
    }

    pub fn set_label_offset(&mut self, label_offset: Vec2) {
        self.label_offset = label_offset;
        self.dirty = true;
    }

    pub fn get_label_color(&self) -> &Color {
        &self.label_color
    }

    pub fn set_label_color(&mut self, color: Color) {
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

    /* #region Component-specific properties */
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
        self.dirty = true;
    }
    /* #endregion */

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
        let x2 = self.screen_position.x + self.label_offset.x + self.screen_size.x;
        let y2 = self.screen_position.y + self.label_offset.y + self.screen_size.y;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }
}

impl Component for Checkbox {
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

        // All component
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
        if !self.dirty {
            return Ok(());
        }

        // We have to set text first, to get the size used later
        let font_storage = renderer.get_fonts();
        let font_storage_lock = font_storage.lock().unwrap();
        let font = font_storage_lock.get(self.label_font_id)?;
        renderer.get_drawable_with_type_mut::<Text>(self.label_id)?.set_font(font);
        renderer.get_drawable_with_type_mut::<Text>(self.label_id)?.set_text(&self.label_text);

        self.screen_size = renderer.get_drawable_with_type_mut::<Text>(self.label_id)?.get_size();
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

        let r#box = renderer.get_drawable_with_type_mut::<Rectangle>(self.box_id)?;
        r#box.set_position(self.screen_position + self.box_offset);
        r#box.set_color(self.box_color.clone());

        let texture_storage = renderer.get_textures();
        let texture_storage_lock = texture_storage.lock().unwrap();

        if self.checked {
            let texture = texture_storage_lock.get(self.box_checked_texture_id)?;
            self.box_size = texture.get_size();
            renderer.get_drawable_with_type_mut::<Rectangle>(self.box_id)?.set_texture(texture);
        } else {
            let texture = texture_storage_lock.get(self.box_unchecked_texture_id)?;
            self.box_size = texture.get_size();
            renderer.get_drawable_with_type_mut::<Rectangle>(self.box_id)?.set_texture(texture);
        }

        let label = renderer.get_drawable_with_type_mut::<Text>(self.label_id)?;
        label.set_position(self.screen_position + self.label_offset);
        label.set_color(self.label_color.clone());

        self.dirty = false;

        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        if self.shadow_enabled {
            let panel = renderer.get_drawable_mut(self.label_id)?;
            let original_position = panel.get_position();
            let original_color = panel.get_color().clone();

            let panel = renderer.get_drawable_mut(self.label_id)?;
            panel.set_position(original_position + self.shadow_offset);
            panel.set_color(self.shadow_color.clone());
            renderer.draw(self.label_id)?;

            let panel = renderer.get_drawable_mut(self.label_id)?;
            panel.set_position(original_position);
            panel.set_color(original_color);
        }

        renderer.draw(self.box_id)?;
        renderer.draw(self.label_id)?;
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
