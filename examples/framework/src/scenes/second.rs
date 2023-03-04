use crate::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::drawable::DrawableEnum;
use lemao_framework::app::Application;
use lemao_framework::app::Scene;
use std::any::Any;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Scene 2
Press Enter to switch";

pub struct SecondScene {
    description_text_id: usize,
}

impl SecondScene {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { description_text_id: 0 }
    }
}

impl Scene<GlobalAppData> for SecondScene {
    fn on_init(&mut self, app: &mut Application<GlobalAppData>) -> Result<(), String> {
        let font_id = app.renderer.create_font("./assets/inconsolata.bff")?;

        self.description_text_id = app.renderer.create_text(font_id)?;
        let description_text = app.renderer.texts.get_mut(self.description_text_id)?;
        description_text.text = DESCRIPTION.to_string();
        description_text.anchor = Vec2::new(0.0, 1.0);
        description_text.line_height = 20;
        description_text.update();

        Ok(())
    }

    fn on_activation(&mut self, app: &mut Application<GlobalAppData>) -> Result<(), String> {
        let size = app.window.get_size();

        app.renderer.set_viewport_size(size)?;
        app.renderer.texts.get_mut(self.description_text_id)?.position = Vec2::new(5.0, size.y - 0.0);

        Ok(())
    }

    fn on_deactivation(&mut self, _app: &mut Application<GlobalAppData>) -> Result<(), String> {
        Ok(())
    }

    fn on_tick(&mut self, app: &mut Application<GlobalAppData>) -> Result<(), String> {
        while let Some(event) = app.window.poll_event() {
            match event {
                InputEvent::KeyPressed(Key::Enter) => {
                    app.switch_to_scene("Scene 1");
                }
                InputEvent::WindowSizeChanged(size) => {
                    app.renderer.set_viewport_size(size)?;
                    app.renderer.texts.get_mut(self.description_text_id)?.position = Vec2::new(5.0, size.y - 0.0);
                }
                InputEvent::WindowClosed => {
                    app.close();
                }
                _ => {}
            }
        }

        app.renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        app.renderer.draw(DrawableEnum::Text, self.description_text_id)?;
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
