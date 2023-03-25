use std::any::TypeId;

use crate::scenes::game::messages::Message;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::app::Application;
use lemao_framework::ecs::systems::{System, SystemStage};
use lemao_framework::ecs::world::World;

#[derive(Default)]
pub struct WindowSystem {}

impl System<GlobalAppData, GameScene, Message> for WindowSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::GameLogic
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<WindowSystem>()
    }

    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        _scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
    ) -> Result<(), String> {
        while let Some(message) = world.messages.poll_message::<Self>() {
            match message {
                Message::InputEvent(event) => match event {
                    InputEvent::WindowSizeChanged(size) => {
                        app.renderer.set_viewport_size(size)?;

                        let window_size = app.window.get_size();
                        let mut camera = app.renderer.cameras.get_mut(app.renderer.active_camera_id)?;
                        camera.position = Vec2::new(
                            -(window_size.x - (app.global_data.board_width as f32 * app.global_data.cell_size.x)) / 2.0,
                            -(window_size.y - (app.global_data.board_height as f32 * app.global_data.cell_size.y)) / 2.0,
                        ) - Vec2::new(app.global_data.cell_size.x, app.global_data.cell_size.y) / 2.0;
                        camera.position = camera.position.floor();
                    }
                    InputEvent::WindowClosed => {
                        app.close();
                    }
                    _ => {}
                },
                _ => {}
            }
        }

        Ok(())
    }
}
