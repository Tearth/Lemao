use super::bus::MessageBus;
use super::commands::CommandBus;
use super::components::manager::ComponentManager;
use super::entities::list::EntityList;
use super::systems::list::SystemList;
use super::systems::SystemStage;
use crate::app::Application;
use std::fmt::Debug;
use std::sync::Arc;
use std::sync::RwLock;

pub struct World<G, S, M>
where
    M: Copy,
{
    pub entities: EntityList,
    pub components: ComponentManager,
    pub systems: Arc<RwLock<SystemList<G, S, M>>>,
    pub commands: CommandBus<G, S, M>,
    pub messages: MessageBus<M>,

    pub initialized: bool,
}

impl<G, S, M> World<G, S, M>
where
    M: Copy + Debug,
{
    pub fn update(&mut self, app: &mut Application<G>, scene: &mut S) -> Result<(), String> {
        let systems = self.systems.clone();
        let mut systems = systems.write().unwrap();

        // Init stage
        if !self.initialized {
            for system in &mut systems.iter_mut().filter(|system| system.get_stage() == SystemStage::Initialization) {
                system.update(app, scene, self)?;
            }

            self.initialized = true;
        }

        // Input stage
        for system in &mut systems.iter_mut().filter(|system| system.get_stage() == SystemStage::Input) {
            system.update(app, scene, self)?;
        }

        // Game logic stage
        let mut first_iteration = true;
        loop {
            let mut clear = !first_iteration;

            for system in &mut systems.iter_mut().filter(|system| system.get_stage() == SystemStage::GameLogic) {
                if first_iteration || !self.messages.is_empty_by_type(system.get_type()) {
                    system.update(app, scene, self)?;
                    clear = false;
                }

                while let Some(command) = self.commands.poll_message() {
                    command.execute(self)?;
                }
            }

            if clear {
                break;
            }

            first_iteration = false;
        }

        // UI logic stage
        for system in &mut systems.iter_mut().filter(|system| system.get_stage() == SystemStage::UiLogic) {
            system.update(app, scene, self)?;
        }

        // Frame begin stage
        for system in &mut systems.iter_mut().filter(|system| system.get_stage() == SystemStage::FrameBegin) {
            system.update(app, scene, self)?;
        }

        // Game rendering stage
        for system in &mut systems.iter_mut().filter(|system| system.get_stage() == SystemStage::GameRendering) {
            system.update(app, scene, self)?;
        }

        // UI rendering stage
        for system in &mut systems.iter_mut().filter(|system| system.get_stage() == SystemStage::UiRendering) {
            system.update(app, scene, self)?;
        }

        // Frame end stage
        for system in &mut systems.iter_mut().filter(|system| system.get_stage() == SystemStage::FrameEnd) {
            system.update(app, scene, self)?;
        }

        Ok(())
    }
}

impl<G, S, M> Default for World<G, S, M>
where
    M: Copy + Debug,
{
    fn default() -> Self {
        Self {
            entities: Default::default(),
            components: Default::default(),
            systems: Arc::new(RwLock::new(SystemList::<G, S, M>::new())),
            commands: CommandBus::new(),
            messages: MessageBus::<M>::new(),
            initialized: false,
        }
    }
}
