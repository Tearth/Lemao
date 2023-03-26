use super::state::MenuState;
use crate::state::global::GlobalAppData;
use crate::GameApp;
use lemao_core::audio::sounds::Sound;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::Color;
use lemao_core::renderer::fonts::Font;
use lemao_core::renderer::textures::Texture;
use lemao_framework::app::Scene;
use lemao_framework::assets::AssetsLoader;
use lemao_ui::components::button::Button;
use lemao_ui::components::panel::Panel;
use lemao_ui::components::ComponentPosition;
use lemao_ui::components::ComponentShape;
use lemao_ui::context::UiContext;
use lemao_ui::events::UiEvent;
use std::any::Any;

pub struct MenuScene {
    pub ui: UiContext,
    pub state: MenuState,
}

impl MenuScene {
    pub fn new(app: &mut GameApp) -> Self {
        Self { ui: UiContext::new(&mut app.renderer).unwrap(), state: Default::default() }
    }
}

impl Scene<GlobalAppData> for MenuScene {
    fn on_init(&mut self, app: &mut GameApp) -> Result<(), String> {
        let mut assets_loader = AssetsLoader::default();
        assets_loader.set_queue("./assets/")?;
        assets_loader.start_loading();

        loop {
            if *assets_loader.loaded_assets.read().unwrap() == assets_loader.total_assets {
                break;
            }
        }

        for texture in assets_loader.textures.read().unwrap().iter() {
            app.renderer.textures.store_with_name(&texture.name, Texture::new(&app.renderer, &texture.data)?)?;
        }

        for font in assets_loader.fonts.read().unwrap().iter() {
            app.renderer.fonts.store_with_name(&font.name, Font::new(&app.renderer, &font.data)?)?;
        }

        for sample in assets_loader.samples.read().unwrap().iter() {
            app.audio.sounds.store_with_name(&sample.name, Sound::new(&sample.data)?)?;
        }

        let font_id = app.renderer.fonts.get_by_name("pixeled")?.id;

        self.state.ui.logo_panel_id = self.ui.components.store(Panel::new(&mut app.renderer, ComponentShape::Rectangle)?);
        let logo_panel = self.ui.components.get_and_cast_mut::<Panel>(self.state.ui.logo_panel_id)?;
        logo_panel.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 1.0));
        logo_panel.anchor = Vec2::new(0.5, 1.0);
        logo_panel.set_texture(app.renderer.textures.get_by_name("logo")?);
        self.ui.components.get_mut(self.ui.main_canvas_id)?.add_child(self.state.ui.logo_panel_id);

        self.state.ui.play_button_id = self.ui.components.store(Button::new(&mut app.renderer, ComponentShape::Rectangle, font_id)?);
        let play_button = self.ui.components.get_and_cast_mut::<Button>(self.state.ui.play_button_id)?;
        play_button.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.5));
        play_button.anchor = Vec2::new(0.5, 0.5);
        play_button.set_texture(app.renderer.textures.get_by_name("button")?);
        play_button.label_text = "PLAY".to_string();
        play_button.label_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
        play_button.on_cursor_enter = Some(|button, _| button.color.set_alpha(0.8));
        play_button.on_cursor_leave = Some(|button, _| button.color.set_alpha(1.0));
        play_button.on_mouse_button_pressed = Some(|button, _, _| button.color.set_alpha(0.6));
        play_button.on_mouse_button_released = Some(|button, _, _| button.color.set_alpha(1.0));
        self.ui.components.get_mut(self.ui.main_canvas_id)?.add_child(self.state.ui.play_button_id);

        self.state.ui.exit_button_id = self.ui.components.store(Button::new(&mut app.renderer, ComponentShape::Rectangle, font_id)?);
        let exit_button = self.ui.components.get_and_cast_mut::<Button>(self.state.ui.exit_button_id)?;
        exit_button.position = ComponentPosition::RelativeToParent(Vec2::new(0.5, 0.5));
        exit_button.anchor = Vec2::new(0.5, 0.5);
        exit_button.offset = Vec2::new(0.0, -80.0);
        exit_button.set_texture(app.renderer.textures.get_by_name("button")?);
        exit_button.label_text = "EXIT".to_string();
        exit_button.label_color = Color::SolidColor(SolidColor::new(0.0, 0.0, 0.0, 1.0));
        exit_button.on_cursor_enter = Some(|button, _| button.color.set_alpha(0.8));
        exit_button.on_cursor_leave = Some(|button, _| button.color.set_alpha(1.0));
        exit_button.on_mouse_button_pressed = Some(|button, _, _| button.color.set_alpha(0.6));
        exit_button.on_mouse_button_released = Some(|button, _, _| button.color.set_alpha(1.0));
        self.ui.components.get_mut(self.ui.main_canvas_id)?.add_child(self.state.ui.exit_button_id);

        Ok(())
    }

    fn on_activation(&mut self, _app: &mut GameApp) -> Result<(), String> {
        Ok(())
    }

    fn on_deactivation(&mut self, _app: &mut GameApp) -> Result<(), String> {
        Ok(())
    }

    fn on_tick(&mut self, app: &mut GameApp) -> Result<(), String> {
        while let Some(event) = app.window.poll_event() {
            match event {
                InputEvent::WindowSizeChanged(size) => {
                    app.renderer.set_viewport_size(size)?;
                }
                InputEvent::WindowClosed => {
                    app.close();
                }
                _ => {}
            }

            self.ui.process_window_event(&mut app.renderer, &event)?;
        }

        while let Some(event) = self.ui.poll_event() {
            if let UiEvent::ButtonPressed(button_id, _) = event {
                if button_id == self.state.ui.play_button_id {
                    app.switch_to_scene("Game");
                } else if button_id == self.state.ui.exit_button_id {
                    app.close();
                }
            }
        }

        app.renderer.clear(SolidColor::new_rgb(210, 150, 100, 255));
        self.ui.update(&mut app.renderer)?;
        self.ui.draw(&mut app.renderer, self.state.ui.logo_panel_id)?;
        self.ui.draw(&mut app.renderer, self.state.ui.play_button_id)?;
        self.ui.draw(&mut app.renderer, self.state.ui.exit_button_id)?;

        app.window.swap_buffers();
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
