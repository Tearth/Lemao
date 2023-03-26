use crate::scenes::game::messages::Message;
use crate::scenes::game::scene::GameScene;
use crate::scenes::game::scene::GameWorld;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_core::audio::sounds::Sound;
use lemao_framework::ecs::systems::System;
use lemao_framework::ecs::systems::SystemStage;
use std::any::TypeId;

#[derive(Default)]
pub struct AudioPlayerSystem {}

impl System<GlobalAppData, GameScene, Message> for AudioPlayerSystem {
    fn get_stage(&self) -> SystemStage {
        SystemStage::AudioPlayer
    }

    fn get_type(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn update(&mut self, app: &mut GameApp, scene: &mut GameScene, world: &mut GameWorld) -> Result<(), String> {
        while let Some(message) = world.messages.poll_message::<Self>() {
            match message {
                Message::Init => {
                    scene.state.audio.music_id = app.audio.sounds.store(Sound::new(app.audio.samples.get_by_name("music")?)?);
                    scene.state.audio.click_id = app.audio.sounds.store(Sound::new(app.audio.samples.get_by_name("click")?)?);
                    scene.state.audio.hit_id = app.audio.sounds.store(Sound::new(app.audio.samples.get_by_name("hit")?)?);
                }
                Message::FoodEaten => {
                    app.audio.sounds.get_mut(scene.state.audio.click_id)?.play()?;
                }
                Message::KillSnake => {
                    app.audio.sounds.get_mut(scene.state.audio.hit_id)?.play()?;
                }
                _ => (),
            }
        }

        let music = app.audio.sounds.get_mut(scene.state.audio.music_id)?;
        if !music.is_playing()? {
            music.play()?;
        }

        Ok(())
    }
}
