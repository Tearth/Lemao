use super::world::World;
use crate::app::Application;
use lemao_core::lemao_common_platform::input::InputEvent;

pub type System<G, S> = fn(app: &mut Application<G>, scene: &mut S, world: &mut World<G, S>, input: &[InputEvent]) -> Result<(), String>;
