use std::any::TypeId;
use std::time::SystemTime;

use crate::components::body::BodyComponent;
use crate::components::food::FoodComponent;
use crate::components::head::HeadComponent;
use crate::components::position::PositionComponent;
use crate::components::sprite::SpriteComponent;
use crate::global::GlobalAppData;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::utils::rand;
use lemao_framework::app::Application;
use lemao_framework::ecs::components::ComponentManager;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;

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
        while let Some(message) = world.bus.poll_message::<Self>() {
            match message {
                Message::GameTick => {
                    let foods = world.components.get_mut(&TypeId::of::<FoodComponent>()).unwrap().clone();
                    let mut foods_lock = foods.write().unwrap();
                    let foods = foods_lock.as_any_mut().downcast_mut::<ComponentManager<FoodComponent>>().unwrap();

                    if foods.is_empty() || scene.food_last_refresh_time.elapsed().unwrap().as_millis() >= app.global_data.food_refresh_interval as u128 {
                        let mut entites_to_remove = Vec::new();
                        for body in foods.iter_mut() {
                            entites_to_remove.push(body.entity_id);
                        }

                        drop(foods_lock);

                        for entity_id in entites_to_remove {
                            world.remove_entity(entity_id)?;
                        }

                        let foods = world.components.get_mut(&TypeId::of::<FoodComponent>()).unwrap().clone();
                        let mut foods_lock = foods.write().unwrap();
                        let foods = foods_lock.as_any_mut().downcast_mut::<ComponentManager<FoodComponent>>().unwrap();

                        let heads = world.components.get_mut(&TypeId::of::<HeadComponent>()).unwrap().clone();
                        let mut heads_lock = heads.read().unwrap();
                        let heads = heads_lock.as_any().downcast_ref::<ComponentManager<HeadComponent>>().unwrap();

                        let bodies = world.components.get_mut(&TypeId::of::<BodyComponent>()).unwrap().clone();
                        let mut bodies_lock = bodies.read().unwrap();
                        let bodies = bodies_lock.as_any().downcast_ref::<ComponentManager<BodyComponent>>().unwrap();

                        let positions = world.components.get_mut(&TypeId::of::<PositionComponent>()).unwrap().clone();
                        let mut positions_lock = positions.read().unwrap();
                        let positions = positions_lock.as_any().downcast_ref::<ComponentManager<PositionComponent>>().unwrap();

                        let mut forbidden_positions = heads.iter().map(|h| positions.get(h.entity_id).unwrap()).collect::<Vec<&PositionComponent>>();
                        forbidden_positions.extend(bodies.iter().map(|h| positions.get(h.entity_id).unwrap()));

                        let mut new_food_positions = Vec::new();
                        for _ in 0..app.global_data.food_refresh_amount {
                            let mut row = 0;
                            let mut col = 0;

                            loop {
                                row = rand::u8(1..app.global_data.board_height - 2);
                                col = rand::u8(1..app.global_data.board_width - 2);

                                if !forbidden_positions.iter().any(|p| p.row == row && p.col == col) {
                                    break;
                                }
                            }

                            new_food_positions.push((row, col));
                        }

                        drop(foods_lock);
                        drop(heads_lock);
                        drop(bodies_lock);
                        drop(positions_lock);

                        for position in new_food_positions {
                            let food_id = world.create_entity();
                            let mut food_rectangle = app.renderer.create_rectangle()?;
                            food_rectangle.size = app.global_data.cell_size;
                            food_rectangle.set_texture(app.renderer.textures.get_by_name("food")?);
                            food_rectangle.update();

                            world.create_component(food_id, FoodComponent::new(food_id))?;
                            world.create_component(food_id, PositionComponent::new(food_id, position.0, position.1))?;
                            world.create_component(food_id, SpriteComponent::new(food_id, food_rectangle))?;
                        }

                        scene.food_last_refresh_time = SystemTime::now();
                    }
                }
            }
        }

        Ok(())
    }
}
