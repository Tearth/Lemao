use lemao_framework::ecs::components::Component;

pub struct BodyComponent {
    pub entity_id: usize,
    pub lifetime: u32,
}

impl BodyComponent {
    pub fn new(entity_id: usize, lifetime: u32) -> Self {
        Self { entity_id, lifetime }
    }
}

impl Component for BodyComponent {
    fn get_entity(&self) -> usize {
        self.entity_id
    }

    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
}
