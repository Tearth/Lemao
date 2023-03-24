use super::bus::MessageBus;
use super::commands::CommandBus;
use super::components::manager::ComponentManager;
use super::entities::storage::EntityManager;
use super::systems::list::SystemList;
use crate::app::Application;
use lemao_core::lemao_common_platform::input::InputEvent;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

pub struct World<G, S, M>
where
    M: Copy,
{
    pub entities: EntityManager,
    pub components: ComponentManager,
    pub systems: Arc<RwLock<SystemList<G, S, M>>>,
    pub commands: CommandBus<G, S, M>,
    pub messages: MessageBus<M>,
}

impl<G, S, M> World<G, S, M>
where
    M: Copy + Debug,
{
    pub fn new() -> Self {
        Self {
            entities: Default::default(),
            components: Default::default(),
            systems: Arc::new(RwLock::new(SystemList::<G, S, M>::new())),
            commands: CommandBus::new(),
            messages: MessageBus::<M>::new(),
        }
    }

    pub fn update(&mut self, app: &mut Application<G>, scene: &mut S, input: &[InputEvent]) -> Result<(), String> {
        let systems = self.systems.clone();
        let mut systems = systems.write().unwrap();

        for system in &mut systems.iter_mut() {
            system.update(app, scene, self, input)?;

            while let Some(command) = self.commands.poll_message() {
                command.execute(self)?;
            }
        }

        Ok(())
    }
}
