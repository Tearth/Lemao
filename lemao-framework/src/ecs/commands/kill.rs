use super::Command;
use crate::ecs::world::World;

#[derive(Copy, Clone, Debug)]
pub struct KillCommand {
    entity_id: usize,
}

impl KillCommand {
    pub fn new(entity_id: usize) -> Self {
        Self { entity_id }
    }
}

impl<G, S, M> Command<G, S, M> for KillCommand {
    fn execute(self: Box<Self>, world: &mut World<G, S, M>) -> Result<(), String>
    where
        M: Copy,
    {
        if !world.entities.contains(self.entity_id) {
            return Err("Entity not found".to_string());
        }

        world.entities.remove(self.entity_id)?;

        for component_manager in world.components.iter_mut() {
            let component_manager = component_manager;

            if component_manager.contains(self.entity_id) {
                component_manager.remove(self.entity_id)?;
            }
        }

        Ok(())
    }
}
