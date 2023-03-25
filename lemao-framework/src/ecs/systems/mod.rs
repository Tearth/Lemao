use std::any::TypeId;

use super::world::World;
use crate::app::Application;
use lemao_core::lemao_common_platform::input::InputEvent;

pub mod list;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SystemStage {
    Initialization,
    Input,
    GameLogic,
    GameRendering,
    UiLogic,
    UiRendering,
}

pub trait System<G, S, M>
where
    M: Copy + Clone,
{
    fn get_stage(&self) -> SystemStage;
    fn get_type(&self) -> TypeId;
    fn update(&mut self, app: &mut Application<G>, scene: &mut S, world: &mut World<G, S, M>) -> Result<(), String>;
}
