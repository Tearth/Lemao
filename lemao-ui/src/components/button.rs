use super::Component;
use super::ComponentBorderShape;
use super::ComponentBorderThickness;
use super::ComponentCornerRounding;
use super::ComponentFillingShape;
use super::ComponentMargin;
use super::ComponentPosition;
use super::ComponentShape;
use super::ComponentSize;
use super::EventMask;
use super::HorizontalAlignment;
use super::VerticalAlignment;
use crate::events::UiEvent;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::Texture;
use std::any::Any;

pub struct Button {
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

    // Shape properties
    pub filling: ComponentFillingShape,
    pub shape: ComponentShape,
    pub color: Color,
    pub corner_rounding: ComponentCornerRounding,
    pub texture_id: Option<usize>,
    pub texture_original_size: Vec2,

    // Border properties
    pub border: ComponentBorderShape,
    pub border_thickness: ComponentBorderThickness,
    pub border_color: Color,

    // Label properties
    pub label: Text,
    pub label_font_id: usize,
    pub label_text: String,
    pub label_horizontal_alignment: HorizontalAlignment,
    pub label_vertical_alignment: VerticalAlignment,
    pub label_offset: Vec2,
    pub label_color: Color,

    // Shadow properties
    pub shadow: ComponentFillingShape,
    pub shadow_enabled: bool,
    pub shadow_offset: Vec2,
    pub shadow_color: Color,
    pub shadow_scale: Vec2,
    pub shadow_corner_rounding: ComponentCornerRounding,

    // Label shadow properties
    pub label_shadow_enabled: bool,
    pub label_shadow_offset: Vec2,
    pub label_shadow_color: Color,

    // Component-specific properties
    pub pressed: bool,
    pub toggleable: bool,

    // Event handlers
    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_button_clicked: Option<fn(component: &mut Self, mouse_button: MouseButton)>,
}

impl Button {
    pub fn new(renderer: &mut RendererContext, shape: ComponentShape, label_font_id: usize) -> Result<Self, String> {
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

            // Shape properties
            filling: match shape {
                ComponentShape::Rectangle => ComponentFillingShape::Rectangle(renderer.create_rectangle()?),
                ComponentShape::Disc => ComponentFillingShape::Disc(renderer.create_disc()?),
            },
            shape,
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            corner_rounding: Default::default(),
            texture_id: None,
            texture_original_size: Default::default(),

            // Border properties
            border: match shape {
                ComponentShape::Rectangle => ComponentBorderShape::Frame(renderer.create_frame()?),
                ComponentShape::Disc => ComponentBorderShape::Circle(renderer.create_circle()?),
            },
            border_thickness: Default::default(),
            border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Label properties
            label: renderer.create_text(label_font_id)?,
            label_font_id,
            label_text: Default::default(),
            label_horizontal_alignment: HorizontalAlignment::Middle,
            label_vertical_alignment: VerticalAlignment::Middle,
            label_offset: Default::default(),
            label_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Shadow properties
            shadow: match shape {
                ComponentShape::Rectangle => ComponentFillingShape::Rectangle(renderer.create_rectangle()?),
                ComponentShape::Disc => ComponentFillingShape::Disc(renderer.create_disc()?),
            },
            shadow_enabled: false,
            shadow_offset: Default::default(),
            shadow_color: Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)),
            shadow_scale: Vec2::new(1.0, 1.0),
            shadow_corner_rounding: Default::default(),

            // Label shadow properties
            label_shadow_enabled: false,
            label_shadow_offset: Default::default(),
            label_shadow_color: Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)),

            // Component-specific properties
            pressed: false,
            toggleable: false,

            // Event handlers
            on_cursor_enter: None,
            on_cursor_leave: None,
            on_mouse_button_pressed: None,
            on_mouse_button_released: None,
            on_button_pressed: None,
            on_button_released: None,
            on_button_clicked: None,
        })
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.texture_id = Some(texture.id);
        self.texture_original_size = texture.size;
        self.size = ComponentSize::Absolute(texture.size);
        self.dirty = true;
    }

    pub fn set_label_font(&mut self, label_font: &Font) {
        self.label_font_id = label_font.id;
        self.dirty = true;
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

        if self.shape == ComponentShape::Rectangle {
            let x1 = self.screen_position.x;
            let y1 = self.screen_position.y;
            let x2 = self.screen_position.x + self.screen_size.x;
            let y2 = self.screen_position.y + self.screen_size.y;

            point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
        } else {
            let scale = self.screen_size.x / self.screen_size.y;
            let component_center = self.screen_position + self.screen_size / 2.0;
            let normalized_point = point - component_center;
            let scaled_point = normalized_point * Vec2::new(1.0, scale);

            scaled_point.distance(Vec2::new(0.0, 0.0)) <= self.screen_size.x / 2.0
        }
    }
}

