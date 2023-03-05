use std::collections::VecDeque;

pub struct Storage<T> {
    data: Vec<Option<T>>,
    removed_ids: VecDeque<usize>,
}

pub trait StorageItem {
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);
}

impl<T> Storage<T>
where
    T: StorageItem,
{
    pub fn store(&mut self, mut item: T) -> usize {
        let id = self.get_new_id();
        item.set_id(id);
        self.data[id] = Some(item);

        id
    }

    pub fn get(&self, id: usize) -> Result<&T, String> {
        match self.data.get(id) {
            Some(item) => Ok(item.as_ref().unwrap()),
            None => Err(format!("Storage item {} not found", id)),
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut T, String> {
        match self.data.get_mut(id) {
            Some(item) => Ok(item.as_mut().unwrap()),
            None => Err(format!("Storage item {} not found", id)),
        }
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

impl<T> Default for Storage<T> {
    fn default() -> Self {
        Self { data: Default::default(), removed_ids: Default::default() }
    }
}
