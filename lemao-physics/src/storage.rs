use crate::body::Body;
use std::collections::VecDeque;
use std::slice::Iter;
use std::slice::IterMut;

#[derive(Default)]
pub struct PhysicsStorage {
    id_lookup: Vec<Option<usize>>,
    removed_ids: VecDeque<usize>,
    data: Vec<Body>,
}

impl PhysicsStorage {
    pub fn store(&mut self, mut body: Body) -> Result<(), String> {
        let body_id = self.get_new_id();
        body.id = body_id;
        self.data.push(body);

        if body_id + 1 > self.id_lookup.len() {
            self.id_lookup.resize(body_id + 1, None);
        }

        self.id_lookup[body_id] = Some(self.data.len() - 1);
        Ok(())
    }

    pub fn get(&self, entity_id: usize) -> Result<&Body, String> {
        match self.id_lookup.get(entity_id) {
            Some(Some(body_id)) => match self.data.get(*body_id) {
                Some(body) => Ok(body),
                None => Err(format!("Body {} not found", entity_id)),
            },
            _ => Err(format!("Body {} not found", entity_id)),
        }
    }

    pub fn get_mut(&mut self, body_id: usize) -> Result<&mut Body, String> {
        match self.id_lookup.get_mut(body_id) {
            Some(Some(body_id)) => match self.data.get_mut(*body_id) {
                Some(body) => Ok(body),
                None => Err(format!("Body {} not found", body_id)),
            },
            _ => Err(format!("Body {} not found", body_id)),
        }
    }

    pub fn get_mut_2(&mut self, body1_id: usize, body2_id: usize) -> Result<(&mut Body, &mut Body), String> {
        unsafe {
            let body1 = self.get_mut(body1_id)? as *mut _ as *mut Body;
            let body2 = self.get_mut(body2_id)? as *mut _ as *mut Body;

            Ok((&mut *body1, &mut *body2))
        }
    }

    pub fn iter(&self) -> Iter<Body> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<Body> {
        self.data.iter_mut()
    }

    pub fn is_empty(&mut self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&mut self) -> usize {
        self.data.len()
    }

    fn get_new_id(&mut self) -> usize {
        if let Some(id) = self.removed_ids.pop_front() {
            id
        } else {
            self.id_lookup.push(None);
            self.id_lookup.len() - 1
        }
    }
}
