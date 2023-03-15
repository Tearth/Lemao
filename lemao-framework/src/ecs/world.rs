use super::bus::MessageBus;
use super::commands::CommandBus;
use super::components::list::ComponentList;
use super::components::list::ComponentListTrait;
use super::components::manager::ComponentManager;
use super::components::Component;
use super::entities::storage::EntityManager;
use super::entities::Entity;
use super::systems::list::SystemList;
use super::systems::System;
use crate::app::Application;
use lemao_core::lemao_common_platform::input::InputEvent;
use std::any::TypeId;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

pub struct World<G, S, M>
where
    M: Copy,
{
    pub entities: EntityManager,
    pub components: ComponentManager,
    pub systems: Arc<RwLock<SystemList<G, S, M>>>,
    pub commands: Arc<RwLock<CommandBus<G, S, M>>>,
    pub messages: MessageBus<M>,
}

impl<G, S, M> World<G, S, M>
where
    M: Copy,
{
    pub fn new() -> Self {
        Self {
            entities: Default::default(),
            components: Default::default(),
            systems: Arc::new(RwLock::new(SystemList::<G, S, M>::new())),
            commands: Arc::new(RwLock::new(CommandBus::new())),
            messages: MessageBus::<M>::new(),
        }
    }

    pub fn update(&mut self, app: &mut Application<G>, scene: &mut S, input: &[InputEvent]) -> Result<(), String> {
        let systems = self.systems.clone();
        let mut systems = systems.write().unwrap();

        for system in &mut systems.iter_mut() {
            system.update(app, scene, self, input)?;

            let commands = self.commands.clone();
            let mut commands = commands.write().unwrap();

            while let Some(command) = commands.poll_message() {
                command.execute(self)?;
            }
        }

        Ok(())
    }
}