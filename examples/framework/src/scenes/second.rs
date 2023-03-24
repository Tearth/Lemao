use crate::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_common_platform::input::Key;
use lemao_core::lemao_math::color::SolidColor;
use lemao_core::lemao_math::vec2::Vec2;
use lemao_core::renderer::fonts::bff;
use lemao_core::renderer::fonts::Font;
use lemao_framework::app::Application;
use lemao_framework::app::Scene;
use lemao_ui::components::label::Label;
use lemao_ui::components::ComponentPosition;
use lemao_ui::context::UiContext;
use std::any::Any;

#[rustfmt::skip]
const DESCRIPTION: &str = 
"Scene 2
Press Enter to switch";

pub struct SecondScene {
    ui: UiContext,
    description_text_id: usize,
}

impl SecondScene {
    pub fn new(app: &mut Application<GlobalAppData>) -> Self {
        Self { ui: UiContext::new(&mut app.renderer).unwrap(), description_text_id: 0 }
    }
}

impl Scene<GlobalAppData> for SecondScene {
    fn on_init(&mut self, app: &mut Application<GlobalAppData>) -> Result<(), String> {
        let font_id = app.renderer.fonts.store(Font::new(&app.renderer, &bff::load("./assets/inconsolata.bff")?)?);

        self.description_text_id = self.ui.components.store(Box::new(Label::new(&mut app.renderer, font_id)?));
        let description_text = self.ui.components.get_and_cast_mut::<Label>(self.description_text_id)?;
        description_text.label_text = DESCRIPTION.to_string();
        description_text.position = ComponentPosition::RelativeToParent(Vec2::new(0.0, 1.0));
        description_text.offset = Vec2::new(5.0, 0.0);
        description_text.anchor = Vec2::new(0.0, 1.0);
        self.ui.components.get_mut(self.ui.main_canvas_id)?.add_child(self.description_text_id);

        Ok(())
    }

    fn on_activation(&mut self, _app: &mut Application<GlobalAppData>) -> Result<(), String> {
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
                }
                InputEvent::WindowClosed => {
                    app.close();
                }
                _ => {}
            }

            self.ui.process_window_event(&mut app.renderer, &event)?;
        }

        self.ui.update(&mut app.renderer)?;

        app.renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
        self.ui.draw(&mut app.renderer, self.description_text_id)?;
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
