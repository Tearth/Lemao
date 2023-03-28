use lemao_framework::ecs::components::Component;

#[derive(Copy, Clone, Debug)]
pub struct CellComponent {
    pub entity_id: usize,
}

impl CellComponent {
    pub fn new(entity_id: usize) -> Self {
        Self { entity_id }
    }
}

impl Component for CellComponent {
    fn get_entity_id(&self) -> usize {
        self.entity_id
    }

    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
}
