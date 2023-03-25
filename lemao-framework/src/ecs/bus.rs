use std::any::TypeId;
use std::collections::hash_map::Iter;
use std::collections::hash_map::IterMut;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt::Debug;

const MAX_EVENTS_PER_QUEUE: usize = 100;

#[derive(Default)]
pub struct MessageBus<M> {
    queues: HashMap<TypeId, VecDeque<M>>,
}

impl<M> MessageBus<M>
where
    M: Copy + Debug,
{
    pub fn new() -> Self {
        Self { queues: Default::default() }
    }

    pub fn register_receiver<R>(&mut self) -> Result<(), String>
    where
        R: 'static,
    {
        if self.queues.contains_key(&TypeId::of::<R>()) {
            return Err("Receiver already registered".to_string());
        }

        self.queues.insert(TypeId::of::<R>(), VecDeque::new());
        Ok(())
    }

    pub fn broadcast(&mut self, message: M) -> Result<(), String> {
        for queue in &mut self.queues {
            if queue.1.len() > MAX_EVENTS_PER_QUEUE {
                return Err("Too many messages".to_string());
            }

            queue.1.push_back(message);
        }

        Ok(())
    }

    pub fn send_to_1<R1>(&mut self, messsage: M) -> Result<(), String>
    where
        R1: 'static,
    {
        match self.queues.get_mut(&TypeId::of::<R1>()) {
            Some(queue) => {
                if queue.len() > MAX_EVENTS_PER_QUEUE {
                    return Err("Too many messages".to_string());
                }

                queue.push_back(messsage);
            }
            None => return Err("Received doesn't exist".to_string()),
        }

        Ok(())
    }

    pub fn send_to_2<R1, R2>(&mut self, messsage: M) -> Result<(), String>
    where
        R1: 'static,
        R2: 'static,
    {
        self.send_to_1::<R1>(messsage)?;
        self.send_to_1::<R2>(messsage)?;
        Ok(())
    }

    pub fn send_to_3<R1, R2, R3>(&mut self, messsage: M) -> Result<(), String>
    where
        R1: 'static,
        R2: 'static,
        R3: 'static,
    {
        self.send_to_1::<R1>(messsage)?;
        self.send_to_1::<R2>(messsage)?;
        self.send_to_1::<R3>(messsage)?;
        Ok(())
    }

    pub fn send_to_4<R1, R2, R3, R4>(&mut self, messsage: M) -> Result<(), String>
    where
        R1: 'static,
        R2: 'static,
        R3: 'static,
        R4: 'static,
    {
        self.send_to_1::<R1>(messsage)?;
        self.send_to_1::<R2>(messsage)?;
        self.send_to_1::<R3>(messsage)?;
        self.send_to_1::<R4>(messsage)?;
        Ok(())
    }

    pub fn is_empty<R>(&self) -> bool
    where
        R: 'static,
    {
        self.is_empty_by_type(TypeId::of::<R>())
    }

    pub fn is_empty_by_type(&self, r#type: TypeId) -> bool {
        match self.queues.get(&r#type) {
            Some(queue) => queue.is_empty(),
            None => true,
        }
    }

    pub fn poll_message<R>(&mut self) -> Option<M>
    where
        R: 'static,
    {
        match self.queues.get_mut(&TypeId::of::<R>()) {
            Some(queue) => queue.pop_front(),
            None => None,
        }
    }

    pub fn iter(&self) -> Iter<TypeId, VecDeque<M>> {
        self.queues.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<TypeId, VecDeque<M>> {
        self.queues.iter_mut()
    }
}
