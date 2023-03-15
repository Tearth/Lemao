use crate::ecs::{
    components::{list::ComponentList, Component},
    world::World,
};

use super::Command;

pub struct SpawnCommand<C>
where
    C: Component + 'static,
{
    entity_id: usize,
    data: C,
}

impl<C> SpawnCommand<C>
where
    C: Component + 'static,
{
    pub fn new(entity_id: usize, data: C) -> Self {
        Self { entity_id, data }
    }
}

impl<G, S, M, C> Command<G, S, M> for SpawnCommand<C>
where
    C: Component + 'static,
{
    fn execute(self: Box<Self>, world: &mut World<G, S, M>) -> Result<(), String>
    where
        M: Copy,
    {
        world.components.get_mut::<C>()?.as_any_mut().downcast_mut::<ComponentList<C>>().unwrap().store(self.entity_id, self.data)
    }
}
