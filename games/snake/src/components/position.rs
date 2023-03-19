use lemao_framework::ecs::components::Component;

use crate::utils::Coordinates;

pub struct PositionComponent {
    pub entity_id: usize,
    pub coordinates: Coordinates,
    pub changed: bool,
}

impl PositionComponent {
    pub fn new(entity_id: usize, coordinates: Coordinates) -> Self {
        Self { entity_id, coordinates, changed: true }
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
