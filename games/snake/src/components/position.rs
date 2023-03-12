use lemao_framework::ecs::components::Component;

pub struct PositionComponent {
    pub entity_id: usize,
    pub row: u8,
    pub col: u8,
    pub changed: bool,
}

impl PositionComponent {
    pub fn new(entity_id: usize, row: u8, col: u8) -> Self {
        Self { entity_id, row, col, changed: true }
    }
}

impl Component for PositionComponent {
    fn get_entity(&self) -> usize {
        self.entity_id
    }

    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
}
