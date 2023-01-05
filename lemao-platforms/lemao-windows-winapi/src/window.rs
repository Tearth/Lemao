use crate::bindings::winapi;
use crate::input;
use crate::renderer::WindowsWinAPIRenderer;
use lemao_common_platform::input::InputEvent;
use lemao_common_platform::input::Key;
use lemao_common_platform::input::MouseButton;
use lemao_common_platform::input::MouseWheelDirection;
use lemao_common_platform::renderer::RendererPlatformSpecific;
use lemao_common_platform::window::WindowPlatformSpecific;
use lemao_common_platform::window::WindowStyle;
use lemao_math::vec2::Vec2;
use std::collections::VecDeque;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct WindowWinAPI {
    pub(crate) hwnd: winapi::HWND,
    pub(crate) hdc: winapi::HDC,
    wnd_proc_events: VecDeque<WndProcEvent>,
    initialized: bool,
    fake: bool,

    style: WindowStyle,
    position: Vec2,
    size: Vec2,
    last_cursor_position: Vec2,
}

pub struct WndProcEvent {
    message: winapi::UINT,
    l_param: winapi::LPARAM,
}

impl WindowWinAPI {
    pub fn new(title: &str, style: WindowStyle) -> Result<Box<Self>, String> {
        unsafe {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let class_cstr = CString::new(format!("LemaoWindow_{}", timestamp)).unwrap();
            let module_handle = winapi::GetModuleHandleA(ptr::null_mut());

            let wnd_class = winapi::WNDCLASS {
                lpfnWndProc: wnd_proc,
                hInstance: module_handle,
                hbrBackground: winapi::COLOR_BACKGROUND as winapi::HBRUSH,
                lpszClassName: class_cstr.as_ptr(),
                style: winapi::CS_OWNDC,
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: winapi::LoadIconA(ptr::null_mut(), 32512 as *const i8),
                hCursor: winapi::LoadCursorA(ptr::null_mut(), 32512 as *const i8),
                lpszMenuName: ptr::null_mut(),
            };

            if winapi::RegisterClassA(&wnd_class) == 0 {
                return Err(format!("Error while initializing a new window class, GetLastError()={}", winapi::GetLastError()));
            }

            let mut context = Box::new(Self {
                hwnd: ptr::null_mut(),
                hdc: ptr::null_mut(),
                wnd_proc_events: VecDeque::new(),
                initialized: false,
                fake: false,
                style,
                position: Default::default(),
                size: Default::default(),
                last_cursor_position: Default::default(),
            });
            let title_cstr = CString::new(title).unwrap();

            let hwnd = winapi::CreateWindowExA(
                0,
                wnd_class.lpszClassName,
                title_cstr.as_ptr(),
                winapi::WS_OVERLAPPEDWINDOW | winapi::WS_VISIBLE,
                0,
                0,
                0,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                module_handle,
                context.as_mut() as *mut _ as winapi::LPVOID,
            );

            if hwnd.is_null() {
                return Err(format!("Error while initializing a new window instance, GetLastError()={}", winapi::GetLastError()));
            }

            // Wait for WM_CREATE, where the context is initialized
            while !context.initialized {}

            context.set_style(context.style)?;
            Ok(context)
        }
    }

