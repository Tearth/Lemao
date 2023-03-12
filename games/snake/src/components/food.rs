use lemao_framework::ecs::components::Component;

pub struct FoodComponent {
    pub entity_id: usize,
}

impl FoodComponent {
    pub fn new(entity_id: usize) -> Self {
        Self { entity_id }
    }
}

impl Component for FoodComponent {
    fn get_entity(&self) -> usize {
        self.entity_id
    }

    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
}