impl Component for Button {
    /* #region Common properties */
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

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

                    self.pressed = if !self.toggleable { true } else { !self.pressed };

                    if self.pressed {
                        if let Some(f) = self.on_button_pressed {
                            (f)(self, *button, *cursor_position);
                            self.dirty = true;
                        };
                        events.push(UiEvent::ButtonPressed(self.id, *button));
                    } else {
                        if let Some(f) = self.on_button_released {
                            (f)(self, *button, *cursor_position);
                            self.dirty = true;
                        };
                        events.push(UiEvent::ButtonReleased(self.id, *button));
                    }
                }
            }
            InputEvent::MouseButtonReleased(button, cursor_position) => {
                let pressed = self.pressed;

                if self.is_point_inside(*cursor_position) {
                    if !self.toggleable && self.pressed {
                        if let Some(f) = self.on_button_clicked {
                            (f)(self, *button);
                            self.dirty = true;
                        };
                        events.push(UiEvent::ButtonClicked(self.id, *button));
                    }

                    if pressed {
                        if !self.toggleable {
                            self.pressed = false;
                        }

                        if let Some(f) = self.on_mouse_button_released {
                            (f)(self, *button, *cursor_position);
                            self.dirty = true;
                        };
                        events.push(UiEvent::MouseButtonReleased(self.id, *button));
                    }
                }

                if !self.toggleable && pressed {
                    self.pressed = false;

                    if let Some(f) = self.on_button_released {
                        (f)(self, *button, *cursor_position);
                        self.dirty = true;
                    };
                    events.push(UiEvent::ButtonReleased(self.id, *button));
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

        if self.screen_size.x.is_nan() {
            self.screen_size.x = (self.texture_original_size.x / self.texture_original_size.y) * self.screen_size.y;
        } else if self.screen_size.y.is_nan() {
            self.screen_size.y = (self.texture_original_size.y / self.texture_original_size.x) * self.screen_size.x;
        }

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

        if self.border_thickness != Default::default() {
            match &mut self.border {
                ComponentBorderShape::Frame(border) => {
                    border.position = self.screen_position;
                    border.size = self.screen_size;
                    border.color = self.border_color.clone();
                    border.thickness = self.border_thickness.into();
                    border.corner_rounding = self.corner_rounding.into();

                    border.update();
                }
                ComponentBorderShape::Circle(border) => {
                    border.position = self.screen_position;
                    border.size = self.screen_size;
                    border.color = self.border_color.clone();
                    border.thickness = Vec2::new(self.border_thickness.left, self.border_thickness.top);

                    border.update();
                }
            }

            self.screen_position += Vec2::new(self.border_thickness.left, self.border_thickness.bottom);
            self.screen_size -= Vec2::new(self.border_thickness.left + self.border_thickness.right, self.border_thickness.top + self.border_thickness.bottom);

            self.screen_size = self.screen_size.floor();
            self.screen_position = self.screen_position.floor();
        }

        match &mut self.filling {
            ComponentFillingShape::Rectangle(filling) => {
                filling.position = self.screen_position;
                filling.color = self.color.clone();
                filling.size = self.screen_size;
                filling.corner_rounding = self.corner_rounding.into();

                if let Some(texture_id) = self.texture_id {
                    filling.set_texture(renderer.textures.get(texture_id)?)
                }

                filling.update();
            }
            ComponentFillingShape::Disc(filling) => {
                filling.position = self.screen_position;
                filling.color = self.color.clone();
                filling.size = self.screen_size;

                if let Some(texture_id) = self.texture_id {
                    filling.set_texture(renderer.textures.get(texture_id)?)
                }

                filling.update();
            }
        };

        if self.shadow_enabled {
            match &mut self.shadow {
                ComponentFillingShape::Rectangle(shadow) => {
                    shadow.position = self.screen_position + self.screen_size / 2.0 + self.shadow_offset;
                    shadow.size = self.screen_size;
                    shadow.anchor = Vec2::new(0.5, 0.5);
                    shadow.color = self.shadow_color.clone();
                    shadow.scale = self.shadow_scale;
                    shadow.corner_rounding = self.shadow_corner_rounding.into();

                    shadow.update();
                }
                ComponentFillingShape::Disc(shadow) => {
                    shadow.position = self.screen_position + self.screen_size / 2.0 + self.shadow_offset;
                    shadow.size = self.screen_size;
                    shadow.anchor = Vec2::new(0.5, 0.5);
                    shadow.color = self.shadow_color.clone();
                    shadow.scale = self.shadow_scale;

                    shadow.update();
                }
            };
        }

        let (horizontal_position, horizontal_anchor) = match self.label_horizontal_alignment {
            HorizontalAlignment::Left => (Vec2::new(self.screen_position.x, 0.0), Vec2::new(0.0, 0.0)),
            HorizontalAlignment::Middle => (Vec2::new(self.screen_position.x + (self.screen_size.x) / 2.0, 0.0), Vec2::new(0.5, 0.0)),
            HorizontalAlignment::Right => (Vec2::new(self.screen_position.x + self.screen_size.x, 0.0), Vec2::new(1.0, 0.0)),
        };

        let (vertical_position, vertical_anchor) = match self.label_vertical_alignment {
            VerticalAlignment::Top => (Vec2::new(0.0, self.screen_position.y), Vec2::new(0.0, 0.0)),
            VerticalAlignment::Middle => (Vec2::new(0.0, self.screen_position.y + (self.screen_size.y) / 2.0), Vec2::new(0.0, 0.5)),
            VerticalAlignment::Bottom => (Vec2::new(0.0, self.screen_position.y + self.screen_size.y), Vec2::new(0.0, 1.0)),
        };

        let font = renderer.fonts.get(self.label_font_id)?;
        self.label.set_font(font);
        self.label.text = self.label_text.clone();
        self.label.color = self.label_color.clone();
        self.label.position = horizontal_position + vertical_position + self.label_offset;
        self.label.anchor = horizontal_anchor + vertical_anchor;
        self.label.update();

        self.dirty = false;

        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        if self.shadow_enabled {
            match &mut self.shadow {
                ComponentFillingShape::Rectangle(shadow) => renderer.draw(shadow)?,
                ComponentFillingShape::Disc(shadow) => renderer.draw(shadow)?,
            }
        }

        match &mut self.filling {
            ComponentFillingShape::Rectangle(filling) => renderer.draw(filling)?,
            ComponentFillingShape::Disc(filling) => renderer.draw(filling)?,
        }

        if self.label_shadow_enabled {
            let original_position = self.label.position;
            let original_color = self.label.color.clone();

            self.label.position = original_position + self.label_shadow_offset;
            self.label.color = self.label_shadow_color.clone();
            renderer.draw(&mut self.label)?;

            self.label.position = original_position;
            self.label.color = original_color;
        }

        renderer.draw(&mut self.label)?;

        if self.border_thickness != Default::default() {
            match &mut self.border {
                ComponentBorderShape::Frame(border) => renderer.draw(border)?,
                ComponentBorderShape::Circle(border) => renderer.draw(border)?,
            }
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
