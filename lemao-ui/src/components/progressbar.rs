use super::Component;
use super::ComponentBorderThickness;
use super::ComponentCornerRounding;
use super::ComponentMargin;
use super::ComponentPosition;
use super::ComponentSize;
use super::EventMask;
use super::HorizontalAlignment;
use super::VerticalAlignment;
use crate::events::UiEvent;
use crate::utils::storage::UiStorageItem;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::drawable::DrawableEnum;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::Texture;
use std::any::Any;

const MAX_BARS_COUNT: usize = 8;

pub struct ProgressBar {
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
    pub color: Color,
    pub corner_rounding: ComponentCornerRounding,
    pub texture_id: Option<usize>,
    pub texture_original_size: Vec2,

    // Border properties
    pub border_id: usize,
    pub border_color: Color,
    pub border_thickness: ComponentBorderThickness,

    // Label properties
    pub label_id: usize,
    pub label_font_id: usize,
    pub label_text: String,
    pub label_horizontal_alignment: HorizontalAlignment,
    pub label_vertical_alignment: VerticalAlignment,
    pub label_offset: Vec2,
    pub label_color: Color,

    // Shadow properties
    pub shadow_id: usize,
    pub shadow_enabled: bool,
    pub shadow_offset: Vec2,
    pub shadow_color: Color,
    pub shadow_scale: Vec2,
    pub shadow_corner_rounding: ComponentCornerRounding,

    // Label shadow properties
    pub label_shadow_enabled: bool,
    pub label_shadow_offset: Vec2,
    pub label_shadow_color: Color,

    // Bar properties
    pub bars: Vec<Bar>,

    // Event handlers
    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
}

pub struct Bar {
    // Common properties
    pub visible: bool,

    // Shape properties
    pub filling_id: usize,
    pub color: Color,
    pub from: f32,
    pub to: f32,
    pub corner_rounding: ComponentCornerRounding,
    pub texture_id: Option<usize>,

    // Border properties
    pub border_id: usize,
    pub border_color: Color,
    pub border_thickness: ComponentBorderThickness,
}

impl ProgressBar {
    pub fn new(renderer: &mut RendererContext, label_font_id: usize) -> Result<Self, String> {
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
            filling_id: renderer.create_rectangle()?,
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            corner_rounding: Default::default(),
            texture_id: None,
            texture_original_size: Default::default(),

            // Border properties
            border_id: renderer.create_frame()?,
            border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            border_thickness: Default::default(),

            // Label properties
            label_id: renderer.create_text(label_font_id)?,
            label_font_id,
            label_text: Default::default(),
            label_horizontal_alignment: HorizontalAlignment::Middle,
            label_vertical_alignment: VerticalAlignment::Middle,
            label_offset: Default::default(),
            label_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Shadow properties
            shadow_id: renderer.create_rectangle()?,
            shadow_enabled: false,
            shadow_offset: Default::default(),
            shadow_color: Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)),
            shadow_scale: Vec2::new(1.0, 1.0),
            shadow_corner_rounding: Default::default(),

            // Label shadow properties
            label_shadow_enabled: false,
            label_shadow_offset: Default::default(),
            label_shadow_color: Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0)),

            // Bar properties
            bars: std::iter::repeat_with(|| Bar::new(renderer).unwrap()).take(MAX_BARS_COUNT).collect::<Vec<_>>(),

            // Event handlers
            on_cursor_enter: None,
            on_cursor_leave: None,
            on_mouse_button_pressed: None,
            on_mouse_button_released: None,
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn set_bar_texture(&mut self, bar_id: usize, texture: &Texture) {
        self.bars[bar_id].texture_id = Some(texture.id);
        self.dirty = true;
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

        let x1 = self.screen_position.x;
        let y1 = self.screen_position.y;
        let x2 = self.screen_position.x + self.screen_size.x;
        let y2 = self.screen_position.y + self.screen_size.y;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }
}

impl Component for ProgressBar {
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
            let border = renderer.frames.get_mut(self.border_id)?;
            border.position = self.screen_position;
            border.size = self.screen_size;
            border.color = self.border_color.clone();
            border.thickness = self.border_thickness.into();
            border.corner_rounding = self.corner_rounding.into();
            border.update();

