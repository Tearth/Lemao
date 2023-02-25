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
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::MouseButton;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::frame::Frame;
use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::Texture;
use lemao_core::utils::storage::StorageItem;
use std::any::Any;

const MAX_BARS_COUNT: usize = 8;

pub struct ProgressBar {
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

    // Label properties
    label_id: usize,
    label_font_id: usize,
    label_text: String,
    label_horizontal_alignment: HorizontalAlignment,
    label_vertical_alignment: VerticalAlignment,
    label_offset: Vec2,
    label_color: Color,

    // Shadow properties
    shadow_id: usize,
    shadow_enabled: bool,
    shadow_offset: Vec2,
    shadow_color: Color,
    shadow_scale: Vec2,
    shadow_corner_rounding: ComponentCornerRounding,

    // Label shadow properties
    label_shadow_enabled: bool,
    label_shadow_offset: Vec2,
    label_shadow_color: Color,

    // Bar properties
    bars: Vec<Bar>,

    // Event handlers
    pub on_cursor_enter: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_cursor_leave: Option<fn(component: &mut Self, cursor_position: Vec2)>,
    pub on_mouse_button_pressed: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
    pub on_mouse_button_released: Option<fn(component: &mut Self, mouse_button: MouseButton, cursor_position: Vec2)>,
}

pub struct Bar {
    // Common properties
    visible: bool,

    // Shape properties
    filling_id: usize,
    color: Color,
    from: f32,
    to: f32,
    corner_rounding: ComponentCornerRounding,
    texture_id: Option<usize>,

    // Border properties
    border_id: usize,
    border_color: Color,
    border_thickness: ComponentBorderThickness,
}

impl ProgressBar {
    pub fn new(id: usize, renderer: &mut RendererContext, label_font_id: usize) -> Result<Self, String> {
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

            // Label properties
            label_id: renderer.create_text(label_font_id)?.get_id(),
            label_font_id,
            label_text: Default::default(),
            label_horizontal_alignment: HorizontalAlignment::Middle,
            label_vertical_alignment: VerticalAlignment::Middle,
            label_offset: Default::default(),
            label_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),

            // Shadow properties
            shadow_id: renderer.create_rectangle()?.get_id(),
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

    /* #region Bar shape properties */
    pub fn get_bar_color(&self, bar_id: usize) -> &Color {
        &self.bars[bar_id].color
    }

    pub fn set_bar_color(&mut self, bar_id: usize, color: Color) {
        self.bars[bar_id].color = color;
        self.dirty = true;
    }

    pub fn get_bar_from(&self, bar_id: usize) -> f32 {
        self.bars[bar_id].from
    }

    pub fn set_bar_from(&mut self, bar_id: usize, from: f32) {
        self.bars[bar_id].from = from;
        self.dirty = true;
    }

    pub fn get_bar_to(&self, bar_id: usize) -> f32 {
        self.bars[bar_id].to
    }

    pub fn set_bar_to(&mut self, bar_id: usize, to: f32) {
        self.bars[bar_id].to = to;
        self.dirty = true;
    }

    pub fn get_bar_corner_rounding(&self, bar_id: usize) -> ComponentCornerRounding {
        self.bars[bar_id].corner_rounding
    }

    pub fn set_bar_corner_rounding(&mut self, bar_id: usize, corner_rounding: ComponentCornerRounding) {
        self.bars[bar_id].corner_rounding = corner_rounding;
        self.dirty = true;
    }

    pub fn get_bar_texture_id(&self, bar_id: usize) -> Option<usize> {
        self.bars[bar_id].texture_id
    }

    pub fn set_bar_texture(&mut self, bar_id: usize, texture: &Texture) {
        self.bars[bar_id].texture_id = Some(texture.get_id());
        self.dirty = true;
    }
    /* #endregion */

    /* #region Bar border properties */
    pub fn get_bar_border_thickness(&self, bar_id: usize) -> ComponentBorderThickness {
        self.bars[bar_id].border_thickness
    }

    pub fn set_bar_border_thickness(&mut self, bar_id: usize, border_thickness: ComponentBorderThickness) -> Result<(), String> {
        self.bars[bar_id].border_thickness = border_thickness;
        self.dirty = true;
        Ok(())
    }

    pub fn get_bar_border_color(&self, bar_id: usize) -> &Color {
        &self.bars[bar_id].border_color
    }

    pub fn set_bar_border_color(&mut self, bar_id: usize, border_color: Color) {
        self.bars[bar_id].border_color = border_color;
        self.dirty = true;
    }
    /* #endregion */

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

    /* #region Label properties */
    pub fn get_label_font_id(&self) -> usize {
        self.label_font_id
    }

    pub fn set_label_font_id(&mut self, label_font_id: usize) {
        self.label_font_id = label_font_id;
        self.dirty = true;
    }

    pub fn get_label_text(&self) -> &str {
        &self.label_text
    }

    pub fn set_label_text(&mut self, text: String) {
        self.label_text = text;
        self.dirty = true;
    }

