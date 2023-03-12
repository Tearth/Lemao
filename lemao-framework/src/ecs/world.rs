use super::bus::MessageBus;
use super::components::Component;
use super::components::ComponentManager;
use super::components::ComponentManagerTrait;
use super::entites::Entity;
use super::entites::EntityManager;
use super::systems::System;
use crate::app::Application;
use lemao_core::lemao_common_platform::input::InputEvent;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Default)]
pub struct World<G, S, M>
where
    M: Copy,
{
    pub entities: EntityManager,
    pub components: HashMap<TypeId, Arc<RwLock<Box<dyn ComponentManagerTrait>>>>,
    pub systems: Arc<RwLock<Vec<Box<dyn System<G, S, M>>>>>,
    pub bus: MessageBus<M>,
}

impl<G, S, M> World<G, S, M>
where
    M: Copy,
{
    pub fn new() -> Self {
        Self { entities: Default::default(), components: Default::default(), systems: Default::default(), bus: MessageBus::<M>::new() }
    }

    pub fn create_entity(&mut self) -> usize {
        self.entities.store(Entity::default())
    }

    pub fn remove_entity(&mut self, entity_id: usize) -> Result<(), String> {
        if !self.entities.contains(entity_id) {
            return Err("Entity not found".to_string());
        }

        self.entities.remove(entity_id)?;

        for component_manager in self.components.values_mut() {
            let mut component_manager = component_manager.write().unwrap();

            if component_manager.contains(entity_id) {
                component_manager.remove(entity_id)?;
            }
        }

        Ok(())
    }

    pub fn register_component<T>(&mut self) -> Result<(), String>
    where
        T: Component + 'static,
    {
        if self.components.contains_key(&TypeId::of::<T>()) {
            return Err("Component already registered".to_string());
        }

        self.components.insert(TypeId::of::<T>(), Arc::new(RwLock::new(Box::new(ComponentManager::<T>::new()))));
        Ok(())
    }

    pub fn create_component<T>(&mut self, entity_id: usize, component: T) -> Result<(), String>
    where
        T: Component + 'static,
    {
        match self.components.get_mut(&TypeId::of::<T>()) {
            Some(component_manager) => {
                let mut components = component_manager.write().unwrap();
                let component_manager = components.as_any_mut().downcast_mut::<ComponentManager<T>>().unwrap();
                component_manager.store(entity_id, component)?;

                Ok(())
            }
            None => Err("Invalid component".to_string()),
        }
    }

    pub fn create_system<T>(&mut self, system: Box<dyn System<G, S, M>>) -> Result<(), String>
    where
        T: 'static,
    {
        self.systems.write().unwrap().push(system);
        self.bus.register_receiver::<T>()?;

        Ok(())
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
