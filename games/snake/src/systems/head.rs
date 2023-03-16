use crate::components::body::BodyComponent;
use crate::components::food::FoodComponent;
use crate::components::head::{HeadComponent, HeadDirection};
use crate::components::position::PositionComponent;
use crate::components::sprite::SpriteComponent;
use crate::global::GlobalAppData;
use crate::messages::Message;
use crate::scenes::game::GameScene;
use lemao_core::lemao_common_platform::input::{InputEvent, Key};
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::app::Application;
use lemao_framework::ecs::commands::kill::KillCommand;
use lemao_framework::ecs::commands::spawn::SpawnCommand;
use lemao_framework::ecs::components::list::ComponentListTrait;
use lemao_framework::ecs::components::ComponentManagerHashMap;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::world::World;
use std::any::TypeId;

#[derive(Default)]
pub struct HeadSystem {}

impl System<GlobalAppData, GameScene, Message> for HeadSystem {
    fn update(
        &mut self,
        app: &mut Application<GlobalAppData>,
        scene: &mut GameScene,
        world: &mut World<GlobalAppData, GameScene, Message>,
        input: &[InputEvent],
    ) -> Result<(), String> {
        for event in input {
            if let InputEvent::KeyPressed(key) = event {
                let heads = world.components.get_many_mut_1::<HeadComponent>();
                let head = heads.iter_mut().next().unwrap();

                match key {
                    Key::KeyW => {
                        if head.direction != HeadDirection::Down {
                            head.direction = HeadDirection::Up
                        }
                    }
                    Key::KeyS => {
                        if head.direction != HeadDirection::Up {
                            head.direction = HeadDirection::Down
                        }
                    }
                    Key::KeyA => {
                        if head.direction != HeadDirection::Right {
                            head.direction = HeadDirection::Left
                        }
                    }
                    Key::KeyD => {
                        if head.direction != HeadDirection::Left {
                            head.direction = HeadDirection::Right
                        }
                    }
                    _ => {}
                }
            }
        }

        while let Some(message) = world.messages.poll_message::<Self>() {
            match message {
                Message::GameTick => {
                    let (heads, bodies, foods, positions) = world.components.get_many_mut_4::<HeadComponent, BodyComponent, FoodComponent, PositionComponent>();
                    let head = heads.iter_mut().next().unwrap();

                    let position = positions.get_mut(head.entity_id)?;
                    let last_row = position.row;
                    let last_col = position.col;
                    let mut new_row = last_row;
                    let mut new_col = last_col;

                    match head.direction {
                        HeadDirection::Up => new_row += 1,
                        HeadDirection::Down => new_row -= 1,
                        HeadDirection::Right => new_col += 1,
                        HeadDirection::Left => new_col -= 1,
                    }

                    position.row = new_row;
                    position.col = new_col;
                    position.changed = true;

                    for food in foods.iter() {
                        let food_position = positions.get(food.entity_id)?;
                        if food_position.row == new_row && food_position.col == new_col {
                            scene.lifetime += 1;

                            for body in bodies.iter_mut() {
                                body.lifetime += 1;
                            }

                            world.commands.send(Box::new(KillCommand::new(food_position.entity_id)));
                        }
                    }

                    let body_id = world.entities.create();
                    let mut body_rectangle = app.renderer.create_rectangle()?;
                    body_rectangle.size = app.global_data.cell_size;
                    body_rectangle.set_texture(app.renderer.textures.get_by_name("body")?);
                    body_rectangle.update();

                    world.commands.send(Box::new(SpawnCommand::new(body_id, BodyComponent::new(body_id, scene.lifetime))));
                    world.commands.send(Box::new(SpawnCommand::new(body_id, PositionComponent::new(body_id, last_row, last_col))));
                    world.commands.send(Box::new(SpawnCommand::new(body_id, SpriteComponent::new(body_id, body_rectangle))));
                }
            }
        }

        Ok(())
    }
}
