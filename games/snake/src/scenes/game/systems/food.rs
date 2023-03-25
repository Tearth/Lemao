use crate::scenes::game::components::body::BodyComponent;
use crate::scenes::game::components::food::FoodComponent;
use crate::scenes::game::components::head::HeadComponent;
use crate::scenes::game::components::position::PositionComponent;
use crate::scenes::game::components::sprite::SpriteComponent;
use crate::scenes::game::messages::Message;
use crate::scenes::game::utils::Coordinates;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::utils::rand;
use lemao_framework::app::Application;
use lemao_framework::ecs::commands::kill::KillCommand;
use lemao_framework::ecs::commands::spawn::SpawnCommand;
use lemao_framework::ecs::systems::{System, SystemStage};
use lemao_framework::ecs::world::World;
use std::any::TypeId;
use std::time::SystemTime;

use super::LAYER_FOOD;

#[derive(Default)]
pub struct FoodSystem {}

impl System<GlobalAppData, GameScene, Message> for FoodSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::GameLogic
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<FoodSystem>()
    }

    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
    ) -> Result<(), String> {
        while let Some(message) = world.messages.poll_message::<Self>() {
            match message {
                Message::GameTick => {
                    let foods = world.components.get_and_cast_mut::<FoodComponent>()?;

                    if foods.is_empty()
                        || scene.state.game.food_last_refresh_time.elapsed().unwrap().as_millis() >= app.global_data.food_refresh_interval as u128
                    {
                        for body in foods.iter_mut() {
                            world.commands.send(KillCommand::new(body.entity_id));
                        }

                        let (heads, bodies, positions) = world.components.get_and_cast_mut_3::<HeadComponent, BodyComponent, PositionComponent>();

                        let mut forbidden_positions = heads.iter().map(|h| positions.get(h.entity_id).unwrap()).collect::<Vec<&PositionComponent>>();
                        forbidden_positions.extend(bodies.iter().map(|h| positions.get(h.entity_id).unwrap()));

                        let mut new_food_positions = Vec::new();
                        for _ in 0..app.global_data.food_refresh_amount {
                            loop {
                                let row = rand::u8(1..app.global_data.board_height - 2);
                                let col = rand::u8(1..app.global_data.board_width - 2);

                                if !forbidden_positions.iter().any(|p| p.coordinates.row == row && p.coordinates.col == col) {
                                    new_food_positions.push((row, col));
                                    break;
                                }
                            }
                        }

                        for position in new_food_positions {
                            let food_id = world.entities.create();
                            let mut food_rectangle = app.renderer.create_tilemap(app.renderer.textures.get_by_name("food")?.id)?;
                            food_rectangle.size = app.global_data.cell_size;
                            food_rectangle.anchor = Vec2::new(0.5, 0.5);
                            food_rectangle.update();

                            world.commands.send(SpawnCommand::new(food_id, FoodComponent::new(food_id)));
                            world.commands.send(SpawnCommand::new(food_id, PositionComponent::new(food_id, Coordinates::new(position.0, position.1))));
                            world.commands.send(SpawnCommand::new(food_id, SpriteComponent::new(food_id, food_rectangle, LAYER_FOOD)));
                        }

                        scene.state.game.food_last_refresh_time = SystemTime::now();
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
