use std::collections::VecDeque;
use std::slice::Iter;
use std::slice::IterMut;

#[derive(Default)]
pub struct Entity {}

#[derive(Default)]
pub struct EntityManager {
    data: Vec<Option<Entity>>,
    removed_ids: VecDeque<usize>,
}

impl EntityManager {
    pub fn store(&mut self, item: Entity) -> usize {
        let id = self.get_new_id();
        self.data[id] = Some(item);

        id
    }

    pub fn contains(&self, id: usize) -> bool {
        matches!(self.data.get(id), Some(Some(_)))
    }

    pub fn get(&self, id: usize) -> Result<&Entity, String> {
        match self.data.get(id) {
            Some(Some(item)) => Ok(item),
            _ => Err(format!("Storage item {} not found", id)),
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut Entity, String> {
        match self.data.get_mut(id) {
            Some(Some(item)) => Ok(item),
            _ => Err(format!("Storage item {} not found", id)),
        }
    }

    pub fn iter(&self) -> Iter<Option<Entity>> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Option<Entity>> {
        self.data.iter_mut()
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
