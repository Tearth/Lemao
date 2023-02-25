use super::Component;
use super::ComponentBorderThickness;
use super::ComponentCornerRounding;
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
use lemao_core::renderer::drawable::circle::Circle;
use lemao_core::renderer::drawable::frame::Frame;
use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::textures::Texture;
use lemao_core::utils::storage::StorageItem;
use std::any::Any;

pub struct Slider {
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

    // Shape properties
    filling_id: usize,
    color: Color,
    corner_rounding: ComponentCornerRounding,
    texture_id: Option<usize>,
    texture_original_size: Vec2,

    // Border properties
    border_id: usize,
    border_color: Color,
    border_thickness: ComponentBorderThickness,

    // Shadow properties
    shadow_id: usize,
    shadow_enabled: bool,
    shadow_offset: Vec2,
    shadow_color: Color,
    shadow_scale: Vec2,
    shadow_corner_rounding: ComponentCornerRounding,

    // Bar properties
    bar_id: usize,
    bar_color: Color,

    // Selector properties
    selector_id: usize,
    selector_shape: ComponentShape,
    selector_position: Vec2,
    selector_size: Vec2,
    selector_color: Color,

    // Selector border properties
    selector_border_id: usize,
    selector_border_color: Color,
    selector_border_thickness: ComponentBorderThickness,

    // Component-specific properties
    phase: f32,
    phase_unrounded: f32,
    selector_pressed: bool,
    steps_count: u32,
    mouse_wheel_step: f32,

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
    pub fn new(id: usize, renderer: &mut RendererContext, selector_shape: ComponentShape) -> Result<Self, String> {
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

            // Shape properties
            filling_id: renderer.create_rectangle()?.get_id(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            corner_rounding: Default::default(),
            texture_id: None,
            texture_original_size: Default::default(),

            // Border properties
            border_id: renderer.create_frame()?.get_id(),
            border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            border_thickness: Default::default(),

            // Shadow properties
            shadow_id: renderer.create_rectangle()?.get_id(),
            shadow_enabled: false,
            shadow_offset: Default::default(),
            shadow_color: Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)),
            shadow_scale: Vec2::new(1.0, 1.0),
            shadow_corner_rounding: Default::default(),

            // Bar properties
            bar_id: renderer.create_rectangle()?.get_id(),
            bar_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Selector properties
            selector_id: match selector_shape {
                ComponentShape::Rectangle => renderer.create_rectangle()?.get_id(),
                ComponentShape::Disc => renderer.create_disc()?.get_id(),
            },
            selector_shape,
            selector_position: Default::default(),
            selector_size: Vec2::new(20.0, 20.0),
            selector_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Selector border properties
            selector_border_id: match selector_shape {
                ComponentShape::Rectangle => renderer.create_frame()?.get_id(),
                ComponentShape::Disc => renderer.create_circle()?.get_id(),
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

    pub fn get_id(&self) -> usize {
        self.id
    }

    /* #region Shape properties */
    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
        self.dirty = true;
    }

    pub fn get_corner_rounding(&self) -> ComponentCornerRounding {
        self.corner_rounding
    }

    pub fn set_corner_rounding(&mut self, corner_rounding: ComponentCornerRounding) {
        self.corner_rounding = corner_rounding;
        self.dirty = true;
    }

    pub fn get_texture_id(&self) -> Option<usize> {
        self.texture_id
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.texture_id = Some(texture.get_id());
        self.texture_original_size = texture.get_size();
        self.size = ComponentSize::Absolute(texture.get_size());
        self.dirty = true;
    }
    /* #endregion */

    /* #region Border properties */
    pub fn get_border_thickness(&self) -> ComponentBorderThickness {
        self.border_thickness
    }

    pub fn set_border_thickness(&mut self, border_thickness: ComponentBorderThickness) -> Result<(), String> {
        self.border_thickness = border_thickness;
        self.dirty = true;
        Ok(())
    }

    pub fn get_border_color(&self) -> &Color {
        &self.border_color
    }