    pub fn get_label_horizontal_alignment(&self) -> HorizontalAlignment {
        self.label_horizontal_alignment
    }

    pub fn set_label_horizontal_alignment(&mut self, label_horizontal_alignment: HorizontalAlignment) {
        self.label_horizontal_alignment = label_horizontal_alignment;
        self.dirty = true;
    }

    pub fn get_label_vertical_alignment(&self) -> VerticalAlignment {
        self.label_vertical_alignment
    }

    pub fn set_label_vertical_alignment(&mut self, label_vertical_alignment: VerticalAlignment) {
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

    pub fn get_shadow_corner_rounding(&self) -> ComponentCornerRounding {
        self.shadow_corner_rounding
    }

    pub fn set_shadow_corner_rounding(&mut self, shadow_corner_rounding: ComponentCornerRounding) -> Result<(), String> {
        self.shadow_corner_rounding = shadow_corner_rounding;
        self.dirty = true;
        Ok(())
    }
    /* #endregion */

    /* #region Label shadow properties */
    pub fn is_label_shadow_enabled(&self) -> bool {
        self.label_shadow_enabled
    }

    pub fn set_label_shadow_enabled_flag(&mut self, label_shadow_enabled: bool) {
        self.label_shadow_enabled = label_shadow_enabled;
    }

    pub fn get_label_shadow_offset(&self) -> Vec2 {
        self.label_shadow_offset
    }

    pub fn set_label_shadow_offset(&mut self, label_shadow_offset: Vec2) {
        self.label_shadow_offset = label_shadow_offset;
    }

    pub fn get_label_shadow_color(&self) -> &Color {
        &self.label_shadow_color
    }

    pub fn set_label_shadow_color(&mut self, get_label_shadow_color: Color) {
        self.label_shadow_color = get_label_shadow_color;
    }
    /* #endregion */

    /* #region Bar properties */
    pub fn get_bar_visibility(&self, bar_id: usize) -> bool {
        self.bars[bar_id].visible
    }

    pub fn set_bar_visibility(&mut self, bar_id: usize, visible: bool) {
        self.bars[bar_id].visible = visible;
        self.dirty = true;
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

        renderer.get_drawable_and_cast_mut::<Rectangle>(self.filling_id)?.set_corner_rounding(self.corner_rounding.into());
        renderer.get_drawable_and_cast_mut::<Frame>(self.border_id)?.set_corner_rounding(self.corner_rounding.into());

        let font_storage = renderer.get_fonts();
        let font_storage = font_storage.read().unwrap();
        let font = font_storage.get_and_cast::<Font>(self.label_font_id)?;
        let label = renderer.get_drawable_and_cast_mut::<Text>(self.label_id)?;
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

        label.set_position((horizontal_position + vertical_position + self.label_offset).floor());
        label.set_anchor(horizontal_anchor + vertical_anchor);

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

        for bar in &mut self.bars {
            if bar.visible {
                let filling_rectangle = renderer.get_drawable_mut(bar.filling_id)?;
                filling_rectangle.set_position(Vec2::new(self.screen_position.x + self.screen_size.x * bar.from, self.screen_position.y));
                filling_rectangle.set_size(Vec2::new((bar.to - bar.from) * self.screen_size.x, self.screen_size.y));
                filling_rectangle.set_color(bar.color.clone());

                renderer.get_drawable_and_cast_mut::<Rectangle>(bar.filling_id)?.set_corner_rounding(bar.corner_rounding.into());
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

        for bar in self.bars.iter().rev() {
            if bar.visible {
                renderer.draw(bar.filling_id)?;

                if bar.border_thickness != Default::default() {
                    renderer.draw(bar.border_id)?;
                }
            }
        }

        if self.label_shadow_enabled {
            let drawable = renderer.get_drawable_mut(self.label_id)?;
            let original_position = drawable.get_position();
            let original_color = drawable.get_color().clone();

            let drawable = renderer.get_drawable_mut(self.label_id)?;
            drawable.set_position(original_position + self.label_shadow_offset);
            drawable.set_color(self.label_shadow_color.clone());
            renderer.draw(self.label_id)?;

            let drawable = renderer.get_drawable_mut(self.label_id)?;
            drawable.set_position(original_position);
            drawable.set_color(original_color);
        }

        renderer.draw(self.label_id)?;

        if self.border_thickness != Default::default() {
            renderer.draw(self.border_id)?;
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
        renderer.remove_drawable(self.label_id)?;

        for bar in &mut self.bars {
            renderer.remove_drawable(bar.filling_id)?;
            renderer.remove_drawable(bar.border_id)?;
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
            filling_id: renderer.create_rectangle()?.get_id(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            from: 0.0,
            to: 1.0,
            corner_rounding: Default::default(),
            texture_id: None,

            // Border properties
            border_id: renderer.create_frame()?.get_id(),
            border_color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            border_thickness: Default::default(),
        })
    }
}
