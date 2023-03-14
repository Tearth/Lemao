use super::world::World;
use crate::app::Application;
use lemao_core::lemao_common_platform::input::InputEvent;

pub mod list;

pub trait System<G, S, M>
where
    M: Copy + Clone,
{
    fn update(&mut self, app: &mut Application<G>, scene: &mut S, world: &mut World<G, S, M>, input: &[InputEvent]) -> Result<(), String>;
}
