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
use crate::events::UiEvent;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_common_platform::input::MouseWheelDirection;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::frame::Frame;
use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::textures::Texture;
use std::any::Any;

pub struct Slider {
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
    pub filling: Rectangle,
    pub color: Color,
    pub corner_rounding: ComponentCornerRounding,
    pub texture_id: Option<usize>,
    pub texture_original_size: Vec2,

    // Border properties
    pub border: Frame,
    pub border_color: Color,
    pub border_thickness: ComponentBorderThickness,

    // Shadow properties
    pub shadow: Rectangle,
    pub shadow_enabled: bool,
    pub shadow_offset: Vec2,
    pub shadow_color: Color,
    pub shadow_scale: Vec2,
    pub shadow_corner_rounding: ComponentCornerRounding,

    // Bar properties
    pub bar: Rectangle,
    pub bar_color: Color,

    // Selector properties
    pub selector: ComponentFillingShape,
    pub selector_shape: ComponentShape,
    pub selector_position: Vec2,
    pub selector_size: Vec2,
    pub selector_color: Color,

    // Selector border properties
    pub selector_border: ComponentBorderShape,
    pub selector_border_color: Color,
    pub selector_border_thickness: ComponentBorderThickness,

    // Component-specific properties
    pub phase: f32,
    pub phase_unrounded: f32,
    pub selector_pressed: bool,
    pub steps_count: u32,
    pub mouse_wheel_step: f32,

    // Event handlers
    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_selector_move: Option<fn(component: &mut Self, direction: f32)>,
    pub on_cursor_selector_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_selector_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_selector_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_selector_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
}

impl Slider {
    pub fn new(renderer: &mut RendererContext, selector_shape: ComponentShape) -> Result<Self, String> {
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
            filling: renderer.create_rectangle()?,
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            corner_rounding: Default::default(),
            texture_id: None,
            texture_original_size: Default::default(),

            // Border properties
            border: renderer.create_frame()?,
            border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            border_thickness: Default::default(),

            // Shadow properties
            shadow: renderer.create_rectangle()?,
            shadow_enabled: false,
            shadow_offset: Default::default(),
            shadow_color: Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)),
            shadow_scale: Vec2::new(1.0, 1.0),
            shadow_corner_rounding: Default::default(),

            // Bar properties
            bar: renderer.create_rectangle()?,
            bar_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Selector properties
            selector: match selector_shape {
                ComponentShape::Rectangle => ComponentFillingShape::Rectangle(renderer.create_rectangle()?),
                ComponentShape::Disc => ComponentFillingShape::Disc(renderer.create_disc()?),
            },
            selector_shape,
            selector_position: Default::default(),
            selector_size: Vec2::new(20.0, 20.0),
            selector_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Selector border properties
            selector_border: match selector_shape {
                ComponentShape::Rectangle => ComponentBorderShape::Frame(renderer.create_frame()?),
                ComponentShape::Disc => ComponentBorderShape::Circle(renderer.create_circle()?),
            },
            selector_border_thickness: Default::default(),
            selector_border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Component-specific properties
            phase: 0.0,
            phase_unrounded: 0.0,
            selector_pressed: false,
            steps_count: u32::MAX,
            mouse_wheel_step: 0.05,

            // Event handlers
            on_cursor_enter: None,
            on_cursor_leave: None,
            on_mouse_button_pressed: None,
            on_mouse_button_released: None,
            on_selector_move: None,
            on_cursor_selector_enter: None,
            on_cursor_selector_leave: None,
            on_mouse_button_selector_pressed: None,
            on_mouse_button_selector_released: None,
        })
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.texture_id = Some(texture.id);
        self.texture_original_size = texture.size;
        self.size = ComponentSize::Absolute(texture.size);
        self.dirty = true;
    }

    pub fn set_phase(&mut self, phase: f32) {
        self.phase = phase;
        self.phase_unrounded = phase;
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

        let x1 = self.screen_position.x - self.selector_size.x / 2.0;
        let y1 = self.screen_position.y - self.selector_size.y / 2.0;
        let x2 = self.screen_position.x + self.screen_size.x + self.selector_size.x / 2.0;
        let y2 = self.screen_position.y + self.screen_size.y + self.selector_size.y / 2.0;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }

    fn is_point_inside_selector(&self, point: Vec2) -> bool {
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

        let x1 = self.selector_position.x - self.selector_size.x / 2.0;
        let y1 = self.selector_position.y - self.selector_size.y / 2.0;
        let x2 = self.selector_position.x + self.selector_size.x / 2.0;
        let y2 = self.selector_position.y + self.selector_size.y / 2.0;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }

    fn update_selector(&mut self, new_phase: f32, events: &mut Vec<UiEvent>) {
        let difference = new_phase - self.phase;
        let last_phase = self.phase;

        if self.steps_count == u32::MAX {
            self.phase = new_phase;
            self.phase = self.phase.clamp(0.0, 1.0);
        } else {
            self.phase_unrounded = new_phase;
            self.phase_unrounded = self.phase_unrounded.clamp(0.0, 1.0);

            self.phase = (self.phase_unrounded * (self.steps_count as f32 - 1.0)).round() / (self.steps_count as f32 - 1.0);
        }

        if self.phase != last_phase {
            if let Some(f) = self.on_selector_move {
                (f)(self, difference);
                self.dirty = true;
            };
            events.push(UiEvent::SelectorMoved(self.id, difference));
            self.dirty = true;
        }
    }
}

