use super::Component;
use super::ComponentBorderThickness;
use super::ComponentCornerRounding;
use super::ComponentMargin;
use super::ComponentPosition;
use super::ComponentSize;
use super::EventMask;
use crate::events::UiEvent;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_common_platform::input::MouseWheelDirection;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::frame::Frame;
use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_core::renderer::drawable::Color;
use lemao_math::color::SolidColor;
use std::any::Any;

#[derive(Debug, Copy, Clone)]
pub enum ScrollboxOrientation {
    Vertical,
    Horizontal,
}

pub struct Scrollbox {
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

    // Vertical scroll background properties
    vertical_scroll_background_id: usize,
    vertical_scroll_background_color: Color,
    vertical_scroll_background_corner_rounding: ComponentCornerRounding,

    // Horizontal scroll background properties
    horizontal_scroll_background_id: usize,
    horizontal_scroll_background_color: Color,
    horizontal_scroll_background_corner_rounding: ComponentCornerRounding,

    // Vertical scroll background border properties
    vertical_scroll_background_border_id: usize,
    vertical_scroll_background_border_color: Color,
    vertical_scroll_background_border_thickness: ComponentBorderThickness,

    // Horizontal scroll background border properties
    horizontal_scroll_background_border_id: usize,
    horizontal_scroll_background_border_color: Color,
    horizontal_scroll_background_border_thickness: ComponentBorderThickness,

    // Vertical scroll properties
    vertical_scroll_id: usize,
    vertical_scroll_color: Color,
    vertical_scroll_corner_rounding: ComponentCornerRounding,

    // Horizontal scroll properties
    horizontal_scroll_id: usize,
    horizontal_scroll_color: Color,
    horizontal_scroll_corner_rounding: ComponentCornerRounding,

    // Vertical sScroll border properties
    vertical_scroll_border_id: usize,
    vertical_scroll_border_color: Color,
    vertical_scroll_border_thickness: ComponentBorderThickness,

    // Horizontal scroll border properties
    horizontal_scroll_border_id: usize,
    horizontal_scroll_border_color: Color,
    horizontal_scroll_border_thickness: ComponentBorderThickness,

    // Component-specific properties
    total_size: Vec2,
    padding: Vec2,
    scroll_difference: Vec2,
    scroll_delta: Vec2,
    scroll_speed: Vec2,
    scroll_width: Vec2,
    scroll_press_point_offset: Vec2,
    vertical_scroll_position: Vec2,
    vertical_scroll_size: Vec2,
    vertical_scroll_pressed: bool,
    horizontal_scroll_position: Vec2,
    horizontal_scroll_size: Vec2,
    horizontal_scroll_pressed: bool,
    mouse_wheel_mode: ScrollboxOrientation,

