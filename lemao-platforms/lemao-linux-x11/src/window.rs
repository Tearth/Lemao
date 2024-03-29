use crate::bindings::x11;
use crate::input;
use crate::renderer::LinuxX11Renderer;
use lemao_common_platform::input::InputEvent;
use lemao_common_platform::input::Key;
use lemao_common_platform::input::MouseButton;
use lemao_common_platform::input::MouseWheelDirection;
use lemao_common_platform::renderer::RendererPlatformSpecific;
use lemao_common_platform::window::WindowPlatformSpecific;
use lemao_common_platform::window::WindowStyle;
use lemao_math::vec2::Vec2;
use lemao_opengl::bindings::glx;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_void;
use std::ptr;

pub struct WindowX11 {
    pub(crate) display: *mut x11::_XDisplay,
    pub(crate) frame_buffer_config: *mut glx::__GLXFBConfigRec,
    pub(crate) window: u64,

    delete_window_atom: u64,
    keyboard_state: [bool; 256],
    mouse_state: [bool; 16],
    cursor_visible: bool,
    style: WindowStyle,
    position: Vec2,
    size: Vec2,
    last_cursor_position: Vec2,
}

impl WindowX11 {
    pub fn new(title: &str, style: WindowStyle) -> Result<Box<Self>, String> {
        unsafe {
            let display = x11::XOpenDisplay(ptr::null());
            if display.is_null() {
                return Err("Error while creating a new display".to_string());
            }

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
                event_mask: x11::ExposureMask as i64
                    | x11::StructureNotifyMask as i64
                    | x11::ButtonPressMask as i64
                    | x11::ButtonReleaseMask as i64
                    | x11::KeyPressMask as i64
                    | x11::KeyReleaseMask as i64
                    | x11::PointerMotionMask as i64,
                do_not_propagate_mask: 0,
                override_redirect: 1,
                colormap: x11::XCreateColormap(display, x11::XRootWindow(display, screen_id), mem::transmute((*visual_info).visual), x11::AllocNone as i32),
                cursor: 0,
            };

            let window_size = if let WindowStyle::Window { position: _, size } = style { size } else { Vec2::new(1.0, 1.0) };
            let window = x11::XCreateWindow(
                display,
                x11::XRootWindow(display, screen_id),
                0,
                0,
                window_size.x as u32,
                window_size.y as u32,
                0,
                (*visual_info).depth,
                x11::InputOutput,
                mem::transmute((*visual_info).visual),
                x11::CWBackPixel as u64 | x11::CWColormap as u64 | x11::CWBorderPixel as u64 | x11::CWEventMask as u64,
                &mut window_attributes,
            );

            let delete_window_cstr = CString::new("WM_DELETE_WINDOW").unwrap();
            let mut delete_window_atom = x11::XInternAtom(display, delete_window_cstr.as_ptr(), 0);
            x11::XSetWMProtocols(display, window, &mut delete_window_atom, 1);

            let title_cstr = CString::new(title).unwrap();

            x11::XStoreName(display, window, title_cstr.as_ptr());
            x11::XClearWindow(display, window);
            x11::XMapRaised(display, window);

            let mut context = Box::new(Self {
                display,
                frame_buffer_config: best_frame_buffer_config,
                delete_window_atom,
                keyboard_state: [false; 256],
                mouse_state: [false; 16],
                cursor_visible: true,
                window,
                style,
                position: Default::default(),
                size: Default::default(),
                last_cursor_position: Default::default(),
            });

            context.set_style(style)?;
            Ok(context)
        }
    }
}

