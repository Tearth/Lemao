use lemao_framework::ecs::components::Component;

pub struct HeadComponent {
    pub entity_id: usize,
    pub direction: HeadDirection,
    pub next_direction: HeadDirection,
}

#[derive(Copy, Clone, PartialEq)]
pub enum HeadDirection {
    Up,
    Down,
    Right,
    Left,
}

impl HeadComponent {
    pub fn new(entity_id: usize, direction: HeadDirection) -> Self {
        Self { entity_id, direction, next_direction: direction }
    }
}

impl Component for HeadComponent {
    fn get_entity(&self) -> usize {
        self.entity_id
    }

    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
}