    pub(crate) fn new_fake() -> Result<Box<Self>, String> {
        unsafe {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let class_cstr = CString::new(format!("LemaoWindowInit_{}", timestamp)).unwrap();
            let module_handle = winapi::GetModuleHandleA(ptr::null_mut());

            let wnd_class = winapi::WNDCLASS {
                lpfnWndProc: wnd_proc,
                hInstance: module_handle,
                hbrBackground: winapi::COLOR_BACKGROUND as winapi::HBRUSH,
                lpszClassName: class_cstr.as_ptr(),
                style: winapi::CS_OWNDC,
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: ptr::null_mut(),
                hCursor: ptr::null_mut(),
                lpszMenuName: ptr::null_mut(),
            };

            if winapi::RegisterClassA(&wnd_class) == 0 {
                return Err(format!("Error while initializing a new window class, GetLastError()={}", winapi::GetLastError()));
            }

            let mut context = Box::new(Self {
                hwnd: ptr::null_mut(),
                hdc: ptr::null_mut(),
                wnd_proc_events: VecDeque::new(),
                initialized: false,
                fake: true,
                style: WindowStyle::Window { position: Vec2::new(0.0, 0.0), size: Vec2::new(0.0, 0.0) },
                position: Default::default(),
                size: Default::default(),
                last_cursor_position: Default::default(),
            });
            let title_cstr = CString::new("LemaoWindowInit").unwrap();

            let hwnd = winapi::CreateWindowExA(
                0,
                wnd_class.lpszClassName,
                title_cstr.as_ptr(),
                winapi::WS_OVERLAPPEDWINDOW,
                0,
                0,
                0,
                0,
                ptr::null_mut(),
                ptr::null_mut(),
                module_handle,
                context.as_mut() as *mut _ as winapi::LPVOID,
            );

            if hwnd.is_null() {
                return Err(format!("Error while initializing a new fake window instance, GetLastError()={}", winapi::GetLastError()));
            }

            // Wait for WM_CREATE, where the context is initialized
            while !context.initialized {}

            Ok(context)
        }
    }
}

impl WindowPlatformSpecific for WindowWinAPI {
    fn poll_event(&mut self) -> Vec<InputEvent> {
        unsafe {
            let mut event: winapi::MSG = mem::zeroed();

            if winapi::PeekMessageA(&mut event, ptr::null_mut(), 0, 0, winapi::PM_REMOVE) > 0 {
                winapi::TranslateMessage(&event);
                winapi::DispatchMessageA(&event);

                match event.message {
                    winapi::WM_KEYDOWN => return vec![InputEvent::KeyPressed(input::virtual_key_to_key(event.wParam))],
                    winapi::WM_KEYUP => return vec![InputEvent::KeyReleased(input::virtual_key_to_key(event.wParam))],
                    winapi::WM_CHAR => return vec![InputEvent::CharPressed(char::from_u32(event.wParam as u32).unwrap())],
                    winapi::WM_LBUTTONDOWN => return vec![InputEvent::MouseButtonPressed(MouseButton::Left, self.get_cursor_position())],
                    winapi::WM_RBUTTONDOWN => return vec![InputEvent::MouseButtonPressed(MouseButton::Right, self.get_cursor_position())],
                    winapi::WM_MBUTTONDOWN => return vec![InputEvent::MouseButtonPressed(MouseButton::Middle, self.get_cursor_position())],
                    winapi::WM_LBUTTONUP => return vec![InputEvent::MouseButtonReleased(MouseButton::Left, self.get_cursor_position())],
                    winapi::WM_RBUTTONUP => return vec![InputEvent::MouseButtonReleased(MouseButton::Right, self.get_cursor_position())],
                    winapi::WM_MBUTTONUP => return vec![InputEvent::MouseButtonReleased(MouseButton::Middle, self.get_cursor_position())],
                    winapi::WM_MOUSEMOVE => {
                        let position = Vec2::new(((event.lParam as i32) & 0xffff) as f32, ((event.lParam as i32) >> 16) as f32);
                        let screen_position = Vec2::new(position.x, self.size.y - position.y);
                        let last_cursor_position = self.last_cursor_position;
                        self.last_cursor_position = screen_position;

                        return vec![InputEvent::MouseMoved(screen_position, last_cursor_position)];
                    }
                    winapi::WM_MOUSEWHEEL => {
                        let direction = if ((event.wParam as i32) >> 16) > 0 { MouseWheelDirection::Up } else { MouseWheelDirection::Down };
                        return vec![InputEvent::MouseWheelRotated(direction)];
                    }
                    winapi::WM_QUIT => return vec![InputEvent::WindowClosed],
                    _ => {}
                }
            }

            if let Some(event) = self.wnd_proc_events.pop_back() {
                match event.message {
                    winapi::WM_MOVE => {
                        let mut rect = winapi::tagRECT { left: 0, top: 0, right: 0, bottom: 0 };
                        winapi::AdjustWindowRect(&mut rect, winapi::WS_OVERLAPPEDWINDOW | winapi::WS_VISIBLE, 0);

                        let x = (event.l_param & 0xffff) as i32 + rect.left;
                        let y = (event.l_param >> 16) as i32 + rect.top;
                        self.position = Vec2::new(x as f32, y as f32);

                        return vec![InputEvent::WindowMoved(self.position)];
                    }
                    winapi::WM_SIZE => {
                        let width = (event.l_param & 0xffff) as u32;
                        let height = (event.l_param >> 16) as u32;
                        self.size = Vec2::new(width as f32, height as f32);

                        return vec![InputEvent::WindowSizeChanged(self.size)];
                    }
                    _ => return Vec::new(),
                }
            }

            Vec::new()
        }
    }

