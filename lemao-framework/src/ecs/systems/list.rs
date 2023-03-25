use super::{System, SystemStage};
use std::{
    any::TypeId,
    slice::{Iter, IterMut},
};

#[derive(Default)]
pub struct SystemList<G, S, M> {
    data: Vec<Box<dyn System<G, S, M>>>,
}

impl<G, S, M> SystemList<G, S, M> {
    pub fn new() -> Self {
        Self { data: Default::default() }
    }

    pub fn store<T>(&mut self, item: Box<dyn System<G, S, M>>) -> Result<(), String>
    where
        T: 'static,
    {
        self.data.push(item);
        Ok(())
    }

    pub fn iter(&self) -> Iter<Box<dyn System<G, S, M>>> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Box<dyn System<G, S, M>>> {
        self.data.iter_mut()
    }
}
