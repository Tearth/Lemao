use super::body::BodySystem;
use super::*;
use crate::scenes::game::components::body::BodyComponent;
use crate::scenes::game::components::body::BodyOrientation;
use crate::scenes::game::components::food::FoodComponent;
use crate::scenes::game::components::head::HeadComponent;
use crate::scenes::game::components::position::PositionComponent;
use crate::scenes::game::components::sprite::SpriteComponent;
use crate::scenes::game::messages::Message;
use crate::scenes::game::scene::{GameScene, GameWorld};
use crate::scenes::game::systems::audio::player::AudioPlayerSystem;
use crate::scenes::game::systems::ui::logic::UiLogicSystem;
use crate::scenes::game::utils::Coordinates;
use crate::scenes::game::utils::Direction;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_framework::ecs::commands::kill::KillCommand;
use lemao_framework::ecs::commands::spawn::SpawnCommand;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::systems::SystemStage;
use std::any::TypeId;
use std::cmp;
use std::collections::HashMap;
use std::time::SystemTime;

pub struct HeadSystem {
    orientation_map: HashMap<u8, BodyOrientation>,
}

impl System<GlobalAppData, GameScene, Message> for HeadSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::GameLogic
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn update(&mut self, app: &mut GameApp, scene: &mut GameScene, world: &mut GameWorld) -> Result<(), String> {
        while let Some(message) = world.messages.poll_message::<Self>() {
            match message {
                Message::Init => {
                    let head_id = world.entities.create();
                    let mut head_rectangle = app.renderer.create_tilemap(app.renderer.textures.get_by_name("head")?.id)?;
                    head_rectangle.size = app.global_data.cell_size;
                    head_rectangle.anchor = Vec2::new(0.5, 0.5);
                    head_rectangle.update();

                    let head_coordinates = self.get_head_default_position(app);
                    world.commands.send(SpawnCommand::new(head_id, HeadComponent::new(head_id, Direction::Right)));
                    world.commands.send(SpawnCommand::new(head_id, PositionComponent::new(head_id, head_coordinates, Some(Direction::Right))));
                    world.commands.send(SpawnCommand::new(head_id, SpriteComponent::new(head_id, head_rectangle, LAYER_SNAKE)));

                    scene.state.game.lifetime = app.global_data.initial_lifetime;
                    scene.state.game.tick_length = app.global_data.initial_tick_length;

                    return Ok(());
                }
                Message::InputEvent(InputEvent::KeyPressed(key)) => {
                    let head = world.components.get_and_cast_mut::<HeadComponent>()?.get_mut_first()?;

                    match key {
                        Key::KeyW => {
                            if head.direction != Direction::Down {
                                head.next_direction = Direction::Up
                            }
                        }
                        Key::KeyS => {
                            if head.direction != Direction::Up {
                                head.next_direction = Direction::Down
                            }
                        }
                        Key::KeyA => {
                            if head.direction != Direction::Right {
                                head.next_direction = Direction::Left
                            }
                        }
                        Key::KeyD => {
                            if head.direction != Direction::Left {
                                head.next_direction = Direction::Right
                            }
                        }
                        _ => {}
                    }
                }
                Message::GameTick => {
                    if !scene.state.game.snake_killed {
                        let (heads, bodies, foods, positions) =
                            world.components.get_and_cast_mut_4::<HeadComponent, BodyComponent, FoodComponent, PositionComponent>()?;
                        let head = heads.get_mut_first()?;

                        let position = positions.get_mut(head.entity_id)?;
                        let last_coordinates = position.coordinates;
                        let last_direction = head.direction;
                        let mut new_coordinates = last_coordinates;

                        if head.direction != head.next_direction {
                            position.direction = Some(head.next_direction);
                        }

                        head.direction = head.next_direction;

                        match head.direction {
                            Direction::Up => new_coordinates.row += 1,
                            Direction::Down => new_coordinates.row -= 1,
                            Direction::Right => new_coordinates.col += 1,
                            Direction::Left => new_coordinates.col -= 1,
                        }

                        let mut collision = false;

                        if new_coordinates.row == 0
                            || new_coordinates.row == app.global_data.board_height - 1
                            || new_coordinates.col == 0
                            || new_coordinates.col == app.global_data.board_width - 1
                        {
                            collision = true;
                        }

                        if !collision {
                            for body in bodies.iter().filter(|b| !b.killed) {
                                let body_position = positions.get(body.entity_id)?;
                                if body_position.coordinates == new_coordinates {
                                    collision = true;
                                }
                            }
                        }

                        if collision {
                            scene.state.game.snake_killed = true;
                            scene.state.game.snake_killed_time = SystemTime::now();

                            world.messages.send_to_3::<HeadSystem, BodySystem, AudioPlayerSystem>(Message::KillSnake)?;
                        } else {
                            let position = positions.get_mut(head.entity_id)?;
                            position.coordinates = new_coordinates;
                            position.changed = true;

                            for food in foods.iter() {
                                let food_position = positions.get(food.entity_id)?;
                                if food_position.coordinates == new_coordinates {
                                    scene.state.game.lifetime += 1;
                                    scene.state.game.tick_length = (scene.state.game.tick_length as f32 * 0.95) as u32;
                                    scene.state.game.score += 1;

                                    world.commands.send(KillCommand::new(food_position.entity_id));
                                    world.messages.send_to_3::<BodySystem, UiLogicSystem, AudioPlayerSystem>(Message::FoodEaten)?;
                                }
                            }

                            let key = (8 << last_direction as u8) | (1 << head.direction as u8);
                            let body_orientation = self.orientation_map.get(&key).unwrap();

                            let body_id = world.entities.create();
                            let mut body_rectangle = app.renderer.create_tilemap(app.renderer.textures.get_by_name("body")?.id)?;
                            body_rectangle.size = app.global_data.cell_size;
                            body_rectangle.anchor = Vec2::new(0.5, 0.5);
                            body_rectangle.frame = *body_orientation as u32;
                            body_rectangle.update();

                            world
                                .commands
                                .send(SpawnCommand::new(body_id, BodyComponent::new(body_id, scene.state.game.lifetime, *body_orientation, head.direction)));
                            world.commands.send(SpawnCommand::new(body_id, PositionComponent::new(body_id, last_coordinates, None)));
                            world.commands.send(SpawnCommand::new(body_id, SpriteComponent::new(body_id, body_rectangle, LAYER_SNAKE)));
                        }
                    }
                }
                Message::KillSnake => {
                    let (heads, sprites) = world.components.get_and_cast_mut_2::<HeadComponent, SpriteComponent>()?;
                    let head = heads.get_mut_first()?;
                    let sprite = sprites.get_mut(head.entity_id)?;

                    sprite.blinking = true;
                    sprite.blinking_interval = 200;
                    sprite.blinking_last_change_time = scene.state.game.snake_killed_time;
                }
                Message::ResetSnake => {
                    let (heads, positions, sprites) = world.components.get_and_cast_mut_3::<HeadComponent, PositionComponent, SpriteComponent>()?;
                    let head = heads.get_mut_first()?;

                    head.direction = Direction::Right;
                    head.next_direction = Direction::Right;

                    let head_position = positions.get_mut(head.entity_id)?;
                    head_position.coordinates = self.get_head_default_position(app);
                    head_position.direction = Some(Direction::Right);
                    head_position.changed = true;

                    let sprite = sprites.get_mut(head.entity_id)?;
                    sprite.blinking = false;
                }
                _ => {}
            }
        }

        if scene.state.game.snake_killed && scene.state.game.snake_killed_time.elapsed().unwrap().as_millis() >= 2000 {
            scene.state.game.snake_killed = false;
            scene.state.game.best_score = cmp::max(scene.state.game.score, scene.state.game.best_score);
            scene.state.game.score = 0;
            scene.state.game.lifetime = app.global_data.initial_lifetime;
            scene.state.game.tick_length = app.global_data.initial_tick_length;
            scene.state.game.game_start_time = SystemTime::now();

            world.messages.send_to_3::<HeadSystem, BodySystem, UiLogicSystem>(Message::ResetSnake)?;
        }

        Ok(())
    }
}

