use crate::scenes::game::utils::Coordinates;
use crate::scenes::game::utils::Direction;
use lemao_framework::ecs::components::Component;

#[derive(Copy, Clone, Debug)]
pub struct PositionComponent {
    pub entity_id: usize,
    pub coordinates: Coordinates,
    pub direction: Option<Direction>,
    pub changed: bool,
}

impl PositionComponent {
    pub fn new(entity_id: usize, coordinates: Coordinates, direction: Option<Direction>) -> Self {
        Self { entity_id, coordinates, direction, changed: true }
    }
}

impl Component for PositionComponent {
    fn get_entity_id(&self) -> usize {
        self.entity_id
    }

    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
}
