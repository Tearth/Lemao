use std::collections::{HashMap, VecDeque};

pub struct Storage<T> {
    data: Vec<Option<T>>,
    name_to_id_hashmap: HashMap<String, usize>,
    id_to_name_hashmap: HashMap<usize, String>,
    removed_ids: VecDeque<usize>,
}

pub trait StorageItem {
    fn get_id(&self) -> usize;
    fn set_id(&mut self, id: usize);

    fn get_name(&self) -> Option<String>;
    fn set_name(&mut self, name: Option<String>);
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

    pub fn store_with_name(&mut self, name: &str, mut item: T) -> Result<usize, String> {
        if self.name_to_id_hashmap.contains_key(name) {
            return Err("Name already exists".to_string());
        }

        let id = self.get_new_id();
        item.set_id(id);
        item.set_name(Some(name.to_string()));
        self.data[id] = Some(item);

        self.name_to_id_hashmap.insert(name.to_string(), id);
        self.id_to_name_hashmap.insert(id, name.to_string());

        Ok(id)
    }

    pub fn get(&self, id: usize) -> Result<&T, String> {
        match self.data.get(id) {
            Some(item) => Ok(item.as_ref().unwrap()),
            None => Err(format!("Storage item {} not found", id)),
        }
    }

    pub fn get_by_name(&self, name: &str) -> Result<&T, String> {
        match self.name_to_id_hashmap.get(name) {
            Some(id) => Ok(self.data[*id].as_ref().unwrap()),
            None => Err(format!("Storage item {} not found", name)),
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut T, String> {
        match self.data.get_mut(id) {
            Some(item) => Ok(item.as_mut().unwrap()),
            None => Err(format!("Storage item {} not found", id)),
        }
    }

    pub fn get_by_name_mut(&mut self, name: &str) -> Result<&mut T, String> {
        match self.name_to_id_hashmap.get_mut(name) {
            Some(id) => Ok(self.data[*id].as_mut().unwrap()),
            None => Err(format!("Storage item {} not found", name)),
        }
    }

    pub fn remove(&mut self, id: usize) -> Result<(), String> {
        if id >= self.data.len() || self.data[id].is_none() {
            return Err(format!("Storage item {} not found", id));
        }

        self.data[id] = None;
        self.removed_ids.push_back(id);

        if let Some(name) = self.id_to_name_hashmap.get(&id) {
            self.name_to_id_hashmap.remove(name);
            self.id_to_name_hashmap.remove(&id);
        }

        Ok(())
    }

    pub fn remove_by_name(&mut self, name: &str) -> Result<(), String> {
        if !self.name_to_id_hashmap.contains_key(name) {
            return Err("Name doesn't exist".to_string());
        }

        let id = self.name_to_id_hashmap[name];

        self.data[id] = None;
        self.removed_ids.push_back(id);

        self.name_to_id_hashmap.remove(name);
        self.id_to_name_hashmap.remove(&id);

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
        Self { data: Default::default(), name_to_id_hashmap: Default::default(), id_to_name_hashmap: Default::default(), removed_ids: Default::default() }
    }
}
