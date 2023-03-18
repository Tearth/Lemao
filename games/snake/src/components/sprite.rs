use std::{alloc::System, time::SystemTime};

use lemao_core::renderer::drawable::rectangle::Rectangle;
use lemao_framework::ecs::components::Component;

pub struct SpriteComponent {
    pub entity_id: usize,
    pub rectangle: Rectangle,
    pub layer: u8,

    pub blinking: bool,
    pub blinking_interval: u32,
    pub blinking_last_change_time: SystemTime,
}

impl SpriteComponent {
    pub fn new(entity_id: usize, rectangle: Rectangle, layer: u8) -> Self {
        Self { entity_id, rectangle, layer, blinking: false, blinking_interval: 0, blinking_last_change_time: SystemTime::now() }
    }
}

impl Component for SpriteComponent {
    fn get_entity(&self) -> usize {
        self.entity_id
    }

    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
}