impl Component for Slider {
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

        // Component
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
                    self.selector_pressed = true;

                    if !self.is_point_inside_selector(*cursor_position) {
                        let new_phase = ((cursor_position.x - self.screen_position.x) / self.screen_size.x).clamp(0.0, 1.0);
                        self.update_selector(new_phase, &mut events);
                    }
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
            InputEvent::MouseWheelRotated(direction, cursor_position) => {
                if self.is_point_inside(*cursor_position) {
                    let difference = if self.steps_count == u32::MAX {
                        match direction {
                            MouseWheelDirection::Up => -self.mouse_wheel_step,
                            MouseWheelDirection::Down => self.mouse_wheel_step,
                            _ => 0.0,
                        }
                    } else {
                        let step = 1.0 / self.steps_count as f32;
                        match direction {
                            MouseWheelDirection::Up => -step,
                            MouseWheelDirection::Down => step,
                            _ => 0.0,
                        }
                    };

                    let new_phase = (self.phase + difference).clamp(0.0, 1.0);
                    self.update_selector(new_phase, &mut events);
                }
            }
            _ => {}
        }

        // Selector
        match event {
            InputEvent::MouseMoved(cursor_position, previous_cursor_position) => {
                if self.is_point_inside_selector(*cursor_position) {
                    if !self.is_point_inside_selector(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_selector_enter {
                            (f)(self, *cursor_position);
                            self.dirty = true;
                        };

                        events.push(UiEvent::SelectorCursorEnter(self.id, *cursor_position));
                    }
                } else {
                    if self.is_point_inside_selector(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_selector_leave {
                            (f)(self, *cursor_position);
                            self.dirty = true;
                        };
                        events.push(UiEvent::SelectorCursorLeave(self.id, *cursor_position));
                    }
                }

                if self.selector_pressed {
                    let new_phase = ((cursor_position.x - self.screen_position.x) / self.screen_size.x).clamp(0.0, 1.0);
                    self.update_selector(new_phase, &mut events);
                }
            }
            InputEvent::MouseButtonPressed(button, cursor_position) => {
                if self.is_point_inside_selector(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_selector_pressed {
                        (f)(self, *button, *cursor_position);
                        self.dirty = true;
                    };
                    events.push(UiEvent::SelectorMouseButtonPressed(self.id, *button));
                    self.selector_pressed = true;
                }
            }
            InputEvent::MouseButtonReleased(button, cursor_position) => {
                if self.is_point_inside_selector(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_selector_released {
                        (f)(self, *button, *cursor_position);
                        self.dirty = true;
                    };
                    events.push(UiEvent::SelectorMouseButtonReleased(self.id, *button));
                }

                self.selector_pressed = false;
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
            self.border.position = self.screen_position;
            self.border.size = self.screen_size;
            self.border.color = self.border_color.clone();
            self.border.thickness = self.border_thickness.into();
            self.border.corner_rounding = self.corner_rounding.into();
            self.border.update();

            self.screen_position += Vec2::new(self.border_thickness.left, self.border_thickness.bottom);
            self.screen_size -= Vec2::new(self.border_thickness.left + self.border_thickness.right, self.border_thickness.top + self.border_thickness.bottom);

            self.screen_size = self.screen_size.floor();
            self.screen_position = self.screen_position.floor();
        }

        self.filling.position = self.screen_position;
        self.filling.color = self.color.clone();
        self.filling.corner_rounding = self.corner_rounding.into();

        if let Some(texture_id) = self.texture_id {
            self.filling.set_texture(renderer.textures.get(texture_id)?)
        }

        self.filling.update();

        self.bar.position = self.screen_position;
        self.bar.color = self.bar_color.clone();
        self.bar.size = self.screen_size * Vec2::new(self.phase, 1.0);
        self.bar.corner_rounding = self.corner_rounding.into();
        self.bar.update();

        self.selector_position = Vec2::new(self.screen_position.x + self.screen_size.x * self.phase, self.screen_position.y + self.screen_size.y / 2.0);

        let mut selector_size_offset = Default::default();
        if self.selector_border_thickness != Default::default() {
            match &mut self.selector_border {
                ComponentBorderShape::Frame(selector_border) => {
                    selector_border.position = self.selector_position;
                    selector_border.size = self.selector_size;
                    selector_border.color = self.selector_border_color.clone();
                    selector_border.anchor = Vec2::new(0.5, 0.5);
                    selector_border.thickness = self.selector_border_thickness.into();
                    selector_border.update();
                }
                ComponentBorderShape::Circle(selector_border) => {
                    selector_border.position = self.selector_position;
                    selector_border.size = self.selector_size;
                    selector_border.color = self.selector_border_color.clone();
                    selector_border.anchor = Vec2::new(0.5, 0.5);
                    selector_border.thickness = Vec2::new(self.selector_border_thickness.left, self.selector_border_thickness.top);
                    selector_border.update();
                }
            }

            selector_size_offset = Vec2::new(
                self.selector_border_thickness.left + self.selector_border_thickness.right,
                self.selector_border_thickness.top + self.selector_border_thickness.bottom,
            );
        }

        match &mut self.selector {
            ComponentFillingShape::Rectangle(selector) => {
                selector.position = self.selector_position;
                selector.anchor = Vec2::new(0.5, 0.5);
                selector.color = self.selector_color.clone();
                selector.size = self.selector_size - selector_size_offset;
                selector.update();
            }
            ComponentFillingShape::Disc(selector) => {
                selector.position = self.selector_position;
                selector.anchor = Vec2::new(0.5, 0.5);
                selector.color = self.selector_color.clone();
                selector.size = self.selector_size - selector_size_offset;
                selector.update();
            }
        };

        if self.shadow_enabled {
            self.shadow.position = self.screen_position + self.screen_size / 2.0 + self.shadow_offset;
            self.shadow.size = self.screen_size;
            self.shadow.anchor = Vec2::new(0.5, 0.5);
            self.shadow.color = self.shadow_color.clone();
            self.shadow.scale = self.shadow_scale;
            self.shadow.corner_rounding = self.shadow_corner_rounding.into();

            self.shadow.update();
        }

        self.dirty = false;
        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        if self.shadow_enabled {
            renderer.draw(&mut self.shadow)?;
        }

        renderer.draw(&mut self.filling)?;
        renderer.draw(&mut self.bar)?;

        if self.border_thickness != Default::default() {
            renderer.draw(&mut self.border)?;
        }

        match &mut self.selector {
            ComponentFillingShape::Rectangle(selector) => renderer.draw(selector)?,
            ComponentFillingShape::Disc(selector) => renderer.draw(selector)?,
        }

        if self.selector_border_thickness != Default::default() {
            match &mut self.selector_border {
                ComponentBorderShape::Frame(selector_border) => renderer.draw(selector_border)?,
                ComponentBorderShape::Circle(selector_border) => renderer.draw(selector_border)?,
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