    pub fn set_border_color(&mut self, border_color: Color) {
        self.border_color = border_color;
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

    pub fn get_shadow_scale(&self) -> Vec2 {
        self.shadow_scale
    }

    pub fn set_shadow_scale(&mut self, shadow_scale: Vec2) {
        self.shadow_scale = shadow_scale;
    }

    pub fn get_shadow_corner_rounding(&self) -> ComponentCornerRounding {
        self.shadow_corner_rounding
    }

    pub fn set_shadow_corner_rounding(&mut self, shadow_corner_rounding: ComponentCornerRounding) -> Result<(), String> {
        self.shadow_corner_rounding = shadow_corner_rounding;
        self.dirty = true;
        Ok(())
    }
    /* #endregion */

    /* #region Bar properties */
    pub fn get_bar_color(&self) -> &Color {
        &self.bar_color
    }

    pub fn set_bar_color(&mut self, bar_color: Color) {
        self.bar_color = bar_color;
        self.dirty = true;
    }
    /* #endregion */

    /* #region Selector properties */
    pub fn get_shape(&self) -> ComponentShape {
        self.selector_shape
    }

    pub fn get_selector_position(&self) -> Vec2 {
        self.selector_position
    }

    pub fn get_selector_size(&self) -> Vec2 {
        self.selector_size
    }

    pub fn set_selector_size(&mut self, selector_size: Vec2) {
        self.selector_size = selector_size;
        self.dirty = true;
    }

    pub fn get_selector_color(&self) -> &Color {
        &self.selector_color
    }

    pub fn set_selector_color(&mut self, selector_color: Color) {
        self.selector_color = selector_color;
        self.dirty = true;
    }
    /* #endregion */

    /* #region Selector border properties */
    pub fn get_selector_border_color(&self) -> &Color {
        &self.selector_border_color
    }

    pub fn set_selector_border_color(&mut self, selector_border_color: Color) {
        self.selector_border_color = selector_border_color;
        self.dirty = true;
    }

    pub fn get_selector_border_thickness(&self) -> ComponentBorderThickness {
        self.selector_border_thickness
    }

    pub fn set_selector_border_thickness(&mut self, selector_border_thickness: ComponentBorderThickness) {
        self.selector_border_thickness = selector_border_thickness;
        self.dirty = true;
    }
    /* #endregion */

    /* #region Component-specific properties */
    pub fn get_phase(&self) -> f32 {
        self.phase
    }

    pub fn set_phase(&mut self, phase: f32) {
        self.phase = phase;
        self.phase_unrounded = phase;
        self.dirty = true;
    }

    pub fn get_steps_count(&self) -> u32 {
        self.steps_count
    }

    pub fn set_steps_count(&mut self, steps_count: u32) {
        self.steps_count = steps_count;
        self.dirty = true;
    }

    pub fn get_mouse_wheel_step(&self) -> f32 {
        self.mouse_wheel_step
    }

    pub fn set_mouse_wheel_step(&mut self, mouse_wheel_step: f32) {
        self.mouse_wheel_step = mouse_wheel_step;
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

            if point.x < event_mask_left_bottom.x || point.y < event_mask_left_bottom.y || point.x > event_mask_right_top.x || point.y > event_mask_right_top.y
            {
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

                    let new_phase = (self.get_phase() + difference).clamp(0.0, 1.0);
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
            let border_rectangle = renderer.get_drawable_mut(self.border_id)?;
            border_rectangle.set_position(self.screen_position);
            border_rectangle.set_size(self.screen_size);
            border_rectangle.set_color(self.border_color.clone());

            renderer.get_drawable_and_cast_mut::<Frame>(self.border_id)?.set_thickness(self.border_thickness.into());

            self.screen_position += Vec2::new(self.border_thickness.left, self.border_thickness.bottom);
            self.screen_size -= Vec2::new(self.border_thickness.left + self.border_thickness.right, self.border_thickness.top + self.border_thickness.bottom);

            self.screen_size = self.screen_size.floor();
            self.screen_position = self.screen_position.floor();
        }

        let filling_rectangle = renderer.get_drawable_mut(self.filling_id)?;
        filling_rectangle.set_position(self.screen_position);
        filling_rectangle.set_color(self.color.clone());

        if let Some(texture_id) = self.texture_id {
            let texture_storage = renderer.get_textures();
            let texture_storage = texture_storage.read().unwrap();
            let texture = texture_storage.get_and_cast::<Texture>(texture_id)?;

            renderer.get_drawable_and_cast_mut::<Rectangle>(self.filling_id)?.set_texture(texture)
        }

        renderer.get_drawable_mut(self.filling_id)?.set_size(self.screen_size);

        let bar = renderer.get_drawable_mut(self.bar_id)?;
        bar.set_position(self.screen_position);
        bar.set_color(self.bar_color.clone());
        bar.set_size(self.screen_size * Vec2::new(self.phase, 1.0));

        self.selector_position = Vec2::new(self.screen_position.x + self.screen_size.x * self.phase, self.screen_position.y + self.screen_size.y / 2.0);
        let mut selector_size_offset = Default::default();

        if self.selector_border_thickness != Default::default() {
            let border_rectangle = renderer.get_drawable_mut(self.selector_border_id)?;
            border_rectangle.set_position(self.selector_position);
            border_rectangle.set_size(self.selector_size);
            border_rectangle.set_anchor(Vec2::new(0.5, 0.5));
            border_rectangle.set_color(self.selector_border_color.clone());

            match self.selector_shape {
                ComponentShape::Rectangle => {
                    renderer.get_drawable_and_cast_mut::<Frame>(self.selector_border_id)?.set_thickness(self.selector_border_thickness.into())
                }
                ComponentShape::Disc => renderer
                    .get_drawable_and_cast_mut::<Circle>(self.selector_border_id)?
                    .set_thickness(Vec2::new(self.selector_border_thickness.left, self.selector_border_thickness.top)),
            }

            selector_size_offset = Vec2::new(
                self.selector_border_thickness.left + self.selector_border_thickness.right,
                self.selector_border_thickness.top + self.selector_border_thickness.bottom,
            );
        }

        let selector = renderer.get_drawable_mut(self.selector_id)?;
        selector.set_position(self.selector_position);
        selector.set_anchor(Vec2::new(0.5, 0.5));
        selector.set_color(self.selector_color.clone());
        selector.set_size(self.selector_size - selector_size_offset);

        renderer.get_drawable_and_cast_mut::<Rectangle>(self.filling_id)?.set_corner_rounding(self.corner_rounding.into());
        renderer.get_drawable_and_cast_mut::<Rectangle>(self.bar_id)?.set_corner_rounding(self.corner_rounding.into());
        renderer.get_drawable_and_cast_mut::<Frame>(self.border_id)?.set_corner_rounding(self.corner_rounding.into());

        if self.shadow_enabled {
            let shadow = renderer.get_drawable_mut(self.shadow_id)?;
            shadow.set_position(self.screen_position + self.screen_size / 2.0 + self.shadow_offset);
            shadow.set_size(self.screen_size);
            shadow.set_anchor(Vec2::new(0.5, 0.5));
            shadow.set_color(self.shadow_color.clone());
            shadow.set_scale(self.shadow_scale);

            if let Ok(rectangle) = renderer.get_drawable_and_cast_mut::<Rectangle>(self.shadow_id) {
                rectangle.set_corner_rounding(self.shadow_corner_rounding.into());
            }
        }

        self.dirty = false;
        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        if self.shadow_enabled {
            renderer.draw(self.shadow_id)?;
        }

        renderer.draw(self.filling_id)?;
        renderer.draw(self.bar_id)?;

        if self.border_thickness != Default::default() {
            renderer.draw(self.border_id)?;
        }

        renderer.draw(self.selector_id)?;

        if self.selector_border_thickness != Default::default() {
            renderer.draw(self.selector_border_id)?;
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
        renderer.remove_drawable(self.filling_id)?;
        renderer.remove_drawable(self.border_id)?;
        renderer.remove_drawable(self.shadow_id)?;
        renderer.remove_drawable(self.bar_id)?;
        renderer.remove_drawable(self.selector_id)?;
        renderer.remove_drawable(self.selector_border_id)?;

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
