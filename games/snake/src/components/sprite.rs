use lemao_core::renderer::drawable::rectangle::Rectangle;

pub struct SpriteComponent {
    pub rectangle: Rectangle,
}

impl SpriteComponent {
    pub fn new(rectangle: Rectangle) -> Self {
        Self { rectangle }
    }
}
