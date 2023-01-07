use crate::events::UiEvent;

use super::Component;
use super::ComponentBorderThickness;
use super::ComponentMargin;
use super::ComponentPosition;
use super::ComponentShape;
use super::ComponentSize;
use super::EventMask;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::circle::Circle;
use lemao_core::renderer::drawable::disc::Disc;
use lemao_core::renderer::drawable::frame::Frame;
use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::textures::Texture;
use std::any::Any;

pub struct Panel {
    pub(crate) id: usize,

    position: ComponentPosition,
    screen_position: Vec2,
    size: ComponentSize,
    screen_size: Vec2,
    min_size: Vec2,
    max_size: Vec2,
    shape: ComponentShape,
    anchor: Vec2,
    margin: ComponentMargin,
    offset: Vec2,
    scroll_offset: Vec2,
    color: Color,
    border_thickness: ComponentBorderThickness,
    border_color: Color,
    roundness_factor: f32,
    texture_id: Option<usize>,
    texture_original_size: Vec2,
    filling_id: usize,
    border_id: usize,
    children: Vec<usize>,
    dirty: bool,
    event_mask: Option<EventMask>,

    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
}

impl Panel {
    pub fn new(id: usize, renderer: &mut RendererContext, shape: ComponentShape) -> Result<Self, String> {
        Ok(Self {
            id,
            position: ComponentPosition::AbsoluteToParent(Default::default()),
            screen_position: Default::default(),
            size: ComponentSize::Absolute(Default::default()),
            screen_size: Default::default(),
            min_size: Vec2::new(f32::MIN, f32::MIN),
            max_size: Vec2::new(f32::MAX, f32::MAX),
            shape,
            anchor: Default::default(),
            margin: Default::default(),
            offset: Default::default(),
            scroll_offset: Default::default(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            border_thickness: Default::default(),
            border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            roundness_factor: 1.0,
            texture_id: None,
            texture_original_size: Default::default(),
            filling_id: match shape {
                ComponentShape::Rectangle => renderer.create_rectangle()?,
                ComponentShape::Disc => renderer.create_disc(0.0, 512)?,
            },
            border_id: match shape {
                ComponentShape::Rectangle => renderer.create_frame(Default::default())?,
                ComponentShape::Disc => renderer.create_circle(0.0, 512)?,
            },
            children: Default::default(),
            dirty: true,
            event_mask: None,

            on_cursor_enter: None,
            on_cursor_leave: None,
            on_mouse_button_pressed: None,
            on_mouse_button_released: None,
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_shape(&self) -> ComponentShape {
        self.shape
    }

    pub fn get_border_thickness(&self) -> ComponentBorderThickness {
        self.border_thickness
    }

    pub fn set_border_thickness(&mut self, border_thickness: ComponentBorderThickness) -> Result<(), String> {
        if self.shape == ComponentShape::Rectangle && !self.border_thickness.is_axially_uniform() {
            return Err("Not supported".to_string());
        }

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

    pub fn get_roundness_factor(&self) -> f32 {
        self.roundness_factor
    }

    pub fn set_roundness_factor(&mut self, roundness_factor: f32) -> Result<(), String> {
        if self.shape == ComponentShape::Rectangle {
            return Err("Not supported".to_string());
        }

        self.roundness_factor = roundness_factor;
        self.dirty = true;
        Ok(())
    }

    pub fn get_color(&self) -> &Color {
        &self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
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

    fn is_point_inside(&self, point: Vec2) -> bool {
        if let Some(event_mask) = self.event_mask {
            let event_mask_left_bottom = event_mask.position;
            let event_mask_right_top = event_mask.position + event_mask.size;

            if point.x < event_mask_left_bottom.x || point.y < event_mask_left_bottom.y || point.x > event_mask_right_top.x || point.y > event_mask_right_top.y
            {
                return false;
            }
        }

        if self.shape == ComponentShape::Rectangle || (self.shape == ComponentShape::Disc && self.roundness_factor < 0.8) {
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

impl Component for Panel {
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
        if !self.dirty {
            return Ok(());
        }

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

            match self.shape {
                ComponentShape::Rectangle => renderer.get_drawable_with_type_mut::<Frame>(self.border_id)?.set_thickness(self.border_thickness.into()),
                ComponentShape::Disc => renderer
                    .get_drawable_with_type_mut::<Circle>(self.border_id)?
                    .set_thickness(Vec2::new(self.border_thickness.left, self.border_thickness.top)),
            }

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
            let texture_storage_lock = texture_storage.lock().unwrap();
            let texture = texture_storage_lock.get(texture_id)?;

            match self.shape {
                ComponentShape::Rectangle => renderer.get_drawable_with_type_mut::<Rectangle>(self.filling_id)?.set_texture(texture),
                ComponentShape::Disc => renderer.get_drawable_with_type_mut::<Disc>(self.filling_id)?.set_texture(texture),
            }
        }

        renderer.get_drawable_mut(self.filling_id)?.set_size(self.screen_size);

        if self.shape == ComponentShape::Disc {
            renderer.get_drawable_with_type_mut::<Disc>(self.filling_id)?.set_squircle_factor(1.0 - self.roundness_factor);
            renderer.get_drawable_with_type_mut::<Circle>(self.border_id)?.set_squircle_factor(1.0 - self.roundness_factor);
        }

        self.dirty = false;

        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        renderer.draw(self.filling_id)?;

        if self.border_thickness != Default::default() {
            renderer.draw(self.border_id)?;
        }

        Ok(())
    }

    fn is_dirty(&self) -> bool {
        self.dirty
    }

    fn set_dirty_flag(&mut self, dirty: bool) {
        self.dirty = dirty;
    }

    fn get_event_mask(&self) -> Option<EventMask> {
        self.event_mask
    }

    fn set_event_mask(&mut self, event_mask: Option<EventMask>) {
        self.event_mask = event_mask;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
