use super::Component;
use super::ComponentBorderThickness;
use super::ComponentMargin;
use super::ComponentPosition;
use super::ComponentShape;
use super::ComponentSize;
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
    color: Color,
    border_thickness: ComponentBorderThickness,
    border_color: Color,
    roundness_factor: f32,
    texture_id: Option<usize>,
    texture_original_size: Vec2,
    filling_id: usize,
    border_id: usize,
    children: Vec<usize>,
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
        Ok(())
    }

    pub fn get_border_color(&self) -> &Color {
        &self.border_color
    }

    pub fn set_border_color(&mut self, border_color: Color) {
        self.border_color = border_color;
    }

    pub fn get_roundness_factor(&self) -> f32 {
        self.roundness_factor
    }

    pub fn set_roundness_factor(&mut self, roundness_factor: f32) -> Result<(), String> {
        if self.shape == ComponentShape::Rectangle {
            return Err("Not supported".to_string());
        }

        self.roundness_factor = roundness_factor;
        Ok(())
    }

    pub fn get_texture_id(&self) -> Option<usize> {
        self.texture_id
    }

    pub fn set_texture_id(&mut self, texture: &Texture) {
        self.texture_id = Some(texture.get_id());
        self.texture_original_size = texture.get_size();
        self.size = ComponentSize::Absolute(texture.get_size());
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

        if self.screen_size.x.is_nan() {
            self.screen_size.x = (self.texture_original_size.x / self.texture_original_size.y) * self.screen_size.y;
        } else if self.screen_size.y.is_nan() {
            self.screen_size.y = (self.texture_original_size.y / self.texture_original_size.x) * self.screen_size.x;
        }

        self.screen_size = self.screen_size.clamp(self.min_size, self.max_size);

        self.screen_position = match self.position {
            ComponentPosition::AbsoluteToParent(position) => area_position + position,
            ComponentPosition::RelativeToParent(position) => area_position + (position * area_size),
        } - (self.screen_size * self.anchor);

        self.screen_position += Vec2::new(self.margin.left, self.margin.bottom) + self.offset;
        self.screen_size -= Vec2::new(self.margin.left + self.margin.right, self.margin.bottom + self.margin.top);

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

        Ok(())
    }

    fn draw(&mut self, renderer: &mut RendererContext) -> Result<(), String> {
        renderer.draw(self.filling_id)?;

        if self.border_thickness != Default::default() {
            renderer.draw(self.border_id)?;
        }

        Ok(())
    }

    fn is_point_inside(&self, point: Vec2) -> bool {
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

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