    // Event handlers
    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_scroll: Option<fn(component: &mut Self, orientation: ScrollboxOrientation, direction: f32)>,
    pub on_cursor_scroll_enter: Option<fn(component: &mut Self, orientation: ScrollboxOrientation, cursor_position: Vec2)>,
    pub on_cursor_scroll_leave: Option<fn(component: &mut Self, orientation: ScrollboxOrientation, cursor_position: Vec2)>,
    pub on_mouse_button_scroll_pressed: Option<fn(component: &mut Self, orientation: ScrollboxOrientation, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_scroll_released: Option<fn(component: &mut Self, orientation: ScrollboxOrientation, mouse_button: MouseButton, cursor_position: Vec2)>,
}

impl Scrollbox {
    pub fn new(id: usize, renderer: &mut RendererContext) -> Result<Self, String> {
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
            active: true,
            dirty: true,
            children: Default::default(),
            event_mask: None,

            // Vertical scroll background properties
            vertical_scroll_background_id: renderer.create_rectangle()?,
            vertical_scroll_background_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            vertical_scroll_background_corner_rounding: Default::default(),

            // Horizontal scroll background properties
            horizontal_scroll_background_id: renderer.create_rectangle()?,
            horizontal_scroll_background_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            horizontal_scroll_background_corner_rounding: Default::default(),

            // Vertical scroll background border properties
            vertical_scroll_background_border_id: renderer.create_frame(Default::default())?,
            vertical_scroll_background_border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            vertical_scroll_background_border_thickness: Default::default(),

            // Horizontal scroll background border properties
            horizontal_scroll_background_border_id: renderer.create_frame(Default::default())?,
            horizontal_scroll_background_border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            horizontal_scroll_background_border_thickness: Default::default(),

            // Vertical scroll properties
            vertical_scroll_id: renderer.create_rectangle()?,
            vertical_scroll_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            vertical_scroll_corner_rounding: Default::default(),

            // Horizontal scroll properties
            horizontal_scroll_id: renderer.create_rectangle()?,
            horizontal_scroll_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            horizontal_scroll_corner_rounding: Default::default(),

            // Vertical scroll border properties
            vertical_scroll_border_id: renderer.create_frame(Default::default())?,
            vertical_scroll_border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            vertical_scroll_border_thickness: Default::default(),

            // Horizontal scroll border properties
            horizontal_scroll_border_id: renderer.create_frame(Default::default())?,
            horizontal_scroll_border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            horizontal_scroll_border_thickness: Default::default(),

            // Component-specific properties
            total_size: Default::default(),
            padding: Default::default(),
            scroll_offset: Default::default(),
            scroll_difference: Default::default(),
            scroll_delta: Default::default(),
            scroll_speed: Vec2::new(5.0, 5.0),
            scroll_width: Vec2::new(20.0, 20.0),
            scroll_press_point_offset: Default::default(),
            vertical_scroll_position: Default::default(),
            vertical_scroll_size: Default::default(),
            vertical_scroll_pressed: false,
            horizontal_scroll_position: Default::default(),
            horizontal_scroll_size: Default::default(),
            horizontal_scroll_pressed: false,
            mouse_wheel_mode: ScrollboxOrientation::Vertical,

            // Event handlers
            on_cursor_enter: None,
            on_cursor_leave: None,
            on_mouse_button_pressed: None,
            on_mouse_button_released: None,
            on_scroll: None,
            on_cursor_scroll_enter: None,
            on_cursor_scroll_leave: None,
            on_mouse_button_scroll_pressed: None,
            on_mouse_button_scroll_released: None,
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    /* #region Vertical scroll properties */
    pub fn get_vertical_scroll_color(&self) -> &Color {
        &self.vertical_scroll_color
    }

    pub fn set_vertical_scroll_color(&mut self, vertical_scroll_color: Color) {
        self.vertical_scroll_color = vertical_scroll_color;
        self.dirty = true;
    }

    pub fn get_vertical_scroll_corner_rounding(&self) -> ComponentCornerRounding {
        self.vertical_scroll_corner_rounding
    }

    pub fn set_vertical_scroll_corner_rounding(&mut self, vertical_scroll_corner_rounding: ComponentCornerRounding) -> Result<(), String> {
        self.vertical_scroll_corner_rounding = vertical_scroll_corner_rounding;
        self.dirty = true;
        Ok(())
    }
    /* #endregion */

    /* #region Horizontal scroll properties */
    pub fn get_horizontal_scroll_color(&self) -> &Color {
        &self.horizontal_scroll_color
    }

    pub fn set_horizontal_scroll_color(&mut self, horizontal_scroll_color: Color) {
        self.horizontal_scroll_color = horizontal_scroll_color;
        self.dirty = true;
    }

    pub fn get_horizontal_scroll_corner_rounding(&self) -> ComponentCornerRounding {
        self.horizontal_scroll_corner_rounding
    }

    pub fn set_horizontal_scroll_corner_rounding(&mut self, horizontal_scroll_corner_rounding: ComponentCornerRounding) -> Result<(), String> {
        self.horizontal_scroll_corner_rounding = horizontal_scroll_corner_rounding;
        self.dirty = true;
        Ok(())
    }
    /* #endregion */

    /* #region Vertical scroll border properties */
    pub fn get_vertical_scroll_border_color(&self) -> &Color {
        &self.vertical_scroll_border_color
    }

    pub fn set_vertical_scroll_border_color(&mut self, vertical_scroll_border_color: Color) {
        self.vertical_scroll_border_color = vertical_scroll_border_color;
        self.dirty = true;
    }

    pub fn get_vertical_scroll_border_thickness(&self) -> ComponentBorderThickness {
        self.vertical_scroll_border_thickness
    }

    pub fn set_vertical_scroll_border_thickness(&mut self, vertical_scroll_border_thickness: ComponentBorderThickness) {
        self.vertical_scroll_border_thickness = vertical_scroll_border_thickness;
        self.dirty = true;
    }
    /* #endregion */

    /* #region Horizontal scroll border properties */
    pub fn get_horizontal_scroll_border_color(&self) -> &Color {
        &self.horizontal_scroll_border_color
    }

    pub fn set_horizontal_scroll_border_color(&mut self, horizontal_scroll_border_color: Color) {
        self.horizontal_scroll_border_color = horizontal_scroll_border_color;
        self.dirty = true;
    }

    pub fn get_horizontal_scroll_border_thickness(&self) -> ComponentBorderThickness {
        self.horizontal_scroll_border_thickness
    }

    pub fn set_horizontal_scroll_border_thickness(&mut self, horizontal_scroll_border_thickness: ComponentBorderThickness) {
        self.horizontal_scroll_border_thickness = horizontal_scroll_border_thickness;
        self.dirty = true;
    }
    /* #endregion */

    /* #region Vertical scroll background properties */
    pub fn get_vertical_scroll_background_color(&self) -> &Color {
        &self.vertical_scroll_background_color
    }

    pub fn set_vertical_scroll_background_color(&mut self, vertical_scroll_background_color: Color) {
        self.vertical_scroll_background_color = vertical_scroll_background_color;
        self.dirty = true;
    }

    pub fn set_vertical_scroll_background_corner_rounding(
        &mut self,
        vertical_scroll_background_corner_rounding: ComponentCornerRounding,
    ) -> Result<(), String> {
        self.vertical_scroll_background_corner_rounding = vertical_scroll_background_corner_rounding;
        self.dirty = true;
        Ok(())
    }

    pub fn get_vertical_scroll_background_corner_rounding(&self) -> ComponentCornerRounding {
        self.vertical_scroll_background_corner_rounding
    }
    /* #endregion */

    /* #region Horizontal scroll background properties */
    pub fn get_horizontal_scroll_background_color(&self) -> &Color {
        &self.horizontal_scroll_background_color
    }

    pub fn set_horizontal_scroll_background_color(&mut self, horizontal_scroll_background_color: Color) {
        self.horizontal_scroll_background_color = horizontal_scroll_background_color;
        self.dirty = true;
    }

    pub fn set_horizontal_scroll_background_corner_rounding(
        &mut self,
        horizontal_scroll_background_corner_rounding: ComponentCornerRounding,
    ) -> Result<(), String> {
        self.horizontal_scroll_background_corner_rounding = horizontal_scroll_background_corner_rounding;
        self.dirty = true;
        Ok(())
    }

    pub fn get_horizontal_scroll_background_corner_rounding(&self) -> ComponentCornerRounding {
        self.horizontal_scroll_background_corner_rounding
    }
    /* #endregion */

    /* #region Vertical scroll background border properties */
    pub fn get_vertical_scroll_background_border_color(&self) -> &Color {
        &self.vertical_scroll_background_border_color
    }

    pub fn set_vertical_scroll_background_border_color(&mut self, vertical_scroll_background_border_color: Color) {
        self.vertical_scroll_background_border_color = vertical_scroll_background_border_color;
        self.dirty = true;
    }

    pub fn get_vertical_scroll_background_border_thickness(&self) -> ComponentBorderThickness {
        self.vertical_scroll_background_border_thickness
    }

    pub fn set_vertical_scroll_background_border_thickness(&mut self, vertical_scroll_background_border_thickness: ComponentBorderThickness) {
        self.vertical_scroll_background_border_thickness = vertical_scroll_background_border_thickness;
        self.dirty = true;
    }
    /* #endregion */

    /* #region Horizontal scroll background border properties */
    pub fn get_horizontal_scroll_background_border_color(&self) -> &Color {
        &self.horizontal_scroll_background_border_color
    }

    pub fn set_horizontal_scroll_background_border_color(&mut self, horizontal_scroll_background_border_color: Color) {
        self.horizontal_scroll_background_border_color = horizontal_scroll_background_border_color;
        self.dirty = true;
    }

    pub fn get_horizontal_scroll_background_border_thickness(&self) -> ComponentBorderThickness {
        self.horizontal_scroll_background_border_thickness
    }

    pub fn set_horizontal_scroll_background_border_thickness(&mut self, horizontal_scroll_background_border_thickness: ComponentBorderThickness) {
        self.horizontal_scroll_background_border_thickness = horizontal_scroll_background_border_thickness;
        self.dirty = true;
    }
    /* #endregion */

    /* #region Component-specific properties */
    pub fn get_total_size(&self) -> Vec2 {
        self.total_size
    }

    pub(crate) fn set_total_size(&mut self, total_size: Vec2) {
        self.total_size = total_size + self.padding;
        self.dirty = true;
    }

    pub fn set_padding(&mut self, padding: Vec2) {
        self.padding = padding;
        self.dirty = true;
    }

    pub fn get_padding(&self) -> Vec2 {
        self.padding
    }

    pub fn get_scroll_delta(&self) -> Vec2 {
        Vec2::new(-self.scroll_delta.x, self.scroll_delta.y)
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

    pub fn get_scroll_width(&self) -> Vec2 {
        self.scroll_width
    }

    pub fn set_scroll_width(&mut self, scroll_width: Vec2) {
        self.scroll_width = scroll_width;
        self.dirty = true;
    }

    pub fn get_vertical_scroll_position(&self) -> Vec2 {
        self.vertical_scroll_position
    }

    pub fn get_vertical_scroll_size(&self) -> Vec2 {
        self.vertical_scroll_size
    }

    pub fn is_vertical_scroll_pressed(&self) -> bool {
        self.vertical_scroll_pressed
    }

    pub fn set_vertical_scroll_pressed_flag(&mut self, vertical_scroll_pressed: bool) {
        self.vertical_scroll_pressed = vertical_scroll_pressed;
        self.dirty = true;
    }

    pub fn get_horizontal_scroll_position(&self) -> Vec2 {
        self.horizontal_scroll_position
    }

    pub fn get_horizontal_scroll_size(&self) -> Vec2 {
        self.horizontal_scroll_size
    }

    pub fn is_horizontal_scroll_pressed(&self) -> bool {
        self.horizontal_scroll_pressed
    }

    pub fn set_horizontal_scroll_pressed_flag(&mut self, horizontal_scroll_pressed: bool) {
        self.horizontal_scroll_pressed = horizontal_scroll_pressed;
        self.dirty = true;
    }

    pub fn get_mouse_wheel_mode(&self) -> ScrollboxOrientation {
        self.mouse_wheel_mode
    }

    pub fn set_mouse_wheel_mode(&mut self, mouse_wheel_mode: ScrollboxOrientation) {
        self.mouse_wheel_mode = mouse_wheel_mode;
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

    fn is_point_inside_vertical_scroll(&self, point: Vec2) -> bool {
        if !self.active {
            return false;
        }

        let x1 = self.vertical_scroll_position.x - self.vertical_scroll_size.x;
        let y1 = self.vertical_scroll_position.y - self.vertical_scroll_size.y;
        let x2 = self.vertical_scroll_position.x;
        let y2 = self.vertical_scroll_position.y;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }

    fn is_point_inside_horizontal_scroll(&self, point: Vec2) -> bool {
        if !self.active {
            return false;
        }

        let x1 = self.horizontal_scroll_position.x;
        let y1 = self.horizontal_scroll_position.y;
        let x2 = self.horizontal_scroll_position.x + self.horizontal_scroll_size.x;
        let y2 = self.horizontal_scroll_position.y + self.horizontal_scroll_size.y;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }
}

impl Component for Scrollbox {
    /* #region Common properties */
    fn get_position(&self) -> ComponentPosition {
        self.position
    }

    fn get_work_area_position(&self) -> Vec2 {
        self.screen_position + Vec2::new(0.0, self.scroll_width.y)
    }

    fn set_position(&mut self, position: ComponentPosition) {
        self.position = position;
        self.dirty = true;
    }

    fn get_size(&self) -> ComponentSize {
        self.size
    }

    fn get_work_area_size(&self) -> Vec2 {
        self.screen_size - self.scroll_width
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
                    match self.mouse_wheel_mode {
                        ScrollboxOrientation::Vertical => {
                            let last_delta = self.scroll_delta;
                            let difference = match direction {
                                MouseWheelDirection::Up => -self.scroll_speed.y,
                                MouseWheelDirection::Down => self.scroll_speed.y,
                                _ => 0.0,
                            };

                            self.scroll_delta += Vec2::new(0.0, difference);
                            self.scroll_delta = self.scroll_delta.clamp(Vec2::new(0.0, 0.0), self.scroll_difference);

                            if self.scroll_delta != last_delta {
                                if let Some(f) = self.on_scroll {
                                    (f)(self, ScrollboxOrientation::Vertical, difference);
                                };
                                events.push(UiEvent::ScrollMoved(self.id, difference));
                                self.dirty = true;
                            }
                        }
                        ScrollboxOrientation::Horizontal => {
                            let last_delta = self.scroll_delta;
                            let difference = match direction {
                                MouseWheelDirection::Up => -self.scroll_speed.x,
                                MouseWheelDirection::Down => self.scroll_speed.x,
                                _ => 0.0,
                            };

                            self.scroll_delta += Vec2::new(difference, 0.0);
                            self.scroll_delta = self.scroll_delta.clamp(Vec2::new(0.0, 0.0), self.scroll_difference);

                            if self.scroll_delta != last_delta {
                                if let Some(f) = self.on_scroll {
                                    (f)(self, ScrollboxOrientation::Horizontal, difference);
                                };
                                events.push(UiEvent::ScrollMoved(self.id, difference));
                                self.dirty = true;
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        // Vertical scrollbar
        match event {
            InputEvent::MouseMoved(cursor_position, previous_cursor_position) => {
                if self.is_point_inside_vertical_scroll(*cursor_position) {
                    if !self.is_point_inside_vertical_scroll(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_scroll_enter {
                            (f)(self, ScrollboxOrientation::Vertical, *cursor_position);
                            self.dirty = true;
                        };

                        events.push(UiEvent::ScrollCursorEnter(self.id, *cursor_position));
                    }
                } else {
                    if self.is_point_inside_vertical_scroll(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_scroll_leave {
                            (f)(self, ScrollboxOrientation::Vertical, *cursor_position);
                            self.dirty = true;
                        };
                        events.push(UiEvent::ScrollCursorLeave(self.id, *cursor_position));
                    }
                }

                if self.vertical_scroll_pressed {
                    let scroll_ratio = (self.total_size.y + self.scroll_width.y) / (self.screen_size.y - self.scroll_width.y);
                    let last_delta = self.scroll_delta;

                    self.scroll_delta.y = (self.screen_position.y + self.screen_size.y - (cursor_position.y + self.scroll_press_point_offset.y)) * scroll_ratio;
                    self.scroll_delta = self.scroll_delta.clamp(Vec2::new(0.0, 0.0), self.scroll_difference);
                    let difference = self.scroll_delta.y - last_delta.y;

                    if self.scroll_delta != last_delta {
                        if let Some(f) = self.on_scroll {
                            (f)(self, ScrollboxOrientation::Vertical, difference);
                            self.dirty = true;
                        };
                        events.push(UiEvent::ScrollMoved(self.id, difference));
                        self.dirty = true;
                    }
                }
            }
            InputEvent::MouseButtonPressed(button, cursor_position) => {
                if self.is_point_inside_vertical_scroll(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_scroll_pressed {
                        (f)(self, ScrollboxOrientation::Vertical, *button, *cursor_position);
                        self.dirty = true;
                    };
                    events.push(UiEvent::ScrollMouseButtonPressed(self.id, *button));
                    self.vertical_scroll_pressed = true;
                    self.scroll_press_point_offset = self.vertical_scroll_position - *cursor_position;
                }
            }
            InputEvent::MouseButtonReleased(button, cursor_position) => {
                if self.is_point_inside_vertical_scroll(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_scroll_released {
                        (f)(self, ScrollboxOrientation::Vertical, *button, *cursor_position);
                        self.dirty = true;
                    };
                    events.push(UiEvent::ScrollMouseButtonReleased(self.id, *button));
                }

                self.vertical_scroll_pressed = false;
            }
            _ => {}
        }

        // Horizontal scrollbar
        match event {
            InputEvent::MouseMoved(cursor_position, previous_cursor_position) => {
                if self.is_point_inside_horizontal_scroll(*cursor_position) {
                    if !self.is_point_inside_horizontal_scroll(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_scroll_enter {
                            (f)(self, ScrollboxOrientation::Horizontal, *cursor_position);
                            self.dirty = true;
                        };

                        events.push(UiEvent::ScrollCursorEnter(self.id, *cursor_position));
                    }
                } else {
                    if self.is_point_inside_horizontal_scroll(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_scroll_leave {
                            (f)(self, ScrollboxOrientation::Horizontal, *cursor_position);
                            self.dirty = true;
                        };
                        events.push(UiEvent::ScrollCursorLeave(self.id, *cursor_position));
                    }
                }

                if self.horizontal_scroll_pressed {
                    let scroll_ratio = (self.total_size.x + self.scroll_width.x) / (self.screen_size.x - self.scroll_width.x);
                    let last_delta = self.scroll_delta;

                    self.scroll_delta.x = ((cursor_position.x + self.scroll_press_point_offset.x) - self.screen_position.x) * scroll_ratio;
                    self.scroll_delta = self.scroll_delta.clamp(Vec2::new(0.0, 0.0), self.scroll_difference);
                    let difference = self.scroll_delta.x - last_delta.x;

                    if self.scroll_delta != last_delta {
                        if let Some(f) = self.on_scroll {
                            (f)(self, ScrollboxOrientation::Horizontal, difference);
                            self.dirty = true;
                        };
                        events.push(UiEvent::ScrollMoved(self.id, difference));
                        self.dirty = true;
                    }
                }
            }
            InputEvent::MouseButtonPressed(button, cursor_position) => {
                if self.is_point_inside_horizontal_scroll(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_scroll_pressed {
                        (f)(self, ScrollboxOrientation::Horizontal, *button, *cursor_position);
                        self.dirty = true;
                    };
                    events.push(UiEvent::ScrollMouseButtonPressed(self.id, *button));
                    self.horizontal_scroll_pressed = true;
                    self.scroll_press_point_offset = self.horizontal_scroll_position - *cursor_position;
                }
            }
            InputEvent::MouseButtonReleased(button, cursor_position) => {
                if self.is_point_inside_horizontal_scroll(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_scroll_released {
                        (f)(self, ScrollboxOrientation::Horizontal, *button, *cursor_position);
                        self.dirty = true;
                    };
                    events.push(UiEvent::ScrollMouseButtonReleased(self.id, *button));
                }

                self.horizontal_scroll_pressed = false;
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

        /* #region Vertical scroll */
        let mut vertical_scroll_background_position = self.screen_position + self.screen_size;
        let mut vertical_scroll_background_size = Vec2::new(self.scroll_width.x, self.screen_size.y - self.scroll_width.y);

        if self.vertical_scroll_background_border_thickness != Default::default() {
            let border_rectangle = renderer.get_drawable_mut(self.vertical_scroll_background_border_id)?;
            border_rectangle.set_position(vertical_scroll_background_position);
            border_rectangle.set_size(vertical_scroll_background_size);
            border_rectangle.set_anchor(Vec2::new(1.0, 1.0));
            border_rectangle.set_color(self.vertical_scroll_background_border_color.clone());

            renderer
                .get_drawable_with_type_mut::<Frame>(self.vertical_scroll_background_border_id)?
                .set_thickness(self.vertical_scroll_background_border_thickness.into());

            vertical_scroll_background_position -=
                Vec2::new(self.vertical_scroll_background_border_thickness.right, self.vertical_scroll_background_border_thickness.top);
            vertical_scroll_background_size -= Vec2::new(
                self.vertical_scroll_background_border_thickness.left + self.vertical_scroll_background_border_thickness.right,
                self.vertical_scroll_background_border_thickness.top + self.vertical_scroll_background_border_thickness.bottom,
            );
        }

        let vertical_scroll_background = renderer.get_drawable_mut(self.vertical_scroll_background_id)?;
        vertical_scroll_background.set_position(vertical_scroll_background_position);
        vertical_scroll_background.set_size(vertical_scroll_background_size);
        vertical_scroll_background.set_anchor(Vec2::new(1.0, 1.0));
        vertical_scroll_background.set_color(self.vertical_scroll_background_color.clone());

        self.scroll_difference.y = (self.total_size.y + self.scroll_width.y - self.screen_size.y).clamp(0.0, f32::MAX);

        if self.scroll_difference.y > 0.0 {
            let vertical_scroll_height = ((self.screen_size.y - self.scroll_width.y) * self.screen_size.y / (self.total_size.y + self.scroll_width.y)).floor();
            let vertical_scroll_free_space = self.screen_size.y - self.scroll_width.y - vertical_scroll_height;
            let vertical_scroll_offset = (vertical_scroll_free_space * self.scroll_delta.y / self.scroll_difference.y).floor();

            self.vertical_scroll_position = self.screen_position + self.screen_size - Vec2::new(0.0, vertical_scroll_offset);
            self.vertical_scroll_size = Vec2::new(self.scroll_width.x, vertical_scroll_height);
        } else {
            self.vertical_scroll_position = self.screen_position + self.screen_size;
            self.vertical_scroll_size = Vec2::new(self.scroll_width.x, self.screen_size.y - self.scroll_width.y);
        }

        if self.vertical_scroll_border_thickness != Default::default() {
            let border_rectangle = renderer.get_drawable_mut(self.vertical_scroll_border_id)?;
            border_rectangle.set_position(self.vertical_scroll_position);
            border_rectangle.set_size(self.vertical_scroll_size);
            border_rectangle.set_anchor(Vec2::new(1.0, 1.0));
            border_rectangle.set_color(self.vertical_scroll_border_color.clone());

            renderer.get_drawable_with_type_mut::<Frame>(self.vertical_scroll_border_id)?.set_thickness(self.vertical_scroll_border_thickness.into());

            self.vertical_scroll_position -= Vec2::new(self.vertical_scroll_border_thickness.right, self.vertical_scroll_border_thickness.top);
            self.vertical_scroll_size -= Vec2::new(
                self.vertical_scroll_border_thickness.left + self.vertical_scroll_border_thickness.right,
                self.vertical_scroll_border_thickness.top + self.vertical_scroll_border_thickness.bottom,
            );
        }

        let vertical_scroll = renderer.get_drawable_mut(self.vertical_scroll_id)?;
        vertical_scroll.set_position(self.vertical_scroll_position);
        vertical_scroll.set_size(self.vertical_scroll_size);
        vertical_scroll.set_anchor(Vec2::new(1.0, 1.0));
        vertical_scroll.set_color(self.vertical_scroll_color.clone());

        renderer.get_drawable_with_type_mut::<Rectangle>(self.vertical_scroll_id)?.set_corner_rounding(self.vertical_scroll_corner_rounding.into());
        renderer.get_drawable_with_type_mut::<Frame>(self.vertical_scroll_border_id)?.set_corner_rounding(self.vertical_scroll_corner_rounding.into());
        renderer
            .get_drawable_with_type_mut::<Rectangle>(self.vertical_scroll_background_id)?
            .set_corner_rounding(self.vertical_scroll_background_corner_rounding.into());
        renderer
            .get_drawable_with_type_mut::<Frame>(self.vertical_scroll_background_border_id)?
            .set_corner_rounding(self.vertical_scroll_background_corner_rounding.into());
        /* #endregion */

        /* #region Horizontal scroll */
        let mut horizontal_scroll_background_position = self.screen_position;
        let mut horizontal_scroll_background_size = Vec2::new(self.screen_size.x - self.scroll_width.x, self.scroll_width.y);

        if self.horizontal_scroll_background_border_thickness != Default::default() {
            let border_rectangle = renderer.get_drawable_mut(self.horizontal_scroll_background_border_id)?;
            border_rectangle.set_position(horizontal_scroll_background_position);
            border_rectangle.set_size(horizontal_scroll_background_size);
            border_rectangle.set_color(self.horizontal_scroll_background_border_color.clone());

            renderer
                .get_drawable_with_type_mut::<Frame>(self.horizontal_scroll_background_border_id)?
                .set_thickness(self.horizontal_scroll_background_border_thickness.into());

            horizontal_scroll_background_position +=
                Vec2::new(self.horizontal_scroll_background_border_thickness.right, self.horizontal_scroll_background_border_thickness.top);
            horizontal_scroll_background_size -= Vec2::new(
                self.horizontal_scroll_background_border_thickness.left + self.horizontal_scroll_background_border_thickness.right,
                self.horizontal_scroll_background_border_thickness.top + self.horizontal_scroll_background_border_thickness.bottom,
            );
        }

        let horizontal_scroll_background = renderer.get_drawable_mut(self.horizontal_scroll_background_id)?;
        horizontal_scroll_background.set_position(horizontal_scroll_background_position);
        horizontal_scroll_background.set_size(horizontal_scroll_background_size);
        horizontal_scroll_background.set_color(self.horizontal_scroll_background_color.clone());

        self.scroll_difference.x = (self.total_size.x + self.scroll_width.x - self.screen_size.x).clamp(0.0, f32::MAX);

        if self.scroll_difference.x > 0.0 {
            let horizontal_scroll_width = ((self.screen_size.x - self.scroll_width.x) * self.screen_size.x / (self.total_size.x + self.scroll_width.x)).floor();
            let horizontal_scroll_free_space = self.screen_size.x - self.scroll_width.x - horizontal_scroll_width;
            let horizontal_scroll_offset = (horizontal_scroll_free_space * self.scroll_delta.x / self.scroll_difference.x).floor();

            self.horizontal_scroll_position = self.screen_position + Vec2::new(horizontal_scroll_offset, 0.0);
            self.horizontal_scroll_size = Vec2::new(horizontal_scroll_width, self.scroll_width.y);
        } else {
            self.horizontal_scroll_position = self.screen_position;
            self.horizontal_scroll_size = Vec2::new(self.screen_size.x - self.scroll_width.x, self.scroll_width.y);
        }

        if self.horizontal_scroll_border_thickness != Default::default() {
            let border_rectangle = renderer.get_drawable_mut(self.horizontal_scroll_border_id)?;
            border_rectangle.set_position(self.horizontal_scroll_position);
            border_rectangle.set_size(self.horizontal_scroll_size);
            border_rectangle.set_color(self.horizontal_scroll_border_color.clone());

            renderer.get_drawable_with_type_mut::<Frame>(self.horizontal_scroll_border_id)?.set_thickness(self.horizontal_scroll_border_thickness.into());

            self.horizontal_scroll_position += Vec2::new(self.horizontal_scroll_border_thickness.right, self.horizontal_scroll_border_thickness.top);
            self.horizontal_scroll_size -= Vec2::new(
                self.horizontal_scroll_border_thickness.left + self.horizontal_scroll_border_thickness.right,
                self.horizontal_scroll_border_thickness.top + self.horizontal_scroll_border_thickness.bottom,
            );
        }

        let horizontal_scroll = renderer.get_drawable_mut(self.horizontal_scroll_id)?;
        horizontal_scroll.set_position(self.horizontal_scroll_position);
        horizontal_scroll.set_size(self.horizontal_scroll_size);
        horizontal_scroll.set_color(self.horizontal_scroll_color.clone());

        renderer.get_drawable_with_type_mut::<Rectangle>(self.horizontal_scroll_id)?.set_corner_rounding(self.horizontal_scroll_corner_rounding.into());
        renderer.get_drawable_with_type_mut::<Frame>(self.horizontal_scroll_border_id)?.set_corner_rounding(self.horizontal_scroll_corner_rounding.into());
        renderer
            .get_drawable_with_type_mut::<Rectangle>(self.horizontal_scroll_background_id)?
            .set_corner_rounding(self.horizontal_scroll_background_corner_rounding.into());
        renderer
            .get_drawable_with_type_mut::<Frame>(self.horizontal_scroll_background_border_id)?
            .set_corner_rounding(self.horizontal_scroll_background_corner_rounding.into());
        /* #endregion */

        self.dirty = false;

        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        if self.scroll_width.x > 0.0 {
            renderer.draw(self.vertical_scroll_background_id)?;
            renderer.draw(self.vertical_scroll_id)?;

            if self.vertical_scroll_background_border_thickness != Default::default() {
                renderer.draw(self.vertical_scroll_background_border_id)?;
            }

            if self.vertical_scroll_border_thickness != Default::default() {
                renderer.draw(self.vertical_scroll_border_id)?;
            }
        }

        if self.scroll_width.y > 0.0 {
            renderer.draw(self.horizontal_scroll_background_id)?;
            renderer.draw(self.horizontal_scroll_id)?;

            if self.horizontal_scroll_background_border_thickness != Default::default() {
                renderer.draw(self.horizontal_scroll_background_border_id)?;
            }

            if self.horizontal_scroll_border_thickness != Default::default() {
                renderer.draw(self.horizontal_scroll_border_id)?;
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

    fn release_internal_resources(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        renderer.remove_drawable(self.vertical_scroll_id)?;
        renderer.remove_drawable(self.vertical_scroll_border_id)?;
        renderer.remove_drawable(self.vertical_scroll_background_id)?;
        renderer.remove_drawable(self.vertical_scroll_background_border_id)?;
        renderer.remove_drawable(self.horizontal_scroll_id)?;
        renderer.remove_drawable(self.horizontal_scroll_border_id)?;
        renderer.remove_drawable(self.horizontal_scroll_background_id)?;
        renderer.remove_drawable(self.horizontal_scroll_background_border_id)?;

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
