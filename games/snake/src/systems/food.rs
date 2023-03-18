use crate::components::body::BodyComponent;
use crate::components::food::FoodComponent;
use crate::components::head::HeadComponent;
use crate::components::position::PositionComponent;
use crate::components::sprite::SpriteComponent;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use crate::state::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::utils::rand;
use lemao_framework::app::Application;
use lemao_framework::ecs::commands::kill::KillCommand;
use lemao_framework::ecs::commands::spawn::SpawnCommand;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;
use std::time::SystemTime;

#[derive(Default)]
pub struct FoodSystem {}

impl System<GlobalAppData, GameScene, Message> for FoodSystem {
    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
        _input: &[InputEvent],
    ) -> Result<(), String> {
        while let Some(message) = world.messages.poll_message::<Self>() {
            match message {
                Message::GameTick => {
                    let foods = world.components.get_many_mut_1::<FoodComponent>();

                    if foods.is_empty()
                        || scene.state.game.food_last_refresh_time.elapsed().unwrap().as_millis() >= app.global_data.food_refresh_interval as u128
                    {
                        for body in foods.iter_mut() {
                            world.commands.send(Box::new(KillCommand::new(body.entity_id)));
                        }

                        let (heads, bodies, positions) = world.components.get_many_mut_3::<HeadComponent, BodyComponent, PositionComponent>();

                        let mut forbidden_positions = heads.iter().map(|h| positions.get(h.entity_id).unwrap()).collect::<Vec<&PositionComponent>>();
                        forbidden_positions.extend(bodies.iter().map(|h| positions.get(h.entity_id).unwrap()));

                        let mut new_food_positions = Vec::new();
                        for _ in 0..app.global_data.food_refresh_amount {
                            loop {
                                let row = rand::u8(1..app.global_data.board_height - 2);
                                let col = rand::u8(1..app.global_data.board_width - 2);

                                if !forbidden_positions.iter().any(|p| p.row == row && p.col == col) {
                                    new_food_positions.push((row, col));
                                    break;
                                }
                            }
                        }

                        for position in new_food_positions {
                            let food_id = world.entities.create();
                            let mut food_rectangle = app.renderer.create_rectangle()?;
                            food_rectangle.size = app.global_data.cell_size;
                            food_rectangle.set_texture(app.renderer.textures.get_by_name("food")?);
                            food_rectangle.update();

                            world.commands.send(Box::new(SpawnCommand::new(food_id, FoodComponent::new(food_id))));
                            world.commands.send(Box::new(SpawnCommand::new(food_id, PositionComponent::new(food_id, position.0, position.1))));
                            world.commands.send(Box::new(SpawnCommand::new(food_id, SpriteComponent::new(food_id, food_rectangle))));
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
