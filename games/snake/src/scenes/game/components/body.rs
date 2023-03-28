use crate::scenes::game::utils::Direction;
use lemao_framework::ecs::components::Component;

#[derive(Copy, Clone, Debug)]
pub struct BodyComponent {
    pub entity_id: usize,
    pub lifetime: u32,
    pub orientation: BodyOrientation,
    pub direction: Direction,
    pub killed: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub fn new(entity_id: usize, lifetime: u32, orientation: BodyOrientation, direction: Direction) -> Self {
        Self { entity_id, lifetime, orientation, direction, killed: false }
    }
}

impl Component for BodyComponent {
    fn get_entity_id(&self) -> usize {
        self.entity_id
    }

    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
}
