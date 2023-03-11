use std::any::Any;
use std::slice::Iter;
use std::slice::IterMut;

pub struct ComponentManager<T>
where
    T: 'static,
{
    id_lookup: Vec<Option<usize>>,
    data: Vec<T>,
}

pub trait ComponentManagerTrait {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T> ComponentManager<T> {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { id_lookup: Default::default(), data: Default::default() }
    }

    pub fn store(&mut self, entity_id: usize, item: T) -> Result<(), String> {
        if self.id_lookup.get(entity_id).is_some() {
            return Err("Component already exists".to_string());
        }

        self.data.push(item);

        if self.id_lookup.len() < self.data.len() {
            self.id_lookup.resize(self.data.len(), None);
        }
        self.id_lookup[entity_id] = Some(self.data.len() - 1);

        Ok(())
    }

    pub fn get(&self, entity_id: usize) -> Result<&T, String> {
        match self.id_lookup.get(entity_id) {
            Some(Some(component_id)) => match self.data.get(*component_id) {
                Some(component) => Ok(component),
                None => Err(format!("Storage item {} not found", entity_id)),
            },
            _ => Err(format!("Storage item {} not found", entity_id)),
        }
    }

    pub fn get_mut(&mut self, entity_id: usize) -> Result<&mut T, String> {
        match self.id_lookup.get_mut(entity_id) {
            Some(Some(component_id)) => match self.data.get_mut(*component_id) {
                Some(component) => Ok(component),
                None => Err(format!("Storage item {} not found", entity_id)),
            },
            _ => Err(format!("Storage item {} not found", entity_id)),
        }
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub fn remove(&mut self, entity_id: usize) -> Result<(), String> {
        if self.id_lookup[entity_id].is_none() {
            return Err("Component not found".to_string());
        }

        let component_index = self.id_lookup[entity_id].unwrap();
        if component_index != self.data.len() {
            let last_id_index = self.id_lookup.len() - 1;
            self.id_lookup[entity_id] = self.id_lookup[last_id_index];
            self.id_lookup[last_id_index] = None;
            self.data[component_index] = self.data.pop().unwrap();
        } else {
            self.id_lookup[entity_id] = None;
            self.data.pop().unwrap();
        }

        Ok(())
    }
}

impl<T> ComponentManagerTrait for ComponentManager<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
