use super::Component;
use super::ComponentMargin;
use super::ComponentPosition;
use super::ComponentSize;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::context::RendererContext;
use lemao_core::renderer::drawable::text::Text;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::drawable::Drawable;
use std::any::Any;

pub struct Label {
    pub(crate) id: usize,

    position: ComponentPosition,
    screen_position: Vec2,
    size: ComponentSize,
    screen_size: Vec2,
    min_size: Vec2,
    max_size: Vec2,
    anchor: Vec2,
    margin: ComponentMargin,
    offset: Vec2,
    color: Color,
    multiline: bool,
    max_multiline_width: f32,
    label_font_id: usize,
    label_text: String,
    label_offset: Vec2,
    label_id: usize,
    children: Vec<usize>,
}

impl Label {
    pub fn new(id: usize, renderer: &mut RendererContext, label_font_id: usize) -> Result<Self, String> {
        Ok(Self {
            id,
            position: ComponentPosition::AbsoluteToParent(Default::default()),
            screen_position: Default::default(),
            size: ComponentSize::Absolute(Default::default()),
            screen_size: Default::default(),
            min_size: Vec2::new(f32::MIN, f32::MIN),
            max_size: Vec2::new(f32::MAX, f32::MAX),
            anchor: Default::default(),
            margin: Default::default(),
            offset: Default::default(),
            color: Color::SolidColor(SolidColor::new(1.0, 1.0, 1.0, 1.0)),
            multiline: false,
            max_multiline_width: 0.0,
            label_font_id,
            label_text: Default::default(),
            label_offset: Default::default(),
            label_id: renderer.create_text(label_font_id)?,
            children: Default::default(),
        })
    }

    pub fn get_id(&self) -> usize {
        self.id
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
        self.multiline = false;
    }

    pub fn set_multiline_text(&mut self, text: String, width: f32) {
        self.label_text = text;
        self.max_multiline_width = width;
        self.multiline = true;
    }

    pub fn get_label_offset(&self) -> Vec2 {
        self.label_offset
    }

    pub fn set_label_offset(&mut self, label_offset: Vec2) {
        self.label_offset = label_offset;
    }
}

impl Component for Label {
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

    fn get_min_size(&self) -> Vec2 {
        self.min_size
    }

    fn set_min_size(&mut self, min_size: Vec2) {
        self.min_size = min_size;
    }

    fn get_max_size(&self) -> Vec2 {
        self.max_size
    }

    fn set_max_size(&mut self, max_size: Vec2) {
        self.max_size = max_size;
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

    fn get_color(&self) -> &Color {
        &self.color
    }

    fn set_color(&mut self, color: Color) {
        self.color = color;
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

        self.screen_size = self.screen_size.clamp(self.min_size, self.max_size);

        self.screen_position = match self.position {
            ComponentPosition::AbsoluteToParent(position) => area_position + position,
            ComponentPosition::RelativeToParent(position) => area_position + (position * area_size),
        } - (self.screen_size * self.anchor);

        self.screen_position += Vec2::new(
            self.margin.left * self.anchor.x - self.margin.right * (self.anchor.x - 1.0),
            self.margin.bottom * (self.anchor.y - 1.0) - self.margin.top * self.anchor.y,
        ) + self.offset;
        self.screen_size -= Vec2::new(self.margin.left + self.margin.right, self.margin.bottom + self.margin.top);

        self.screen_size = self.screen_size.floor();
        self.screen_position = self.screen_position.floor();

        let font_storage = renderer.get_fonts();
        let font_storage_lock = font_storage.lock().unwrap();
        let font = font_storage_lock.get(self.label_font_id)?;
        let label = renderer.get_drawable_with_type_mut::<Text>(self.label_id)?;

        if self.multiline {
            let mut line = String::new();
            let mut result = String::new();

            for token in self.label_text.split_whitespace() {
                if label.calculate_text_size(line.clone() + token).x > self.max_multiline_width {
                    result += &(line.trim().to_string() + "\n");
                    line.clear();
                }

                line += token;
                line += " ";
            }

            self.label_text = result + &line;
        }

        label.set_font(font);
        label.set_text(&self.label_text);
        label.set_position(self.screen_position);
        label.set_anchor(self.anchor);
        label.set_color(self.color.clone());

        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        renderer.draw(self.label_id)?;
        Ok(())
    }

    fn is_point_inside(&self, point: Vec2) -> bool {
        let x1 = self.screen_position.x;
        let y1 = self.screen_position.y;
        let x2 = self.screen_position.x + self.screen_size.x;
        let y2 = self.screen_position.y + self.screen_size.y;

        point.x >= x1 && point.y >= y1 && point.x <= x2 && point.y <= y2
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
