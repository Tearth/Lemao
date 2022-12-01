use crate::bindings::x11;
use crate::renderer::LinuxX11Renderer;
use lemao_common_platform::window::WindowPlatformSpecific;
use lemao_common_platform::window::WindowStyle;
use lemao_math::vec2::Vec2;
use lemao_opengl::bindings::glx;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub struct WindowX11 {
    pub(crate) display: *mut x11::_XDisplay,
    pub(crate) screen: *mut x11::Screen,
    pub(crate) frame_buffer_config: *mut glx::__GLXFBConfigRec,
    pub(crate) window: u64,

    style: WindowStyle,
    position: Vec2,
    size: Vec2,
}

impl WindowX11 {
    pub fn new(title: &str, style: WindowStyle) -> Result<Box<Self>, String> {
        unsafe {
            let display = x11::XOpenDisplay(ptr::null());
            if display.is_null() {
                return Err("Error while creating a new display".to_string());
            }

            let screen = x11::XDefaultScreenOfDisplay(display);
            let screen_id = x11::XDefaultScreen(display);

            let attributes = [
                glx::GLX_X_RENDERABLE,
                1,
                glx::GLX_DRAWABLE_TYPE,
                glx::GLX_WINDOW_BIT,
                glx::GLX_RENDER_TYPE,
                glx::GLX_RGBA_BIT,
                glx::GLX_X_VISUAL_TYPE,
                glx::GLX_TRUE_COLOR,
                glx::GLX_RED_SIZE,
                8,
                glx::GLX_GREEN_SIZE,
                8,
                glx::GLX_BLUE_SIZE,
                8,
                glx::GLX_ALPHA_SIZE,
                8,
                glx::GLX_DEPTH_SIZE,
                24,
                glx::GLX_STENCIL_SIZE,
                8,
                glx::GLX_DOUBLEBUFFER,
                1,
                0,
            ];
            let attributes_ptr = attributes.as_ptr() as *const i32;

            let mut frame_buffers_count = 0;
            let frame_buffer_config = glx::glXChooseFBConfig(mem::transmute(display), screen_id, attributes_ptr, &mut frame_buffers_count);
            if frame_buffer_config.is_null() {
                x11::XCloseDisplay(display);
                return Err("Error while creating a new display".to_string());
            }
            let frame_buffer_config_slice = std::slice::from_raw_parts_mut(frame_buffer_config, frame_buffers_count as usize);

            let mut best_frame_buffer_config_index = -1;
            let mut worst_frame_buffer_config_index = -1;
            let mut best_samples = -1;
            let mut worst_samples = 999;

            for i in 0..frame_buffers_count {
                let config = frame_buffer_config_slice[i as usize];
                let visual_info = glx::glXGetVisualFromFBConfig(mem::transmute(display), config);

                if !visual_info.is_null() {
                    let mut samp_buf = 0;
                    let mut samples = 0;

                    glx::glXGetFBConfigAttrib(mem::transmute(display), config, glx::GLX_SAMPLE_BUFFERS as i32, &mut samp_buf);
                    glx::glXGetFBConfigAttrib(mem::transmute(display), config, glx::GLX_SAMPLES as i32, &mut samples);

                    if best_frame_buffer_config_index < 0 || (samp_buf != 0 && samples > best_samples) {
                        best_frame_buffer_config_index = i;
                        best_samples = samples;
                    }

                    if worst_frame_buffer_config_index < 0 || samp_buf == 0 || samples < worst_samples {
                        worst_frame_buffer_config_index = i;
                    }

                    worst_samples = samples;
                }

                x11::XFree(visual_info as *mut c_void);
            }

            let best_frame_buffer_config = frame_buffer_config_slice[best_frame_buffer_config_index as usize];
            x11::XFree(frame_buffer_config as *mut c_void);

            let visual_info = glx::glXGetVisualFromFBConfig(mem::transmute(display), best_frame_buffer_config);
            if visual_info.is_null() {
                x11::XCloseDisplay(display);
                return Err("Error while creating a new display".to_string());
            }

            if screen_id != (*visual_info).screen {
                x11::XCloseDisplay(display);
                return Err("Error while creating a new display".to_string());
            }

            let mut window_attributes = x11::XSetWindowAttributes {
                background_pixmap: 0,
                background_pixel: x11::XWhitePixel(display, screen_id),
                border_pixmap: 0,
                border_pixel: x11::XBlackPixel(display, screen_id),
                bit_gravity: 0,
                win_gravity: 0,
                backing_store: 0,
                backing_planes: 0,
                backing_pixel: 0,
                save_under: 0,
                event_mask: 0,
                do_not_propagate_mask: 0,
                override_redirect: 1,
                colormap: x11::XCreateColormap(display, x11::XRootWindow(display, screen_id), mem::transmute((*visual_info).visual), x11::AllocNone as i32),
                cursor: 0,
            };

            let window = x11::XCreateWindow(
                display,
                x11::XRootWindow(display, screen_id),
                0,
                0,
                800,
                600,
                0,
                (*visual_info).depth,
                x11::InputOutput,
                mem::transmute((*visual_info).visual),
                x11::CWBackPixel as u64 | x11::CWColormap as u64 | x11::CWBorderPixel as u64 | x11::CWEventMask as u64,
                &mut window_attributes,
            );

            x11::XClearWindow(display, window);
            x11::XMapRaised(display, window);

            Ok(Box::new(Self {
                display,
                screen,
                frame_buffer_config: best_frame_buffer_config,
                window,
                style,
                position: Default::default(),
                size: Default::default(),
            }))
        }
    }
}

impl WindowPlatformSpecific for WindowX11 {
    fn poll_event(&mut self) -> Option<lemao_common_platform::input::InputEvent> {
        unsafe {
            let mut event = mem::zeroed();
            if x11::XPending(self.display) > 0 {
                x11::XNextEvent(self.display, &mut event);
                Some(lemao_common_platform::input::InputEvent::Unknown)
            } else {
                None
            }
        }
    }

    fn create_renderer(&mut self) -> Result<Box<dyn lemao_common_platform::renderer::RendererPlatformSpecific>, String> {
        Ok(Box::new(LinuxX11Renderer::new(self.display, self.frame_buffer_config, self.window)))
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_style(&self) -> WindowStyle {
        WindowStyle::Fullscreen
    }

    fn set_style(&mut self, style: WindowStyle) -> Result<(), String> {
        Ok(())
    }

    fn swap_buffers(&self) {
        unsafe {
            glx::glXSwapBuffers(mem::transmute(self.display), self.window);
        }
    }

    fn close(&self) {}

    fn is_key_pressed(&self, key: lemao_common_platform::input::Key) -> bool {
        false
    }

    fn is_mouse_button_pressed(&self, button: lemao_common_platform::input::MouseButton) -> bool {
        false
    }

    fn get_cursor_position(&self) -> (i32, i32) {
        (0, 0)
    }

    fn set_cursor_visibility(&self, visible: bool) {}

    fn is_cursor_visible(&self) -> bool {
        false
    }
}
