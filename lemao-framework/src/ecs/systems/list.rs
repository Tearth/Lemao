use crate::ecs::components::list::ComponentListTrait;

use super::System;
use std::any::TypeId;
use std::collections::hash_map::Values;
use std::collections::hash_map::ValuesMut;
use std::collections::HashMap;
use std::slice::Iter;
use std::slice::IterMut;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Default)]
pub struct SystemList<G, S, M> {
    data: Vec<Box<dyn System<G, S, M>>>,
}

impl<G, S, M> SystemList<G, S, M> {
    pub fn new() -> Self {
        Self { data: Default::default() }
    }

    pub fn store(&mut self, item: Box<dyn System<G, S, M>>) -> Result<(), String> {
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
