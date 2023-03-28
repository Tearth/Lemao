use crate::scenes::game::utils::Direction;
use lemao_framework::ecs::components::Component;

#[derive(Copy, Clone, Debug)]
pub struct HeadComponent {
    pub entity_id: usize,
    pub direction: Direction,
    pub next_direction: Direction,
}

impl HeadComponent {
    pub fn new(entity_id: usize, direction: Direction) -> Self {
        Self { entity_id, direction, next_direction: direction }
    }
}

impl Component for HeadComponent {
    fn get_entity_id(&self) -> usize {
        self.entity_id
    }

    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
}
