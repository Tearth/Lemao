use super::Component;
use super::ComponentBorderThickness;
use super::ComponentCornerRounding;
use super::ComponentMargin;
use super::ComponentPosition;
use super::ComponentShape;
use super::ComponentSize;
use super::EventMask;
use crate::events::UiEvent;
use crate::utils::storage::UiStorageItem;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::drawable::DrawableEnum;
use lemao_core::renderer::textures::Texture;
use std::any::Any;

pub struct Panel {
    pub(crate) id: usize,

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
    pub filling_id: usize,
    pub shape: ComponentShape,
    pub color: Color,
    pub corner_rounding: ComponentCornerRounding,
    pub start_angle: f32,
    pub end_angle: f32,
    pub texture_id: Option<usize>,
    pub texture_original_size: Vec2,

    // Border properties
    pub border_id: usize,
    pub border_color: Color,
    pub border_thickness: ComponentBorderThickness,

    // Shadow properties
    pub shadow_id: usize,
    pub shadow_enabled: bool,
    pub shadow_offset: Vec2,
    pub shadow_color: Color,
    pub shadow_scale: Vec2,
    pub shadow_corner_rounding: ComponentCornerRounding,

    // Event handlers
    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
}

impl Panel {
    pub fn new(renderer: &mut RendererContext, shape: ComponentShape) -> Result<Self, String> {
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
            filling_id: match shape {
                ComponentShape::Rectangle => renderer.create_rectangle()?,
                ComponentShape::Disc => renderer.create_disc()?,
            },
            shape,
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            corner_rounding: Default::default(),
            start_angle: 0.0,
            end_angle: std::f32::consts::PI * 2.0,
            texture_id: None,
            texture_original_size: Default::default(),

            // Border properties
            border_id: match shape {
                ComponentShape::Rectangle => renderer.create_frame()?,
                ComponentShape::Disc => renderer.create_circle()?,
            },
            border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            border_thickness: Default::default(),

            // Shadow properties
            shadow_id: match shape {
                ComponentShape::Rectangle => renderer.create_rectangle()?,
                ComponentShape::Disc => renderer.create_disc()?,
            },
            shadow_enabled: false,
            shadow_offset: Default::default(),
            shadow_color: Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)),
            shadow_scale: Vec2::new(1.0, 1.0),
            shadow_corner_rounding: Default::default(),

            // Event handlers
            on_cursor_enter: None,
            on_cursor_leave: None,
            on_mouse_button_pressed: None,
            on_mouse_button_released: None,
        })
    }

    pub fn set_texture(&mut self, texture: &Texture) {
        self.texture_id = Some(texture.id);
        self.texture_original_size = texture.size;
        self.size = ComponentSize::Absolute(texture.size);
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

            let mut angle = Vec2::new_from_angle(self.start_angle).signed_angle(point - component_center);
            if angle < 0.0 {
                angle += std::f32::consts::PI * 2.0;
            }

            let within_angle = angle >= 0.0 && angle <= self.end_angle - self.start_angle;
            within_angle && scaled_point.distance(Vec2::new(0.0, 0.0)) <= self.screen_size.x / 2.0
        }
    }
}

impl Component for Panel {
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
            match self.shape {
                ComponentShape::Rectangle => {
                    let border = renderer.frames.get_mut(self.border_id)?;
                    border.position = self.screen_position;
                    border.size = self.screen_size;
                    border.color = self.border_color.clone();
                    border.thickness = self.border_thickness.into();
                    border.corner_rounding = self.corner_rounding.into();

                    border.update();
                }
                ComponentShape::Disc => {
                    let border = renderer.circles.get_mut(self.border_id)?;
                    border.position = self.screen_position;
                    border.size = self.screen_size;
                    border.color = self.border_color.clone();
                    border.thickness = Vec2::new(self.border_thickness.left, self.border_thickness.top);
                    border.start_angle = self.start_angle;
                    border.end_angle = self.end_angle;

                    border.update();
                }
            }

            self.screen_position += Vec2::new(self.border_thickness.left, self.border_thickness.bottom);
            self.screen_size -= Vec2::new(self.border_thickness.left + self.border_thickness.right, self.border_thickness.top + self.border_thickness.bottom);

            self.screen_size = self.screen_size.floor();
            self.screen_position = self.screen_position.floor();
        }

        match self.shape {
            ComponentShape::Rectangle => {
                let filling = renderer.rectangles.get_mut(self.filling_id)?;
                filling.position = self.screen_position;
                filling.color = self.color.clone();
                filling.size = self.screen_size;
                filling.corner_rounding = self.corner_rounding.into();

                if let Some(texture_id) = self.texture_id {
                    filling.set_texture(renderer.textures.get(texture_id)?);
                }

                filling.update();
            }
            ComponentShape::Disc => {
                let filling = renderer.discs.get_mut(self.filling_id)?;
                filling.position = self.screen_position;
                filling.color = self.color.clone();
                filling.size = self.screen_size;
                filling.start_angle = self.start_angle;
                filling.end_angle = self.end_angle;

                if let Some(texture_id) = self.texture_id {
                    filling.set_texture(renderer.textures.get(texture_id)?);
                }

                filling.update();
            }
        };

        if self.shadow_enabled {
            match self.shape {
                ComponentShape::Rectangle => {
                    let shadow = renderer.rectangles.get_mut(self.shadow_id)?;
                    shadow.position = self.screen_position + self.screen_size / 2.0 + self.shadow_offset;
                    shadow.size = self.screen_size;
                    shadow.anchor = Vec2::new(0.5, 0.5);
                    shadow.color = self.shadow_color.clone();
                    shadow.scale = self.shadow_scale;
                    shadow.corner_rounding = self.shadow_corner_rounding.into();
                    shadow.update();
                }
                ComponentShape::Disc => {
                    let shadow = renderer.discs.get_mut(self.shadow_id)?;
                    shadow.position = self.screen_position + self.screen_size / 2.0 + self.shadow_offset;
                    shadow.size = self.screen_size;
                    shadow.anchor = Vec2::new(0.5, 0.5);
                    shadow.color = self.shadow_color.clone();
                    shadow.scale = self.shadow_scale;
                    shadow.start_angle = self.start_angle;
                    shadow.end_angle = self.end_angle;
                    shadow.update()
                }
            };
        }

        self.dirty = false;
        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        let filling_type = match self.shape {
            ComponentShape::Rectangle => DrawableEnum::Rectangle,
            ComponentShape::Disc => DrawableEnum::Disc,
        };
        let border_type = match self.shape {
            ComponentShape::Rectangle => DrawableEnum::Frame,
            ComponentShape::Disc => DrawableEnum::Circle,
        };

        if self.shadow_enabled {
            renderer.draw(filling_type, self.shadow_id)?;
        }

        renderer.draw(filling_type, self.filling_id)?;

        if self.border_thickness != Default::default() {
            renderer.draw(border_type, self.border_id)?;
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
        match self.shape {
            ComponentShape::Rectangle => {
                renderer.rectangles.remove(self.filling_id)?;
                renderer.frames.remove(self.border_id)?;
                renderer.rectangles.remove(self.shadow_id)?;
            }
            ComponentShape::Disc => {
                renderer.discs.remove(self.filling_id)?;
                renderer.circles.remove(self.border_id)?;
                renderer.discs.remove(self.shadow_id)?;
            }
        };

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl UiStorageItem for Panel {
    fn get_id(&self) -> usize {
        self.id
    }

    fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn as_component(&self) -> Option<&dyn Component> {
        Some(self)
    }

    fn as_component_mut(&mut self) -> Option<&mut dyn Component> {
        Some(self)
    }
}
