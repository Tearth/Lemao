use crate::app::Scene;
use std::collections::hash_map::Values;
use std::collections::hash_map::ValuesMut;
use std::collections::HashMap;

#[derive(Default)]
pub struct SceneStorage<G> {
    data: HashMap<String, Box<dyn Scene<G>>>,
}

impl<G> SceneStorage<G> {
    pub fn store(&mut self, name: &str, item: Box<dyn Scene<G>>) {
        self.data.insert(name.to_string(), item);
    }

    pub fn get(&self, name: &str) -> Result<&dyn Scene<G>, String> {
        match self.data.get(&name.to_string()) {
            Some(item) => Ok(item.as_ref()),
            None => Err(format!("Storage item {} not found", name)),
        }
    }

    pub fn get_and_cast<C>(&self, name: &str) -> Result<&C, String>
    where
        C: 'static,
    {
        self.get(name)?.as_any().downcast_ref::<C>().ok_or_else(|| format!("Storage item {} cannot be downcasted", name))
    }

    pub fn get_mut(&mut self, name: &str) -> Result<&mut dyn Scene<G>, String> {
        match self.data.get_mut(&name.to_string()) {
            Some(drawable) => Ok(drawable.as_mut()),
            None => Err(format!("Storage item {} not found", name)),
        }
    }

    pub fn get_and_cast_mut<C>(&mut self, name: &str) -> Result<&mut C, String>
    where
        C: 'static,
    {
        self.get_mut(name)?.as_any_mut().downcast_mut::<C>().ok_or_else(|| format!("Storage item {} cannot be downcasted", name))
    }

    pub fn iter(&self) -> Values<String, Box<dyn Scene<G>>> {
        self.data.values()
    }

    pub fn iter_mut(&mut self) -> ValuesMut<String, Box<dyn Scene<G>>> {
        self.data.values_mut()
    }

    pub fn remove(&mut self, name: &str) -> Result<(), String> {
        if self.data.remove(&name.to_string()).is_none() {
            return Err(format!("Storage item {} not found", name));
        }

        Ok(())
    }
}
