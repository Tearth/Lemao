use lemao_core::renderer::drawable::rectangle::Rectangle;

pub struct Sprite {
    pub rectangle: Rectangle,
}

impl Sprite {
    pub fn new(rectangle: Rectangle) -> Self {
        Self { rectangle }
    }
}