impl WindowPlatformSpecific for WindowX11 {
    fn poll_event(&mut self) -> Vec<InputEvent> {
        unsafe {
            if x11::XPending(self.display) > 0 {
                let mut event = mem::zeroed();
                x11::XNextEvent(self.display, &mut event);

                match event.type_ as u32 {
                    x11::ConfigureNotify => {
                        if event.xconfigure.width != (self.size.x as i32) || event.xconfigure.height != (self.size.y as i32) {
                            self.size = Vec2::new(event.xconfigure.width as f32, event.xconfigure.height as f32);
                            return vec![InputEvent::WindowSizeChanged(Vec2::new(event.xconfigure.width as f32, event.xconfigure.height as f32))];
                        }
                    }
                    x11::KeyPress => {
                        let keysym = x11::XLookupKeysym(&event.xkey as *const _ as *mut x11::XKeyEvent, 0);
                        let key = input::virtual_key_to_key(keysym as u32);
                        self.keyboard_state[key as usize] = true;

                        let mut buffer = vec![0; 1];
                        let buffer_ptr = buffer.as_mut_ptr() as *mut i8;
                        x11::XLookupString(&mut event.xkey, buffer_ptr, 1, ptr::null_mut(), ptr::null_mut());

                        return vec![InputEvent::KeyPressed(key), InputEvent::CharPressed(char::from_u32(buffer[0] as u32).unwrap())];
                    }
                    x11::KeyRelease => {
                        let keysym = x11::XLookupKeysym(&event.xkey as *const _ as *mut x11::XKeyEvent, 0);
                        let key = input::virtual_key_to_key(keysym as u32);
                        let input_event = InputEvent::KeyPressed(key);
                        self.keyboard_state[key as usize] = false;

                        return vec![input_event];
                    }
                    x11::ButtonPress => {
                        self.mouse_state[(event.xbutton.button as usize) - 1] = true;
                        return vec![match event.xbutton.button {
                            x11::Button1 => InputEvent::MouseButtonPressed(MouseButton::Left, self.get_cursor_position()),
                            x11::Button2 => InputEvent::MouseButtonPressed(MouseButton::Right, self.get_cursor_position()),
                            x11::Button3 => InputEvent::MouseButtonPressed(MouseButton::Middle, self.get_cursor_position()),
                            _ => InputEvent::MouseButtonPressed(MouseButton::Unknown, self.get_cursor_position()),
                        }];
                    }
                    x11::ButtonRelease => {
                        self.mouse_state[(event.xbutton.button as usize) - 1] = false;
                        return vec![match event.xbutton.button {
                            x11::Button1 => InputEvent::MouseButtonReleased(MouseButton::Left, self.get_cursor_position()),
                            x11::Button2 => InputEvent::MouseButtonReleased(MouseButton::Right, self.get_cursor_position()),
                            x11::Button3 => InputEvent::MouseButtonReleased(MouseButton::Middle, self.get_cursor_position()),
                            x11::Button4 => InputEvent::MouseWheelRotated(MouseWheelDirection::Up, self.get_cursor_position()),
                            x11::Button5 => InputEvent::MouseWheelRotated(MouseWheelDirection::Down, self.get_cursor_position()),
                            _ => InputEvent::MouseButtonReleased(MouseButton::Unknown, self.get_cursor_position()),
                        }];
                    }
                    x11::MotionNotify => {
                        let position = Vec2::new(event.xmotion.x as f32, event.xmotion.y as f32);
                        let screen_position = Vec2::new(position.x, self.size.y - position.y);
                        let last_cursor_position = self.last_cursor_position;
                        self.last_cursor_position = screen_position;

                        return vec![InputEvent::MouseMoved(screen_position, last_cursor_position)];
                    }
                    x11::ClientMessage => {
                        if event.xclient.data.l[0] == self.delete_window_atom as i64 {
                            return vec![InputEvent::WindowClosed];
                        }
                    }
                    _ => {}
                }

                Vec::new()
            } else {
                Vec::new()
            }
        }
    }

