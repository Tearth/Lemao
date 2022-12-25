use super::Component;
use super::ComponentBorderThickness;
use super::ComponentMargin;
use super::ComponentPosition;
use super::ComponentSize;
use super::HorizontalAlignment;
use super::VerticalAlignment;
use lemao_core::lemao_math::color::Color;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::frame::Frame;
use lemao_core::renderer::drawable::sprite::Sprite;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Drawable;
use std::any::Any;

pub struct ImageButton {
    pub(crate) id: usize,

    position: ComponentPosition,
    screen_position: Vec2,
    size: ComponentSize,
    screen_size: Vec2,
    anchor: Vec2,
    margin: ComponentMargin,
    offset: Vec2,
    color: Color,
    border_thickness: ComponentBorderThickness,
    border_color: Color,
    label_font_id: usize,
    label_text: String,
    label_horizontal_alignment: HorizontalAlignment,
    label_vertical_alignment: VerticalAlignment,
    label_offset: Vec2,
    texture_id: usize,
    sprite_id: usize,
    border_frame_id: usize,
    label_id: usize,
    children: Vec<usize>,
}

impl ImageButton {
    pub fn new(id: usize, renderer: &mut RendererContext, label_font_id: usize, texture_id: usize) -> Result<Self, String> {
        Ok(Self {
            id,
            position: ComponentPosition::AbsoluteToParent(Default::default()),
            screen_position: Default::default(),
            size: ComponentSize::Absolute(Default::default()),
            screen_size: Default::default(),
            anchor: Default::default(),
            margin: Default::default(),
            offset: Default::default(),
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            border_thickness: Default::default(),
            border_color: Color::new(1.0, 1.0, 1.0, 1.0),
            label_font_id,
            label_text: Default::default(),
            label_horizontal_alignment: HorizontalAlignment::Middle,
            label_vertical_alignment: VerticalAlignment::Middle,
            label_offset: Default::default(),
            texture_id,
            sprite_id: renderer.create_sprite(texture_id)?,
            border_frame_id: renderer.create_frame(Vec2::new(100.0, 100.0))?,
            label_id: renderer.create_text(label_font_id)?,
            children: Default::default(),
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn get_border_thickness(&self) -> ComponentBorderThickness {
        self.border_thickness
    }

    pub fn set_border_thickness(&mut self, border_thickness: ComponentBorderThickness) {
        self.border_thickness = border_thickness;
    }

    pub fn get_border_color(&self) -> Color {
        self.border_color
    }

    pub fn set_border_color(&mut self, border_color: Color) {
        self.border_color = border_color;
    }

    pub fn get_font_id(&self) -> usize {
        self.label_font_id
    }

    pub fn set_font_id(&mut self, font_id: usize) {
        self.label_font_id = font_id;
    }

    pub fn get_text(&self) -> &str {
        &self.label_text
    }

    pub fn set_text(&mut self, text: String) {
        self.label_text = text;
    }

    pub fn get_horizontal_alignment(&self) -> HorizontalAlignment {
        self.label_horizontal_alignment
    }

    pub fn set_horizontal_alignment(&mut self, label_horizontal_alignment: HorizontalAlignment) {
        self.label_horizontal_alignment = label_horizontal_alignment;
    }

    pub fn get_vertical_alignment(&self) -> VerticalAlignment {
        self.label_vertical_alignment
    }

    pub fn set_vertical_alignment(&mut self, label_vertical_alignment: VerticalAlignment) {
        self.label_vertical_alignment = label_vertical_alignment;
    }

    pub fn get_label_offset(&self) -> Vec2 {
        self.label_offset
    }

    pub fn set_label_offset(&mut self, label_offset: Vec2) {
        self.label_offset = label_offset;
    }

    pub fn get_texture_id(&self) -> usize {
        self.texture_id
    }

    pub fn set_texture_id(&mut self, texture_id: usize) {
        self.texture_id = texture_id;
    }
}

impl Component for ImageButton {
    fn get_position(&self) -> ComponentPosition {
        self.position
    }

    fn get_work_area_position(&self) -> Vec2 {
        self.screen_position
    }

    fn set_position(&mut self, position: ComponentPosition) {
        self.position = position;
    }

    fn get_size(&self) -> ComponentSize {
        self.size
    }

    fn get_work_area_size(&self) -> Vec2 {
        self.screen_size
    }

    fn set_size(&mut self, size: ComponentSize) {
        self.size = size;
    }

    fn get_anchor(&self) -> Vec2 {
        self.anchor
    }

    fn set_anchor(&mut self, anchor: Vec2) {
        self.anchor = anchor;
    }

    fn get_margin(&self) -> ComponentMargin {
        self.margin
    }

    fn set_margin(&mut self, margin: ComponentMargin) {
        self.margin = margin;
    }

    fn get_offset(&self) -> Vec2 {
        self.offset
    }

    fn set_offset(&mut self, offset: Vec2) {
        self.offset = offset;
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

    fn update(&mut self, renderer: &mut RendererContext, area_position: Vec2, area_size: Vec2) -> Result<(), String> {
        self.screen_size = match self.size {
            ComponentSize::Absolute(size) => size,
            ComponentSize::Relative(size) => area_size * size,
        };

        self.screen_position = match self.position {
            ComponentPosition::AbsoluteToParent(position) => area_position + position,
            ComponentPosition::RelativeToParent(position) => area_position + (position * area_size),
        } - (self.screen_size * self.anchor);

        self.screen_position += Vec2::new(self.margin.left, self.margin.bottom) + self.offset;
        self.screen_size -= Vec2::new(self.margin.left + self.margin.right, self.margin.bottom + self.margin.top);

        self.screen_size = self.screen_size.floor();
        self.screen_position = self.screen_position.floor();

        if self.border_thickness != Default::default() {
            let border_rectangle = renderer.get_drawable_with_type_mut::<Frame>(self.border_frame_id)?;
            border_rectangle.set_position(self.screen_position);
            border_rectangle.set_size(self.screen_size);
            border_rectangle.set_thickness(self.border_thickness.into());
            border_rectangle.set_color(self.border_color);

            self.screen_position += Vec2::new(self.border_thickness.left, self.border_thickness.bottom);
            self.screen_size -= Vec2::new(self.border_thickness.left + self.border_thickness.right, self.border_thickness.top + self.border_thickness.bottom);

            self.screen_size = self.screen_size.floor();
            self.screen_position = self.screen_position.floor();
        }

        let sprite = renderer.get_drawable_with_type_mut::<Sprite>(self.sprite_id)?;
        sprite.set_position(self.screen_position);
        sprite.set_size(self.screen_size);
        sprite.set_color(self.color);

        let font_storage = renderer.get_fonts();
        let font_storage_lock = font_storage.lock().unwrap();
        let font = font_storage_lock.get(self.label_font_id)?;
        let label = renderer.get_drawable_with_type_mut::<Text>(self.label_id)?;
        label.set_font(font);
        label.set_text(&self.label_text);

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

        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        renderer.draw(self.sprite_id)?;
        renderer.draw(self.label_id)?;

        if self.border_thickness != Default::default() {
            renderer.draw(self.border_frame_id)?;
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
