use crate::events::UiEvent;

use super::Component;
use super::ComponentMargin;
use super::ComponentPosition;
use super::ComponentSize;
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

    position: ComponentPosition,
    screen_position: Vec2,
    size: ComponentSize,
    screen_size: Vec2,
    min_size: Vec2,
    max_size: Vec2,
    anchor: Vec2,
    margin: ComponentMargin,
    offset: Vec2,
    color: Color,
    box_color: Color,
    box_offset: Vec2,
    box_size: Vec2,
    pressed: bool,
    checked: bool,
    label_font_id: usize,
    label_text: String,
    label_offset: Vec2,
    label_id: usize,
    box_checked_texture_id: usize,
    box_unchecked_texture_id: usize,
    box_id: usize,
    children: Vec<usize>,

    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,

    pub on_cursor_box_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_box_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_box_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_box_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_box_checked: Option<fn(component: &mut Self, mouse_button: MouseButton)>,
    pub on_box_unchecked: Option<fn(component: &mut Self, mouse_button: MouseButton)>,
    pub on_box_changed: Option<fn(component: &mut Self, mouse_button: MouseButton, checked: bool)>,
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
            position: ComponentPosition::AbsoluteToParent(Default::default()),
            screen_position: Default::default(),
            size: ComponentSize::Absolute(Default::default()),
            screen_size: Default::default(),
            min_size: Vec2::new(f32::MIN, f32::MIN),
            max_size: Vec2::new(f32::MAX, f32::MAX),
            anchor: Default::default(),
            margin: Default::default(),
            offset: Default::default(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            box_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            box_size: Default::default(),
            pressed: false,
            checked: false,
            box_offset: Default::default(),
            label_font_id,
            label_text: Default::default(),
            label_offset: Default::default(),
            label_id: renderer.create_text(label_font_id)?,
            box_checked_texture_id,
            box_unchecked_texture_id,
            box_id: renderer.create_rectangle()?,
            children: Default::default(),

            on_cursor_enter: None,
            on_cursor_leave: None,
            on_mouse_button_pressed: None,
            on_mouse_button_released: None,

            on_cursor_box_enter: None,
            on_cursor_box_leave: None,
            on_mouse_button_box_pressed: None,
            on_mouse_button_box_released: None,
            on_box_checked: None,
            on_box_unchecked: None,
            on_box_changed: None,
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_font_id(&self) -> usize {
        self.label_font_id
    }

    pub fn set_font_id(&mut self, font_id: usize) {
        self.label_font_id = font_id;
    }

    pub fn get_text(&self) -> &str {
        &self.label_text
    }

    pub fn set_text(&mut self, text: String) {
        self.label_text = text;
    }
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    pub fn get_label_offset(&self) -> Vec2 {
        self.label_offset
    }

    pub fn set_label_offset(&mut self, label_offset: Vec2) {
        self.label_offset = label_offset;
    }

    pub fn get_box_checked_texture_id(&self) -> usize {
        self.box_checked_texture_id
    }

    pub fn set_box_checked_texture(&mut self, box_checked_texture: &Texture) {
        self.box_checked_texture_id = box_checked_texture.get_id();
        self.box_size = box_checked_texture.get_size();
    }

    pub fn get_box_unchecked_texture_id(&self) -> usize {
        self.box_unchecked_texture_id
    }

    pub fn set_box_unchecked_texture(&mut self, box_unchecked_texture: &Texture) {
        self.box_unchecked_texture_id = box_unchecked_texture.get_id();
        self.box_size = box_unchecked_texture.get_size();
    }

    pub fn get_box_color(&self) -> &Color {
        &self.box_color
    }

    pub fn set_box_color(&mut self, box_color: Color) {
        self.color = box_color;
    }

    pub fn get_box_offset(&self) -> &Vec2 {
        &self.box_offset
    }

    pub fn set_box_offset(&mut self, box_offset: Vec2) {
        self.box_offset = box_offset;
    }

    fn is_point_inside(&self, point: Vec2) -> bool {
        let x1 = self.screen_position.x;
        let y1 = self.screen_position.y;
        let x2 = self.screen_position.x + self.screen_size.x;
        let y2 = self.screen_position.y + self.screen_size.y;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }

    fn is_point_inside_box(&self, point: Vec2) -> bool {
        let x1 = self.screen_position.x + self.box_offset.x;
        let y1 = self.screen_position.y + self.box_offset.y;
        let x2 = self.screen_position.x + self.box_offset.x + self.box_size.x;
        let y2 = self.screen_position.y + self.box_offset.y + self.box_size.y;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }
}

impl Component for Checkbox {
    fn get_position(&self) -> ComponentPosition {
        self.position
    }

    fn get_work_area_position(&self) -> Vec2 {
        self.screen_position
    }