            self.screen_position += Vec2::new(self.border_thickness.left, self.border_thickness.bottom);
            self.screen_size -= Vec2::new(self.border_thickness.left + self.border_thickness.right, self.border_thickness.top + self.border_thickness.bottom);

            self.screen_size = self.screen_size.floor();
            self.screen_position = self.screen_position.floor();
        }

        let filling = renderer.rectangles.get_mut(self.filling_id)?;
        filling.position = self.screen_position;
        filling.color = self.color.clone();
        filling.size = self.screen_size;
        filling.corner_rounding = self.corner_rounding.into();

        if let Some(texture_id) = self.texture_id {
            filling.set_texture(renderer.textures.get(texture_id)?)
        }

        filling.update();

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
        let label = renderer.texts.get_mut(self.label_id)?;
        label.set_font(font);
        label.text = self.label_text.clone();
        label.color = self.label_color.clone();
        label.position = (horizontal_position + vertical_position + self.label_offset).floor();
        label.anchor = horizontal_anchor + vertical_anchor;
        label.update();

        if self.shadow_enabled {
            let shadow = renderer.rectangles.get_mut(self.shadow_id)?;
            shadow.position = self.screen_position + self.screen_size / 2.0 + self.shadow_offset;
            shadow.size = self.screen_size;
            shadow.anchor = Vec2::new(0.5, 0.5);
            shadow.color = self.shadow_color.clone();
            shadow.scale = self.shadow_scale;
            shadow.corner_rounding = self.shadow_corner_rounding.into();
            shadow.update();
        }

        for bar in &mut self.bars {
            if bar.visible {
                let filling = renderer.rectangles.get_mut(bar.filling_id)?;
                filling.position = Vec2::new(self.screen_position.x + self.screen_size.x * bar.from, self.screen_position.y);
                filling.size = Vec2::new((bar.to - bar.from) * self.screen_size.x, self.screen_size.y);
                filling.color = bar.color.clone();
                filling.corner_rounding = bar.corner_rounding.into();
                filling.update();
            }
        }

        self.dirty = false;
        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        if self.shadow_enabled {
            renderer.draw(DrawableEnum::Rectangle, self.shadow_id)?;
        }

        renderer.draw(DrawableEnum::Rectangle, self.filling_id)?;

        for bar in self.bars.iter().rev() {
            if bar.visible {
                renderer.draw(DrawableEnum::Rectangle, bar.filling_id)?;

                if bar.border_thickness != Default::default() {
                    renderer.draw(DrawableEnum::Frame, bar.border_id)?;
                }
            }
        }

        if self.label_shadow_enabled {
            let drawable = renderer.texts.get_mut(self.label_id)?;
            let original_position = drawable.position;
            let original_color = drawable.color.clone();

            drawable.position = original_position + self.label_shadow_offset;
            drawable.color = self.label_shadow_color.clone();
            renderer.draw(DrawableEnum::Text, self.label_id)?;

            let drawable = renderer.texts.get_mut(self.label_id)?;
            drawable.position = original_position;
            drawable.color = original_color;
        }

        renderer.draw(DrawableEnum::Text, self.label_id)?;

        if self.border_thickness != Default::default() {
            renderer.draw(DrawableEnum::Frame, self.border_id)?;
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
        renderer.rectangles.remove(self.filling_id);
        renderer.frames.remove(self.border_id);
        renderer.rectangles.remove(self.shadow_id);
        renderer.texts.remove(self.label_id);

        for bar in &mut self.bars {
            renderer.rectangles.remove(bar.filling_id);
            renderer.frames.remove(bar.border_id);
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

impl Bar {
    pub fn new(renderer: &mut RendererContext) -> Result<Self, String> {
        Ok(Self {
            // Common properties
            visible: false,

            // Shape properties
            filling_id: renderer.create_rectangle()?,
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            from: 0.0,
            to: 1.0,
            corner_rounding: Default::default(),
            texture_id: None,

            // Border properties
            border_id: renderer.create_frame()?,
            border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            border_thickness: Default::default(),
        })
    }
}

impl UiStorageItem for ProgressBar {
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
