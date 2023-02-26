use crate::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::Drawable;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_core::utils::storage::StorageItem;
use lemao_framework::app::Application;
use lemao_framework::app::Scene;
use std::any::Any;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Scene 1
Press Enter to switch";

pub struct FirstScene {
    description_text_id: usize,
}

impl FirstScene {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { description_text_id: 0 }
    }
}

impl Scene<GlobalAppData> for FirstScene {
    fn on_init(&mut self, app: &mut Application<GlobalAppData>) -> Result<(), String> {
        let font_storage = app.get_renderer_mut().get_fonts();
        let mut font_storage = font_storage.write().unwrap();
        let font_id = font_storage.store(Box::new(Font::new(app.get_renderer_mut(), &bff::load("./assets/inconsolata.bff")?)?));

        drop(font_storage);

        let description_text = app.get_renderer_mut().create_text(font_id)?;
        self.description_text_id = description_text.get_id();
        description_text.set_text(DESCRIPTION);
        description_text.set_anchor(Vec2::new(0.0, 1.0));
        description_text.set_line_height(20);

        Ok(())
    }

    fn on_activation(&mut self, app: &mut Application<GlobalAppData>) -> Result<(), String> {
        let size = app.get_window().get_size();

        app.get_renderer_mut().set_viewport_size(size);
        app.get_renderer_mut().get_active_camera_mut()?.set_size(size);
        app.get_renderer_mut().get_drawable_mut(self.description_text_id)?.set_position(Vec2::new(5.0, size.y - 0.0));

        Ok(())
    }

    fn on_deactivation(&mut self, _app: &mut Application<GlobalAppData>) -> Result<(), String> {
        Ok(())
    }

    fn on_tick(&mut self, app: &mut Application<GlobalAppData>) -> Result<(), String> {
        while let Some(event) = app.get_window_mut().poll_event() {
            match event {
                InputEvent::KeyPressed(Key::Enter) => {
                    app.switch_to_scene("Scene 2");
                }
                InputEvent::WindowSizeChanged(size) => {
                    app.get_renderer_mut().set_viewport_size(size);
                    app.get_renderer_mut().get_active_camera_mut()?.set_size(size);
                    app.get_renderer_mut().get_drawable_mut(self.description_text_id)?.set_position(Vec2::new(5.0, size.y - 0.0));
                }
                InputEvent::WindowClosed => {
                    app.close();
                }
                _ => {}
            }
        }

        app.get_renderer_mut().clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        app.get_renderer_mut().draw(self.description_text_id)?;
        app.get_window_mut().swap_buffers();

        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
