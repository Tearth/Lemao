use std::time::SystemTime;

use lemao_core::renderer::drawable::tilemap::Tilemap;
use lemao_framework::ecs::components::Component;

#[derive(Debug)]
pub struct SpriteComponent {
    pub entity_id: usize,
    pub tilemap: Tilemap,
    pub layer: u8,

    pub blinking: bool,
    pub blinking_interval: u32,
    pub blinking_last_change_time: SystemTime,
}

impl SpriteComponent {
    pub fn new(entity_id: usize, tilemap: Tilemap, layer: u8) -> Self {
        Self { entity_id, tilemap, layer, blinking: false, blinking_interval: 0, blinking_last_change_time: SystemTime::now() }
    }
}

impl Component for SpriteComponent {
    fn get_entity_id(&self) -> usize {
        self.entity_id
    }

    fn set_entity_id(&mut self, entity_id: usize) {
        self.entity_id = entity_id;
    }
}
