use crate::renderer::drawable::Drawable;
use crate::utils::hasher::StorageHasherBuilder;
use crate::utils::rand;
use std::any::Any;
use std::collections::HashMap;

pub trait StorageItem {
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn as_drawable(&self) -> Option<&dyn Drawable> {
        None
    }

    fn as_drawable_mut(&mut self) -> Option<&mut dyn Drawable> {
        None
    }
}

pub struct Storage {
    data: HashMap<usize, Box<dyn StorageItem>, StorageHasherBuilder>,
}

impl Storage {
    pub fn store(&mut self, mut item: Box<dyn StorageItem>) -> usize {
        let id = self.get_new_id();
        item.set_id(id);
        self.data.insert(id, item);

        id
    }

    pub fn get(&self, id: usize) -> Result<&dyn StorageItem, String> {
        match self.data.get(&id) {
            Some(item) => Ok(item.as_ref()),
            None => Err(format!("Storage item {} not found", id)),
        }
    }

    pub fn get_and_cast<C>(&self, id: usize) -> Result<&C, String>
    where
        C: 'static,
    {
        self.get(id)?.as_any().downcast_ref::<C>().ok_or_else(|| format!("Storage item {} cannot be downcasted", id))
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut dyn StorageItem, String> {
        match self.data.get_mut(&id) {
            Some(drawable) => Ok(drawable.as_mut()),
            None => Err(format!("Storage item {} not found", id)),
        }
    }

    pub fn get_and_cast_mut<C>(&mut self, id: usize) -> Result<&mut C, String>
    where
        C: 'static,
    {
        self.get_mut(id)?.as_any_mut().downcast_mut::<C>().ok_or_else(|| format!("Storage item {} cannot be downcasted", id))
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if self.data.remove(&id).is_none() {
            return Err(format!("Storage item {} not found", id));
        }

        Ok(())
    }

    fn get_new_id(&self) -> usize {
        loop {
            let id = rand::usize(..);
            if !self.data.contains_key(&id) {
                return id;
            }
        }
    }
}

impl Default for Storage {
    fn default() -> Self {
        Self { data: HashMap::with_hasher(StorageHasherBuilder) }
    }
}
