use super::world::World;
use std::collections::VecDeque;

pub mod kill;
pub mod spawn;

pub trait Command<G, S, M> {
    fn execute(self: Box<Self>, world: &mut World<G, S, M>) -> Result<(), String>
    where
        M: Copy;
}

#[derive(Default)]
pub struct CommandBus<G, S, M> {
    pub queue: VecDeque<Box<dyn Command<G, S, M>>>,
}

impl<G, S, M> CommandBus<G, S, M> {
    pub fn new() -> Self {
        Self { queue: Default::default() }
    }

    pub fn send(&mut self, command: Box<dyn Command<G, S, M>>) {
        self.queue.push_back(command);
    }

    pub fn poll_message(&mut self) -> Option<Box<dyn Command<G, S, M>>> {
        self.queue.pop_front()
    }
}
