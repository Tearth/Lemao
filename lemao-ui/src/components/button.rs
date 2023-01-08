use super::Component;
use super::ComponentBorderThickness;
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
use lemao_core::renderer::drawable::circle::Circle;
use lemao_core::renderer::drawable::disc::Disc;
use lemao_core::renderer::drawable::frame::Frame;
use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::textures::Texture;
use std::any::Any;

pub struct Button {
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

    // Shape properties
    filling_id: usize,
    shape: ComponentShape,
    color: Color,
    roundness_factor: f32,
    texture_id: Option<usize>,
    texture_original_size: Vec2,

    // Border properties
    border_id: usize,
    border_thickness: ComponentBorderThickness,
    border_color: Color,

    // Label properties
    label_id: usize,
    label_font_id: usize,
    label_text: String,
    label_horizontal_alignment: HorizontalAlignment,
    label_vertical_alignment: VerticalAlignment,
    label_offset: Vec2,
    label_color: Color,

    // Shadow
    shadow_enabled: bool,
    shadow_offset: Vec2,
    shadow_color: Color,
    shadow_scale: Vec2,
    shadow_roundness_factor: f32,

    // Component-specific properties
    pressed: bool,

    // Event handlers
    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_button_clicked: Option<fn(component: &mut Self, mouse_button: MouseButton)>,
}

impl Button {
    pub fn new(id: usize, renderer: &mut RendererContext, shape: ComponentShape, label_font_id: usize) -> Result<Self, String> {
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

            // Shape properties
            filling_id: match shape {
                ComponentShape::Rectangle => renderer.create_rectangle()?,
                ComponentShape::Disc => renderer.create_disc(0.0, 512)?,
            },
            shape,
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            roundness_factor: 1.0,
            texture_id: None,
            texture_original_size: Default::default(),

            // Border properties
            border_id: match shape {
                ComponentShape::Rectangle => renderer.create_frame(Default::default())?,
                ComponentShape::Disc => renderer.create_circle(0.0, 512)?,
            },
            border_thickness: Default::default(),
            border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Label properties
            label_id: renderer.create_text(label_font_id)?,
            label_font_id,
            label_text: Default::default(),
            label_horizontal_alignment: HorizontalAlignment::Middle,
            label_vertical_alignment: VerticalAlignment::Middle,
            label_offset: Default::default(),
            label_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Shadow
            shadow_enabled: false,
            shadow_offset: Default::default(),
            shadow_color: Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)),
            shadow_scale: Vec2::new(1.0, 1.0),
            shadow_roundness_factor: 0.0,

            // Component-specific properties
            pressed: false,

            // Event handlers
            on_cursor_enter: None,
            on_cursor_leave: None,
            on_mouse_button_pressed: None,
            on_mouse_button_released: None,
            on_button_clicked: None,
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    /* #region Shape properties */
    pub fn get_shape(&self) -> ComponentShape {
        self.shape
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

    pub fn set_texture_id(&mut self, texture: &Texture) {
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

    pub fn get_horizontal_alignment(&self) -> HorizontalAlignment {
        self.label_horizontal_alignment
    }

    pub fn set_horizontal_alignment(&mut self, label_horizontal_alignment: HorizontalAlignment) {
        self.label_horizontal_alignment = label_horizontal_alignment;
        self.dirty = true;
    }

    pub fn get_vertical_alignment(&self) -> VerticalAlignment {
        self.label_vertical_alignment
    }

    pub fn set_vertical_alignment(&mut self, label_vertical_alignment: VerticalAlignment) {
        self.label_vertical_alignment = label_vertical_alignment;
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

    pub fn set_label_color(&mut self, label_color: Color) {
        self.label_color = label_color;
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

    pub fn get_shadow_roundness_factor(&self) -> f32 {
        self.shadow_roundness_factor
    }

    pub fn set_shadow_roundness_factor(&mut self, shadow_roundness_factor: f32) {
        self.shadow_roundness_factor = shadow_roundness_factor;
    }
    /* #endregion */

    /* #region Component-specific properties */
    pub fn is_pressed(&self) -> bool {
        self.pressed
    }

    pub fn set_pressed_flag(&mut self, pressed: bool) {
        self.pressed = pressed;
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

impl Component for Button {
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
                    self.pressed = true;
                }
            }
            InputEvent::MouseButtonReleased(button, cursor_position) => {
                if self.is_point_inside(*cursor_position) {
                    if let Some(f) = self.on_mouse_button_released {
                        (f)(self, *button, *cursor_position);
                        self.dirty = true;
                    };
                    events.push(UiEvent::MouseButtonReleased(self.id, *button));

                    if self.pressed {
                        if let Some(f) = self.on_button_clicked {
                            (f)(self, *button);
                            self.dirty = true;
                        };
                        events.push(UiEvent::ButtonClicked(self.id, *button));
                    }
                }

                self.pressed = false;
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

        let font_storage = renderer.get_fonts();
        let font_storage_lock = font_storage.lock().unwrap();
        let font = font_storage_lock.get(self.label_font_id)?;
        let label = renderer.get_drawable_with_type_mut::<Text>(self.label_id)?;
        label.set_font(font);
        label.set_text(&self.label_text);
        label.set_color(self.label_color.clone());

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

        label.set_position(horizontal_position + vertical_position + self.label_offset);
        label.set_anchor(horizontal_anchor + vertical_anchor);

        self.dirty = false;

        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        if self.shadow_enabled {
            let panel = renderer.get_drawable_mut(self.filling_id)?;
            let original_position = panel.get_position();
            let original_scale = panel.get_scale();
            let original_anchor = panel.get_anchor();
            let original_color = panel.get_color().clone();

            let original_squircle_factor = if let Ok(disc) = renderer.get_drawable_with_type_mut::<Disc>(self.filling_id) {
                let original_squircle_factor = disc.get_squircle_factor();
                disc.set_squircle_factor(1.0 - self.shadow_roundness_factor);

                original_squircle_factor
            } else {
                0.0
            };

            let panel = renderer.get_drawable_mut(self.filling_id)?;
            panel.set_position(original_position + (panel.get_size() / 2.0) + self.shadow_offset);
            panel.set_scale(original_scale * self.shadow_scale);
            panel.set_anchor(Vec2::new(0.5, 0.5));
            panel.set_color(self.shadow_color.clone());
            renderer.draw(self.filling_id)?;

            let panel = renderer.get_drawable_mut(self.filling_id)?;
            panel.set_position(original_position);
            panel.set_scale(original_scale);
            panel.set_anchor(original_anchor);
            panel.set_color(original_color);

            if let Ok(disc) = renderer.get_drawable_with_type_mut::<Disc>(self.filling_id) {
                disc.set_squircle_factor(original_squircle_factor);
            }
        }

        renderer.draw(self.filling_id)?;
        renderer.draw(self.label_id)?;

        if self.border_thickness != Default::default() {
            renderer.draw(self.border_id)?;
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
