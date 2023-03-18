use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_framework::ecs::components::Component;

pub struct SpriteComponent {
    pub entity_id: usize,
    pub rectangle: Rectangle,
    pub layer: u8,
}

impl SpriteComponent {
    pub fn new(entity_id: usize, rectangle: Rectangle, layer: u8) -> Self {
        Self { entity_id, rectangle, layer }
    }
}

impl Component for SpriteComponent {
    fn get_entity(&self) -> usize {
        self.entity_id
    }

    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
}