    fn create_renderer(&mut self) -> Result<Box<dyn RendererPlatformSpecific>, String> {
        unsafe { Ok(Box::new(WindowsWinAPIRenderer::new(self.hdc)?)) }
    }

    fn get_position(&self) -> Vec2 {
        self.position
    }

    fn get_size(&self) -> Vec2 {
        self.size
    }

    fn get_style(&self) -> lemao_common_platform::window::WindowStyle {
        self.style
    }

    fn set_style(&mut self, style: lemao_common_platform::window::WindowStyle) -> Result<(), String> {
        unsafe {
            if let WindowStyle::Fullscreen = style {
                if winapi::ChangeDisplaySettingsA(ptr::null_mut(), 0) != winapi::DISP_CHANGE_SUCCESSFUL as i32 {
                    return Err("Error while changing display data".to_string());
                }
            }

            match style {
                WindowStyle::Window { position, size } => {
                    let mut rect = winapi::tagRECT { left: 0, top: 0, right: size.x as i32, bottom: size.y as i32 };
                    winapi::SetWindowLongPtrA(self.hwnd, winapi::GWL_STYLE, (winapi::WS_OVERLAPPEDWINDOW | winapi::WS_VISIBLE) as i64);
                    winapi::AdjustWindowRect(&mut rect, winapi::WS_OVERLAPPEDWINDOW, 0);
                    winapi::MoveWindow(self.hwnd, position.x as i32, position.y as i32, rect.right - rect.left, rect.bottom - rect.top, 1);
                }
                WindowStyle::Borderless => {
                    let mut rect = mem::zeroed();
                    winapi::GetWindowRect(winapi::GetDesktopWindow(), &mut rect);
                    winapi::SetWindowLongPtrA(
                        self.hwnd,
                        winapi::GWL_STYLE,
                        (winapi::WS_SYSMENU | winapi::WS_POPUP | winapi::WS_CLIPCHILDREN | winapi::WS_CLIPSIBLINGS | winapi::WS_VISIBLE) as i64,
                    );
                    winapi::MoveWindow(self.hwnd, 0, 0, rect.right - rect.left, rect.bottom - rect.top, 1);
                }
                WindowStyle::Fullscreen => {
                    let mut rect = mem::zeroed();
                    winapi::GetWindowRect(winapi::GetDesktopWindow(), &mut rect);
                    winapi::SetWindowLongPtrA(
                        self.hwnd,
                        winapi::GWL_STYLE,
                        (winapi::WS_SYSMENU | winapi::WS_POPUP | winapi::WS_CLIPCHILDREN | winapi::WS_CLIPSIBLINGS | winapi::WS_VISIBLE) as i64,
                    );
                    winapi::MoveWindow(self.hwnd, 0, 0, rect.right - rect.left, rect.bottom - rect.top, 1);

                    let mut mode: winapi::DEVMODE = mem::zeroed();
                    mode.dmSize = mem::size_of::<winapi::DEVMODE>() as u16;
                    mode.dmPelsWidth = (rect.right - rect.left) as u32;
                    mode.dmPelsHeight = (rect.bottom - rect.top) as u32;
                    mode.dmBitsPerPel = 32;
                    mode.dmFields = winapi::DM_PELSWIDTH | winapi::DM_PELSHEIGHT | winapi::DM_BITSPERPEL;
                    if winapi::ChangeDisplaySettingsA(&mut mode, winapi::CDS_FULLSCREEN) != winapi::DISP_CHANGE_SUCCESSFUL as i32 {
                        return Err("Error while changing display data".to_string());
                    }
                }
            }

            self.style = style;

            Ok(())
        }
    }

