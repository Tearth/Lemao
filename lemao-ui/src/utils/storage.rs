use crate::components::Component;
use std::collections::VecDeque;

#[derive(Default)]
pub struct UiStorage {
    data: Vec<Option<Box<dyn Component>>>,
    removed_ids: VecDeque<usize>,
}

impl UiStorage {
    pub fn store<T>(&mut self, mut item: T) -> usize
    where
        T: Component + 'static,
    {
        let id = self.get_new_id();
        item.set_id(id);
        self.data[id] = Some(Box::new(item));

        id
    }

    pub fn get(&self, id: usize) -> Result<&dyn Component, String> {
        match self.data.get(id) {
            Some(Some(item)) => Ok(item.as_ref()),
            _ => Err(format!("Storage item {} not found", id)),
        }
    }

    pub fn get_and_cast<C>(&self, id: usize) -> Result<&C, String>
    where
        C: 'static,
    {
        self.get(id)?.as_any().downcast_ref::<C>().ok_or_else(|| format!("Storage item {} cannot be downcasted", id))
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut dyn Component, String> {
        match self.data.get_mut(id) {
            Some(Some(item)) => Ok(item.as_mut()),
            _ => Err(format!("Storage item {} not found", id)),
        }
    }

    pub fn get_and_cast_mut<C>(&mut self, id: usize) -> Result<&mut C, String>
    where
        C: 'static,
    {
        self.get_mut(id)?.as_any_mut().downcast_mut::<C>().ok_or_else(|| format!("Storage item {} cannot be downcasted", id))
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn Component>> {
        self.data.iter().filter_map(|p| p.as_ref())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Box<dyn Component>> {
        self.data.iter_mut().filter_map(|p| p.as_mut())
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() || self.data[id].is_none() {
            return Err(format!("Storage item {} not found", id));
        }

        self.data[id] = None;
        self.removed_ids.push_back(id);

        Ok(())
    }

    fn get_new_id(&mut self) -> usize {
        if let Some(id) = self.removed_ids.pop_front() {
            id
        } else {
            self.data.push(None);
            self.data.len() - 1
        }
    }
}
