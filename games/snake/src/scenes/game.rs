use crate::global::GlobalAppData;
use lemao_core::lemao_common_platform::input::InputEvent;
use lemao_core::lemao_math::color::SolidColor;
use lemao_framework::app::Application;
use lemao_framework::app::Scene;
use lemao_ui::context::UiContext;
use std::any::Any;

pub struct GameScene {
    ui: UiContext,
}

impl GameScene {
    pub fn new(app: &mut Application<GlobalAppData>) -> Self {
        Self { ui: UiContext::new(&mut app.renderer).unwrap() }
    }
}

impl Scene<GlobalAppData> for GameScene {
    fn on_init(&mut self, _app: &mut Application<GlobalAppData>) -> Result<(), String> {
        Ok(())
    }

    fn on_activation(&mut self, app: &mut Application<GlobalAppData>) -> Result<(), String> {
        let size = app.window.get_size();

        app.renderer.set_viewport_size(size)?;
        self.ui.process_window_event(&mut app.renderer, &InputEvent::WindowSizeChanged(size))?;

        Ok(())
    }

    fn on_deactivation(&mut self, _app: &mut Application<GlobalAppData>) -> Result<(), String> {
        Ok(())
    }

    fn on_tick(&mut self, app: &mut Application<GlobalAppData>) -> Result<(), String> {
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

        self.ui.update(&mut app.renderer)?;

        app.renderer.clear(SolidColor::new(0.5, 0.5, 0.5, 1.0));
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
