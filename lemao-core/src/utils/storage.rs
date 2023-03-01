use std::collections::VecDeque;

pub struct Storage<T> {
    data: Vec<Option<T>>,
    removed_ids: VecDeque<usize>,
}

impl<T> Storage<T> {
    pub fn store(&mut self, item: T) -> usize {
        let id = self.get_new_id();
        self.data.insert(id, Some(item));

        id
    }

    pub fn get(&self, id: usize) -> Result<&T, String> {
        match self.data.get(id) {
            Some(item) => Ok(item.as_ref().unwrap()),
            None => {
                let x = 10;
                Err(format!("Storage item {} not found", id))
            }
        }
    }

    pub fn get_mut(&mut self, id: usize) -> Result<&mut T, String> {
        match self.data.get_mut(id) {
            Some(item) => Ok(item.as_mut().unwrap()),
            None => {
                let x = 10;
                Err(format!("Storage item {} not found", id))
            }
        }
    }

    pub fn remove(&mut self, id: usize) {
        self.data.remove(id);
        self.removed_ids.push_back(id);
    }

    fn get_new_id(&mut self) -> usize {
        if let Some(id) = self.removed_ids.pop_front() {
            id
        } else {
            self.data.len()
        }
    }
}

impl<T> Default for Storage<T> {
    fn default() -> Self {
        Self { data: Default::default(), removed_ids: Default::default() }
    }
}
