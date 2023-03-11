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
pub struct World<G, S> {
    pub entities: EntityManager,
    pub components: HashMap<TypeId, Arc<RwLock<Box<dyn ComponentManagerTrait>>>>,
    pub systems: Arc<RwLock<Vec<System<G, S>>>>,
}

impl<G, S> World<G, S> {
    pub fn new() -> Self {
        Self { entities: Default::default(), components: Default::default(), systems: Default::default() }
    }

    pub fn create_entity(&mut self) -> usize {
        self.entities.store(Entity::default())
    }

    pub fn register_component<T>(&mut self) -> Result<(), String>
    where
        T: 'static,
    {
        if self.components.contains_key(&TypeId::of::<T>()) {
            return Err("Component already registered".to_string());
        }

        self.components.insert(TypeId::of::<T>(), Arc::new(RwLock::new(Box::new(ComponentManager::<T>::new()))));
        Ok(())
    }

    pub fn create_component<T>(&mut self, entity_id: usize, component: T) -> Result<(), String>
    where
        T: 'static,
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

    pub fn create_system(&mut self, system: System<G, S>) {
        self.systems.write().unwrap().push(system);
    }

    pub fn update(&mut self, app: &mut Application<G>, scene: &mut S, input: &[InputEvent]) -> Result<(), String> {
        let systems = self.systems.clone();
        let mut systems = systems.write().unwrap();

        for system in &mut systems.iter_mut() {
            (system)(app, scene, self, input)?;
        }

        Ok(())
    }
}
