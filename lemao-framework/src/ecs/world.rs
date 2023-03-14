use super::bus::MessageBus;
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

#[derive(Default)]
pub struct World<G, S, M>
where
    M: Copy,
{
    pub entities: EntityManager,
    pub components: ComponentManager,
    pub systems: Arc<RwLock<SystemList<G, S, M>>>,
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
            messages: MessageBus::<M>::new(),
        }
    }

    pub fn remove_entity(&mut self, entity_id: usize) -> Result<(), String> {
        if !self.entities.contains(entity_id) {
            return Err("Entity not found".to_string());
        }

        self.entities.remove(entity_id)?;

        for component_manager in self.components.iter_mut() {
            let mut component_manager = component_manager;

            if component_manager.contains(entity_id) {
                component_manager.remove(entity_id)?;
            }
        }

        Ok(())
    }

    pub fn create_component<T>(&mut self, entity_id: usize, component: T) -> Result<(), String>
    where
        T: Component + 'static,
    {
        match self.components.get_mut(&TypeId::of::<T>()) {
            Some(component_manager) => {
                let component_manager = component_manager.as_any_mut().downcast_mut::<ComponentManager<T>>().unwrap();
                component_manager.store(entity_id, component)?;

                Ok(())
            }
            None => Err("Invalid component".to_string()),
        }
    }
    pub fn update(&mut self, app: &mut Application<G>, scene: &mut S, input: &[InputEvent]) -> Result<(), String> {
        let systems = self.systems.clone();
        let mut systems = systems.write().unwrap();

        for system in &mut systems.iter_mut() {
            system.update(app, scene, self, input)?;
        }

        Ok(())
    }
}