    fn swap_buffers(&self) {
        unsafe {
            if !self.hdc.is_null() {
                winapi::SwapBuffers(self.hdc);
            }
        }
    }

    fn close(&self) {
        unsafe {
            winapi::DestroyWindow(self.hwnd);
        }
    }

    fn is_key_pressed(&self, key: Key) -> bool {
        unsafe { ((winapi::GetKeyState(input::key_to_virtual_key(key) as i32) as u16) & 0x8000) != 0 }
    }

    fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        unsafe { ((winapi::GetKeyState(input::button_to_virtual_button(button) as i32) as u16) & 0x8000) != 0 }
    }

    fn get_cursor_position(&self) -> Vec2 {
        unsafe {
            let mut point = mem::zeroed();
            winapi::GetCursorPos(&mut point);
            winapi::ScreenToClient(self.hwnd, &mut point);

            Vec2::new(point.x as f32, point.y as f32)
        }
    }

    fn set_cursor_visibility(&mut self, visible: bool) {
        unsafe {
            match visible {
                true => while winapi::ShowCursor(1) < 0 {},
                false => while winapi::ShowCursor(0) >= 0 {},
            };
        }
    }

    fn is_cursor_visible(&self) -> bool {
        unsafe {
            let mut cursor_info: winapi::tagCURSORINFO = mem::zeroed();
            cursor_info.cbSize = mem::size_of::<winapi::tagCURSORINFO>() as u32;
            winapi::GetCursorInfo(&mut cursor_info);

            cursor_info.flags != 0
        }
    }
}

extern "C" fn wnd_proc(hwnd: winapi::HWND, message: winapi::UINT, w_param: winapi::WPARAM, l_param: winapi::LPARAM) -> winapi::LRESULT {
    unsafe {
        match message {
            winapi::WM_CREATE => {
                let create_struct = &mut *(l_param as *mut winapi::CREATESTRUCT);
                let window = &mut *(create_struct.lpCreateParams as *mut WindowWinAPI);
                let hdc: winapi::HDC = winapi::GetDC(hwnd);

                // Save pointer to the window context, so it can be used in all future events
                winapi::SetWindowLongPtrA(hwnd, winapi::GWLP_USERDATA, window as *mut _ as winapi::LONG_PTR);

                window.hwnd = hwnd;
                window.hdc = hdc;
                window.initialized = true;
            }
            winapi::WM_MOVE | winapi::WM_SIZE => {
                let window_ptr = winapi::GetWindowLongPtrA(hwnd, winapi::GWLP_USERDATA);
                let window = &mut *(window_ptr as *mut WindowWinAPI);

                window.wnd_proc_events.push_front(WndProcEvent { message, l_param });
            }
            winapi::WM_CLOSE => {
                if winapi::DestroyWindow(hwnd) == 0 {
                    panic!("{}", winapi::GetLastError());
                }

                return 0;
            }
            winapi::WM_DESTROY => {
                let window_ptr = winapi::GetWindowLongPtrA(hwnd, winapi::GWLP_USERDATA);
                let window = &mut *(window_ptr as *mut WindowWinAPI);

                window.hwnd = ptr::null_mut();
                window.hdc = ptr::null_mut();

                if !window.fake {
                    winapi::PostQuitMessage(0);
                }

                return 0;
            }
            _ => {}
        }

        winapi::DefWindowProcA(hwnd, message, w_param, l_param)
    }
}