    fn create_renderer(&mut self) -> Result<Box<dyn RendererPlatformSpecific>, String> {
        unsafe { Ok(Box::new(LinuxX11Renderer::new(self.display, self.frame_buffer_config, self.window))) }
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_style(&self) -> WindowStyle {
        self.style
    }

    fn set_style(&mut self, style: WindowStyle) -> Result<(), String> {
        unsafe {
            match style {
                WindowStyle::Window { position, size } => {
                    let net_wm_state_cstr = CString::new("_NET_WM_STATE").unwrap();
                    let net_wm_state_fullscreen_cstr = CString::new("_NET_WM_STATE_FULLSCREEN").unwrap();

                    let wm_state = x11::XInternAtom(self.display, net_wm_state_cstr.as_ptr(), 1);
                    let wm_fullscreen = x11::XInternAtom(self.display, net_wm_state_fullscreen_cstr.as_ptr(), 1);

                    let mut event: x11::XEvent = mem::zeroed();
                    event.type_ = x11::ClientMessage as i32;
                    event.xclient.window = self.window;
                    event.xclient.format = 32;
                    event.xclient.message_type = wm_state;
                    event.xclient.data.l[0] = 0;
                    event.xclient.data.l[1] = wm_fullscreen as i64;
                    event.xclient.data.l[2] = 0;
                    event.xclient.data.l[3] = 1;

                    x11::XSendEvent(
                        self.display,
                        x11::XDefaultRootWindow(self.display),
                        0,
                        x11::SubstructureNotifyMask as i64 | x11::SubstructureRedirectMask as i64,
                        &mut event,
                    );

                    x11::XMoveWindow(self.display, self.window, position.x as i32, position.y as i32);
                    x11::XResizeWindow(self.display, self.window, size.x as u32, size.y as u32);
                }
                WindowStyle::Borderless | WindowStyle::Fullscreen => {
                    let net_wm_state_cstr = CString::new("_NET_WM_STATE").unwrap();
                    let net_wm_state_fullscreen_cstr = CString::new("_NET_WM_STATE_FULLSCREEN").unwrap();

                    let wm_state = x11::XInternAtom(self.display, net_wm_state_cstr.as_ptr(), 1);
                    let wm_fullscreen = x11::XInternAtom(self.display, net_wm_state_fullscreen_cstr.as_ptr(), 1);

                    let mut event: x11::XEvent = mem::zeroed();
                    event.type_ = x11::ClientMessage as i32;
                    event.xclient.window = self.window;
                    event.xclient.format = 32;
                    event.xclient.message_type = wm_state;
                    event.xclient.data.l[0] = 1;
                    event.xclient.data.l[1] = wm_fullscreen as i64;
                    event.xclient.data.l[2] = 0;
                    event.xclient.data.l[3] = 1;

                    x11::XSendEvent(
                        self.display,
                        x11::XDefaultRootWindow(self.display),
                        0,
                        x11::SubstructureNotifyMask as i64 | x11::SubstructureRedirectMask as i64,
                        &mut event,
                    );
                }
            }

            self.style = style;
        }

        Ok(())
    }

    fn swap_buffers(&self) {
        unsafe {
            glx::glXSwapBuffers(mem::transmute(self.display), self.window);
        }
    }

    fn close(&self) {}

    fn is_key_pressed(&self, key: Key) -> bool {
        self.keyboard_state[key as usize]
    }

    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_state[button as usize]
    }

    fn get_cursor_position(&self) -> Vec2 {
        unsafe {
            let mut window_returned = mem::zeroed();
            let mut root_cursor_position_x = 0;
            let mut root_cursor_position_y = 0;
            let mut window_cursor_position_x = 0;
            let mut window_cursor_position_y = 0;
            let mut mask = 0;

            x11::XQueryPointer(
                self.display,
                self.window,
                &mut window_returned,
                &mut window_returned,
                &mut root_cursor_position_x,
                &mut root_cursor_position_y,
                &mut window_cursor_position_x,
                &mut window_cursor_position_y,
                &mut mask,
            );

            Vec2::new(window_cursor_position_x as f32, self.size.y - window_cursor_position_y as f32)
        }
    }

    fn set_cursor_visibility(&mut self, visible: bool) {
        if self.cursor_visible == visible {
            return;
        }

        self.cursor_visible = visible;
        unsafe {
            match visible {
                true => x11::XFixesShowCursor(self.display, self.window),
                false => x11::XFixesHideCursor(self.display, self.window),
            }
        }
    }

    fn is_cursor_visible(&self) -> bool {
        self.cursor_visible
    }
}