impl HeadSystem {
    fn get_head_default_position(&self, app: &mut GameApp) -> Coordinates {
        Coordinates::new(app.global_data.board_height / 2, app.global_data.board_width / 2)
    }
}

impl Default for HeadSystem {
    fn default() -> Self {
        let mut orientation_map = HashMap::new();

        orientation_map.insert((8 << Direction::Up as u8) | (1 << Direction::Up as u8), BodyOrientation::TopBottom);
        orientation_map.insert((8 << Direction::Down as u8) | (1 << Direction::Down as u8), BodyOrientation::TopBottom);
        orientation_map.insert((8 << Direction::Right as u8) | (1 << Direction::Right as u8), BodyOrientation::RightLeft);
        orientation_map.insert((8 << Direction::Left as u8) | (1 << Direction::Left as u8), BodyOrientation::RightLeft);

        orientation_map.insert((8 << Direction::Right as u8) | (1 << Direction::Down as u8), BodyOrientation::LeftBottom);
        orientation_map.insert((8 << Direction::Left as u8) | (1 << Direction::Down as u8), BodyOrientation::RightBottom);
        orientation_map.insert((8 << Direction::Right as u8) | (1 << Direction::Up as u8), BodyOrientation::LeftTop);
        orientation_map.insert((8 << Direction::Left as u8) | (1 << Direction::Up as u8), BodyOrientation::RightTop);
        orientation_map.insert((8 << Direction::Up as u8) | (1 << Direction::Right as u8), BodyOrientation::RightBottom);
        orientation_map.insert((8 << Direction::Up as u8) | (1 << Direction::Left as u8), BodyOrientation::LeftBottom);
        orientation_map.insert((8 << Direction::Down as u8) | (1 << Direction::Right as u8), BodyOrientation::RightTop);
        orientation_map.insert((8 << Direction::Down as u8) | (1 << Direction::Left as u8), BodyOrientation::LeftTop);

        Self { orientation_map }
    }
}