    fn set_position(&mut self, position: ComponentPosition) {
        self.position = position;
    }

    fn get_size(&self) -> ComponentSize {
        self.size
    }

    fn get_work_area_size(&self) -> Vec2 {
        self.screen_size
    }

    fn set_size(&mut self, size: ComponentSize) {
        // Can't be set explicitly
        // self.size = size;
    }

    fn get_min_size(&self) -> Vec2 {
        self.min_size
    }

    fn set_min_size(&mut self, min_size: Vec2) {
        // Can't be set explicitly
        // self.min_size = min_size;
    }

    fn get_max_size(&self) -> Vec2 {
        self.max_size
    }

    fn set_max_size(&mut self, max_size: Vec2) {
        // Can't be set explicitly
        // self.max_size = max_size;
    }

    fn get_anchor(&self) -> Vec2 {
        self.anchor
    }

    fn set_anchor(&mut self, anchor: Vec2) {
        self.anchor = anchor;
    }

    fn get_margin(&self) -> ComponentMargin {
        self.margin
    }

    fn set_margin(&mut self, margin: ComponentMargin) {
        self.margin = margin;
    }

    fn get_offset(&self) -> Vec2 {
        self.offset
    }

    fn set_offset(&mut self, offset: Vec2) {
        self.offset = offset;
    }

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
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
                }
            }
            InputEvent::MouseButtonReleased(button, cursor_position) => {
                if self.is_point_inside(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_released {
                        (f)(self, *button, *cursor_position)
                    };
                    events.push(UiEvent::MouseButtonReleased(self.id, *button));
                }
            }
            _ => {}
        }

        // Box
        match event {
            InputEvent::MouseMoved(cursor_position, previous_cursor_position) => {
                if self.is_point_inside_box(*cursor_position) {
                    if !self.is_point_inside_box(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_box_enter {
                            (f)(self, *cursor_position)
                        };
                        events.push(UiEvent::CursorCheckboxEnter(self.id, *cursor_position));
                    }
                } else {
                    if self.is_point_inside_box(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_box_leave {
                            (f)(self, *cursor_position)
                        };
                        events.push(UiEvent::CursorCheckboxLeave(self.id, *cursor_position));
                    }
                }
            }
            InputEvent::MouseButtonPressed(button, cursor_position) => {
                if self.is_point_inside_box(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_box_pressed {
                        (f)(self, *button, *cursor_position)
                    };
                    events.push(UiEvent::MouseButtonCheckboxPressed(self.id, *button));
                    self.pressed = true;
                }
            }
            InputEvent::MouseButtonReleased(button, cursor_position) => {
                if self.is_point_inside_box(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_box_released {
                        (f)(self, *button, *cursor_position)
                    };
                    events.push(UiEvent::MouseButtonCheckboxReleased(self.id, *button));

                    if self.pressed {
                        if self.checked {
                            if let Some(f) = self.on_box_unchecked {
                                (f)(self, *button)
                            };
                            events.push(UiEvent::CheckboxUnchecked(self.id, *button));
                        } else {
                            if let Some(f) = self.on_box_checked {
                                (f)(self, *button)
                            };
                            events.push(UiEvent::CheckboxChecked(self.id, *button));
                        }

                        self.checked = !self.checked;

                        if let Some(f) = self.on_box_changed {
                            (f)(self, *button, self.checked)
                        };
                        events.push(UiEvent::CheckboxChanged(self.id, *button, self.checked));
                    }
                }

                self.pressed = false;
            }
            _ => {}
        }

        events
    }

    fn update(&mut self, renderer: &mut RendererContext, area_position: Vec2, area_size: Vec2) -> Result<(), String> {
        self.screen_size = renderer.get_drawable_with_type_mut::<Text>(self.label_id)?.get_size();
        self.size = ComponentSize::Absolute(self.screen_size);

        self.screen_position = match self.position {
            ComponentPosition::AbsoluteToParent(position) => area_position + position,
            ComponentPosition::RelativeToParent(position) => area_position + (position * area_size),
        } - (self.screen_size * self.anchor);

        self.screen_position += Vec2::new(
            self.margin.left * self.anchor.x - self.margin.right * (self.anchor.x - 1.0),
            self.margin.bottom * (self.anchor.y - 1.0) - self.margin.top * self.anchor.y,
        ) + self.offset;

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

        let font_storage = renderer.get_fonts();
        let font_storage_lock = font_storage.lock().unwrap();
        let font = font_storage_lock.get(self.label_font_id)?;
        let label = renderer.get_drawable_with_type_mut::<Text>(self.label_id)?;

        label.set_font(font);
        label.set_text(&self.label_text);
        label.set_position(self.screen_position + self.label_offset);
        label.set_color(self.color.clone());

        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
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
