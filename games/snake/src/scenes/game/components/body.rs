use lemao_framework::ecs::components::Component;

use super::head::HeadDirection;

pub struct BodyComponent {
    pub entity_id: usize,
    pub lifetime: u32,
    pub orientation: BodyOrientation,
    pub direction: HeadDirection,
    pub killed: bool,
}

#[derive(Copy, Clone)]
pub enum BodyOrientation {
    TopBottom,
    RightLeft,
    RightBottom,
    LeftBottom,
    LeftTop,
    RightTop,
    LeftEnd,
    TopEnd,
    RightEnd,
    BottomEnd,
}

impl BodyComponent {
    pub fn new(entity_id: usize, lifetime: u32, orientation: BodyOrientation, direction: HeadDirection) -> Self {
        Self { entity_id, lifetime, orientation, direction, killed: false }
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
