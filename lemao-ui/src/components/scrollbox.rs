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
use lemao_core::renderer::textures::Texture;
use lemao_math::color::SolidColor;
use std::any::Any;

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
    dirty: bool,
    children: Vec<usize>,
    event_mask: Option<EventMask>,

    // Scroll background properties
    scroll_background_id: usize,
    scroll_background_color: Color,
    scroll_background_corner_rounding: ComponentCornerRounding,
    scroll_background_texture_id: Option<usize>,

    // Scroll background border properties
    scroll_background_border_id: usize,
    scroll_background_border_color: Color,
    scroll_background_border_thickness: ComponentBorderThickness,

    // Scroll properties
    scroll_id: usize,
    scroll_color: Color,
    scroll_corner_rounding: ComponentCornerRounding,
    scroll_texture_id: Option<usize>,

    // Scroll border properties
    scroll_border_id: usize,
    scroll_border_color: Color,
    scroll_border_thickness: ComponentBorderThickness,

    // Component-specific properties
    total_size: Vec2,
    padding: Vec2,
    scroll_position: Vec2,
    scroll_size: Vec2,
    scroll_difference: Vec2,
    scroll_delta: Vec2,
    scroll_speed: Vec2,
    scroll_width: f32,
    scroll_pressed: bool,

    // Event handlers
    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_scroll: Option<fn(component: &mut Self, direction: MouseWheelDirection)>,
    pub on_cursor_scroll_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_scroll_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_scroll_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_scroll_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
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
            dirty: true,
            children: Default::default(),
            event_mask: None,

            // Scroll background properties
            scroll_background_id: renderer.create_rectangle()?,
            scroll_background_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            scroll_background_corner_rounding: Default::default(),
            scroll_background_texture_id: None,

            // Scroll background border properties
            scroll_background_border_id: renderer.create_frame(Default::default())?,
            scroll_background_border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            scroll_background_border_thickness: Default::default(),

            // Scroll properties
            scroll_id: renderer.create_rectangle()?,
            scroll_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            scroll_corner_rounding: Default::default(),
            scroll_texture_id: None,

            // Scroll border properties
            scroll_border_id: renderer.create_frame(Default::default())?,
            scroll_border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            scroll_border_thickness: Default::default(),

            // Component-specific properties
            total_size: Default::default(),
            padding: Default::default(),
            scroll_position: Default::default(),
            scroll_size: Default::default(),
            scroll_offset: Default::default(),
            scroll_difference: Default::default(),
            scroll_delta: Default::default(),
            scroll_speed: Vec2::new(5.0, 5.0),
            scroll_width: 20.0,
            scroll_pressed: false,

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

    /* #region Scroll properties */
    pub fn get_scroll_color(&self) -> &Color {
        &self.scroll_color
    }

    pub fn set_scroll_color(&mut self, scroll_color: Color) {
        self.scroll_color = scroll_color;
        self.dirty = true;
    }

    pub fn get_scroll_corner_rounding(&self) -> ComponentCornerRounding {
        self.scroll_corner_rounding
    }

    pub fn set_scroll_corner_rounding(&mut self, scroll_corner_rounding: ComponentCornerRounding) -> Result<(), String> {
        self.scroll_corner_rounding = scroll_corner_rounding;
        self.dirty = true;
        Ok(())
    }

    pub fn get_scroll_texture_id(&self) -> Option<usize> {
        self.scroll_texture_id
    }

    pub fn set_scroll_texture_id(&mut self, scroll_texture: &Texture) {
        self.scroll_texture_id = Some(scroll_texture.get_id());
        self.dirty = true;
    }
    /* #endregion */

    /* #region Scroll border properties */
    pub fn get_scroll_border_color(&self) -> &Color {
        &self.scroll_border_color
    }

    pub fn set_scroll_border_color(&mut self, scroll_border_color: Color) {
        self.scroll_border_color = scroll_border_color;
        self.dirty = true;
    }

    pub fn get_scroll_border_thickness(&self) -> ComponentBorderThickness {
        self.scroll_border_thickness
    }

    pub fn set_scroll_border_thickness(&mut self, scroll_border_thickness: ComponentBorderThickness) {
        self.scroll_border_thickness = scroll_border_thickness;
        self.dirty = true;
    }
    /* #endregion */

    /* #region Scroll background properties */
    pub fn get_scroll_background_color(&self) -> &Color {
        &self.scroll_background_color
    }

    pub fn set_scroll_background_color(&mut self, scroll_background_color: Color) {
        self.scroll_background_color = scroll_background_color;
        self.dirty = true;
    }

    pub fn set_scroll_background_corner_rounding(&mut self, scroll_background_corner_rounding: ComponentCornerRounding) -> Result<(), String> {
        self.scroll_background_corner_rounding = scroll_background_corner_rounding;
        self.dirty = true;
        Ok(())
    }

    pub fn get_scroll_background_corner_rounding(&self) -> ComponentCornerRounding {
        self.scroll_background_corner_rounding
    }

    pub fn get_scroll_background_texture_id(&self) -> Option<usize> {
        self.scroll_background_texture_id
    }

    pub fn set_scroll_background_texture_id(&mut self, scroll_background_texture: &Texture) {
        self.scroll_background_texture_id = Some(scroll_background_texture.get_id());
        self.dirty = true;
    }
    /* #endregion */

    /* #region Scroll background border properties */
    pub fn get_scroll_background_border_color(&self) -> &Color {
        &self.scroll_background_border_color
    }

    pub fn set_scroll_background_border_color(&mut self, scroll_background_border_color: Color) {
        self.scroll_background_border_color = scroll_background_border_color;
        self.dirty = true;
    }

    pub fn get_scroll_background_border_thickness(&self) -> ComponentBorderThickness {
        self.scroll_background_border_thickness
    }

    pub fn set_scroll_background_border_thickness(&mut self, scroll_background_border_thickness: ComponentBorderThickness) {
        self.scroll_background_border_thickness = scroll_background_border_thickness;
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

    pub fn get_scroll_position(&self) -> Vec2 {
        self.scroll_position
    }

    pub fn get_scroll_size(&self) -> Vec2 {
        self.scroll_size
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

    pub fn get_scroll_width(&self) -> f32 {
        self.scroll_width
    }

    pub fn set_scroll_width(&mut self, scroll_width: f32) {
        self.scroll_width = scroll_width;
        self.dirty = true;
    }

    pub fn is_scroll_pressed(&self) -> bool {
        self.scroll_pressed
    }

    pub fn set_scroll_pressed_flag(&mut self, scroll_pressed: bool) {
        self.scroll_pressed = scroll_pressed;
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
        let x2 = self.screen_position.x + self.screen_size.x;
        let y2 = self.screen_position.y + self.screen_size.y;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }

    fn is_point_inside_scroll(&self, point: Vec2) -> bool {
        if let Some(event_mask) = self.event_mask {
            let event_mask_left_bottom = event_mask.position;
            let event_mask_right_top = event_mask.position + event_mask.size;

            if point.x < event_mask_left_bottom.x || point.y < event_mask_left_bottom.y || point.x > event_mask_right_top.x || point.y > event_mask_right_top.y
            {
                return false;
            }
        }

        let x1 = self.scroll_position.x - self.scroll_size.x;
        let y1 = self.scroll_position.y - self.scroll_size.y;
        let x2 = self.scroll_position.x;
        let y2 = self.scroll_position.y;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }
}

impl Component for Scrollbox {
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
                    let last_delta = self.scroll_delta;

                    match direction {
                        MouseWheelDirection::Up => self.scroll_delta -= Vec2::new(0.0, self.scroll_speed.y),
                        MouseWheelDirection::Down => self.scroll_delta += Vec2::new(0.0, self.scroll_speed.y),
                        _ => {}
                    };

                    self.scroll_delta = self.scroll_delta.clamp(Vec2::new(0.0, 0.0), self.scroll_difference);

                    if self.scroll_delta != last_delta {
                        if let Some(f) = self.on_scroll {
                            (f)(self, *direction);
                        };
                        events.push(UiEvent::ScrollMoved(self.id, *direction));
                        self.dirty = true;
                    }
                }
            }
            _ => {}
        }

        // Scrollbar
        match event {
            InputEvent::MouseMoved(cursor_position, previous_cursor_position) => {
                if self.is_point_inside_scroll(*cursor_position) {
                    if !self.is_point_inside_scroll(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_scroll_enter {
                            (f)(self, *cursor_position);
                            self.dirty = true;
                        };

                        events.push(UiEvent::ScrollCursorEnter(self.id, *cursor_position));
                    }
                } else {
                    if self.is_point_inside_scroll(*previous_cursor_position) {
                        if let Some(f) = self.on_cursor_scroll_leave {
                            (f)(self, *cursor_position);
                            self.dirty = true;
                        };
                        events.push(UiEvent::ScrollCursorLeave(self.id, *cursor_position));
                    }
                }

                if self.scroll_pressed {
                    let scroll_ratio = self.total_size.y / self.screen_size.y;
                    let difference = (previous_cursor_position.y - cursor_position.y) * scroll_ratio;
                    let last_delta = self.scroll_delta;

                    self.scroll_delta += Vec2::new(0.0, difference);
                    self.scroll_delta = self.scroll_delta.clamp(Vec2::new(0.0, 0.0), self.scroll_difference);

                    if self.scroll_delta != last_delta {
                        let direction = if difference > 0.0 { MouseWheelDirection::Down } else { MouseWheelDirection::Up };
                        if let Some(f) = self.on_scroll {
                            (f)(self, direction);
                            self.dirty = true;
                        };
                        events.push(UiEvent::ScrollMoved(self.id, direction));
                        self.dirty = true;
                    }
                }
            }
            InputEvent::MouseButtonPressed(button, cursor_position) => {
                if self.is_point_inside_scroll(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_scroll_pressed {
                        (f)(self, *button, *cursor_position);
                        self.dirty = true;
                    };
                    events.push(UiEvent::ScrollMouseButtonPressed(self.id, *button));
                    self.scroll_pressed = true;
                }
            }
            InputEvent::MouseButtonReleased(button, cursor_position) => {
                if self.is_point_inside_scroll(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_scroll_released {
                        (f)(self, *button, *cursor_position);
                        self.dirty = true;
                    };
                    events.push(UiEvent::ScrollMouseButtonReleased(self.id, *button));
                }

                self.scroll_pressed = false;
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

        let mut scroll_background_position = self.screen_position + self.screen_size;
        let mut scroll_background_size = Vec2::new(self.scroll_width, self.screen_size.y);

        if self.scroll_background_border_thickness != Default::default() {
            let border_rectangle = renderer.get_drawable_mut(self.scroll_background_border_id)?;
            border_rectangle.set_position(scroll_background_position);
            border_rectangle.set_size(scroll_background_size);
            border_rectangle.set_anchor(Vec2::new(1.0, 1.0));
            border_rectangle.set_color(self.scroll_background_border_color.clone());

            renderer.get_drawable_with_type_mut::<Frame>(self.scroll_background_border_id)?.set_thickness(self.scroll_background_border_thickness.into());

            scroll_background_position -= Vec2::new(self.scroll_background_border_thickness.right, self.scroll_background_border_thickness.top);
            scroll_background_size -= Vec2::new(
                self.scroll_background_border_thickness.left + self.scroll_background_border_thickness.right,
                self.scroll_background_border_thickness.top + self.scroll_background_border_thickness.bottom,
            );
        }

        let scroll_background = renderer.get_drawable_mut(self.scroll_background_id)?;
        scroll_background.set_position(scroll_background_position);
        scroll_background.set_size(scroll_background_size);
        scroll_background.set_anchor(Vec2::new(1.0, 1.0));
        scroll_background.set_color(self.scroll_background_color.clone());

        if let Some(scroll_background_texture_id) = self.scroll_background_texture_id {
            let texture_storage = renderer.get_textures();
            let texture_storage_lock = texture_storage.lock().unwrap();
            let texture = texture_storage_lock.get(scroll_background_texture_id)?;

            renderer.get_drawable_with_type_mut::<Rectangle>(self.scroll_background_id)?.set_texture(texture);
        }

        self.scroll_difference = (self.total_size - self.screen_size).clamp(Vec2::new(0.0, 0.0), Vec2::new(f32::MAX, f32::MAX));

        let scroll_height = (self.screen_size.y * self.screen_size.y / self.total_size.y).floor();
        let scroll_free_space_left = self.screen_size.y - scroll_height;
        let scroll_offset = (scroll_free_space_left * self.scroll_delta.y / self.scroll_difference.y).floor();

        self.scroll_position = self.screen_position + self.screen_size - Vec2::new(0.0, scroll_offset);
        self.scroll_size = Vec2::new(self.scroll_width, scroll_height);

        if self.scroll_border_thickness != Default::default() {
            let border_rectangle = renderer.get_drawable_mut(self.scroll_border_id)?;
            border_rectangle.set_position(self.scroll_position);
            border_rectangle.set_size(self.scroll_size);
            border_rectangle.set_anchor(Vec2::new(1.0, 1.0));
            border_rectangle.set_color(self.scroll_border_color.clone());

            renderer.get_drawable_with_type_mut::<Frame>(self.scroll_border_id)?.set_thickness(self.scroll_border_thickness.into());

            self.scroll_position -= Vec2::new(self.scroll_border_thickness.right, self.scroll_border_thickness.top);
            self.scroll_size -= Vec2::new(
                self.scroll_border_thickness.left + self.scroll_border_thickness.right,
                self.scroll_border_thickness.top + self.scroll_border_thickness.bottom,
            );
        }

        let scroll = renderer.get_drawable_mut(self.scroll_id)?;
        scroll.set_position(self.scroll_position);
        scroll.set_size(self.scroll_size);
        scroll.set_anchor(Vec2::new(1.0, 1.0));
        scroll.set_color(self.scroll_color.clone());

        if let Some(scroll_texture_id) = self.scroll_texture_id {
            let texture_storage = renderer.get_textures();
            let texture_storage_lock = texture_storage.lock().unwrap();
            let texture = texture_storage_lock.get(scroll_texture_id)?;

            renderer.get_drawable_with_type_mut::<Rectangle>(self.scroll_id)?.set_texture(texture);
        }

        renderer.get_drawable_with_type_mut::<Rectangle>(self.scroll_id)?.set_corner_rounding(self.scroll_corner_rounding.into());
        renderer.get_drawable_with_type_mut::<Frame>(self.scroll_border_id)?.set_corner_rounding(self.scroll_corner_rounding.into());
        renderer.get_drawable_with_type_mut::<Rectangle>(self.scroll_background_id)?.set_corner_rounding(self.scroll_background_corner_rounding.into());
        renderer.get_drawable_with_type_mut::<Frame>(self.scroll_background_border_id)?.set_corner_rounding(self.scroll_background_corner_rounding.into());

        self.dirty = false;

        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        renderer.draw(self.scroll_background_id)?;
        renderer.draw(self.scroll_id)?;

        if self.scroll_background_border_thickness != Default::default() {
            renderer.draw(self.scroll_background_border_id)?;
        }

        if self.scroll_border_thickness != Default::default() {
            renderer.draw(self.scroll_border_id)?;
        }

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
