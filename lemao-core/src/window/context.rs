use super::input::InputEvent;
use crate::renderer::context::RendererContext;
use crate::renderer::fonts::storage::FontStorage;
use crate::renderer::textures::storage::TextureStorage;
use crate::utils::log;
use lemao_math::vec2::Vec2;
use lemao_winapi::bindings::winapi;
use std::collections::VecDeque;
use std::ffi::CString;
use std::mem;
use std::ptr;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

pub struct WindowContext {
    pub(crate) hwnd: winapi::HWND,
    pub(crate) hdc: winapi::HDC,
    pub(crate) fake: bool,

    style: WindowStyle,
    size: Vec2,
    initialized: bool,
    wnd_proc_events: VecDeque<WndProcEvent>,
}

#[derive(Copy, Clone)]
pub enum WindowStyle {
    Window(Vec2),
    Borderless,
    Fullscreen,
}

pub struct WndProcEvent {
    message: winapi::UINT,
    l_param: winapi::LPARAM,
}

impl WindowContext {
    pub fn new(title: &str, style: WindowStyle) -> Result<Box<Self>, String> {
        unsafe {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let class_cstr = CString::new(format!("LemaoWindow_{}", timestamp)).unwrap();
            let module_handle = winapi::GetModuleHandleA(ptr::null_mut());

            log::debug(&format!("Initializing a new window class {:?}", class_cstr));

            let wnd_class = winapi::WNDCLASS {
                lpfnWndProc: wnd_proc,
                hInstance: module_handle,
                hbrBackground: winapi::COLOR_BACKGROUND as winapi::HBRUSH,
                lpszClassName: class_cstr.as_ptr(),
                style: winapi::CS_OWNDC,
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: ptr::null_mut(),
                hCursor: winapi::LoadCursorA(ptr::null_mut(), 32512 as *const i8),
                lpszMenuName: ptr::null_mut(),
            };

            if winapi::RegisterClassA(&wnd_class) == 0 {
                return Err(format!("Error while initializing a new window class, GetLastError()={}", winapi::GetLastError()));
            }

            let mut context = Box::new(Self {
                hwnd: ptr::null_mut(),
                hdc: ptr::null_mut(),
                fake: false,
                style,
                size: Default::default(),
                initialized: false,
                wnd_proc_events: VecDeque::new(),
            });
            let title_cstr = CString::new(title).unwrap();

            log::debug(&format!("Initializing a new window instance {:?}", title_cstr));

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
                return Err(format!("Error while initializing a new window instance, GetLastError()={}", winapi::GetLastError()));
            }

            log::debug(&format!("New window handle: {:?}", hwnd));
            log::debug("Waiting for WM_CREATE and the rest of initialization");

            // Wait for WM_CREATE, where the context is initialized
            while !context.initialized {}
            log::debug(&format!("Initialization of window with handle {:?} done", hwnd));

            context.set_style(context.style);
            Ok(context)
        }
    }

    pub(crate) fn new_fake() -> Result<Box<Self>, String> {
        unsafe {
            let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
            let class_cstr = CString::new(format!("LemaoWindowInit_{}", timestamp)).unwrap();
            let module_handle = winapi::GetModuleHandleA(ptr::null_mut());

            log::debug(&format!("Initializing a new fake window class {:?}", class_cstr));

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
                fake: true,
                style: WindowStyle::Window(Vec2::new(0.0, 0.0)),
                size: Default::default(),
                initialized: false,
                wnd_proc_events: VecDeque::new(),
            });
            let title_cstr = CString::new("LemaoWindowInit").unwrap();

            log::debug(&format!("Initializing a new fake window instance {:?}", title_cstr));

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

            log::debug(&format!("New window handle: {:?}", hwnd));
            log::debug("Waiting for WM_CREATE and the rest of initialization");

            // Wait for WM_CREATE, where the context is initialized
            while !context.initialized {}
            log::debug(&format!("Initialization of fake window with handle {:?} done", hwnd));

            Ok(context)
        }
    }

    pub fn poll_event(&mut self) -> Option<InputEvent> {
        unsafe {
            let mut event: winapi::MSG = mem::zeroed();

            if winapi::PeekMessageA(&mut event, ptr::null_mut(), 0, 0, winapi::PM_REMOVE) > 0 {
                winapi::TranslateMessage(&event);
                winapi::DispatchMessageA(&event);

                match event.message {
                    winapi::WM_KEYDOWN => return Some(event.into()),
                    winapi::WM_KEYUP => return Some(event.into()),
                    winapi::WM_CHAR => return Some(event.into()),
                    winapi::WM_LBUTTONDOWN => return Some(event.into()),
                    winapi::WM_MBUTTONDOWN => return Some(event.into()),
                    winapi::WM_RBUTTONDOWN => return Some(event.into()),
                    winapi::WM_LBUTTONUP => return Some(event.into()),
                    winapi::WM_MBUTTONUP => return Some(event.into()),
                    winapi::WM_RBUTTONUP => return Some(event.into()),
                    winapi::WM_MOUSEMOVE => return Some(event.into()),
                    winapi::WM_MOUSEWHEEL => return Some(event.into()),
                    winapi::WM_QUIT => return Some(InputEvent::WindowClosed),
                    _ => {}
                }
            }

            if let Some(event) = self.wnd_proc_events.pop_back() {
                match event.message {
                    winapi::WM_SIZE => {
                        let width = (event.l_param & 0xffff) as u32;
                        let height = (event.l_param >> 16) as u32;
                        self.size = Vec2::new(width as f32, height as f32);

                        return Some(InputEvent::WindowSizeChanged(width, height));
                    }
                    _ => return None,
                }
            }

            None
        }
    }

    pub fn create_renderer(&self, textures: Arc<Mutex<TextureStorage>>, fonts: Arc<Mutex<FontStorage>>) -> Result<RendererContext, String> {
        let mut renderer = RendererContext::new(self.hdc, textures, fonts)?;
        renderer.init();
        renderer.init_storages();
        renderer.init_default_camera();
        renderer.init_default_shader()?;
        renderer.set_default_shader();

        Ok(renderer)
    }

    pub fn get_size(&self) -> Vec2 {
        self.size
    }

    pub fn set_style(&mut self, style: WindowStyle) {
        unsafe {
            if let WindowStyle::Fullscreen = style {
                if winapi::ChangeDisplaySettingsA(ptr::null_mut(), 0) != winapi::DISP_CHANGE_SUCCESSFUL as i32 {
                    let x = 10;
                }
            }

            match style {
                WindowStyle::Window(size) => {
                    let mut rect = winapi::tagRECT { left: 0, top: 0, right: size.x as i32, bottom: size.y as i32 };
                    winapi::SetWindowLongPtrA(self.hwnd, winapi::GWL_STYLE, (winapi::WS_OVERLAPPEDWINDOW | winapi::WS_VISIBLE) as i64);
                    winapi::AdjustWindowRect(&mut rect, winapi::WS_OVERLAPPEDWINDOW, 0);
                    winapi::MoveWindow(self.hwnd, 0, 0, rect.right - rect.left, rect.bottom - rect.top, 1);
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
                        let x = 10;
                    }
                }
            }

            self.style = style;
        }
    }

    pub fn swap_buffers(&self) {
        unsafe {
            if !self.hdc.is_null() && winapi::SwapBuffers(self.hdc) == 0 {
                log::error(&format!("Error while swapping buffers, GetLastError()={}", winapi::GetLastError()));
            }
        }
    }

    pub fn close(&self) {
        unsafe {
            if winapi::DestroyWindow(self.hwnd) == 0 {
                log::error(&format!("Error while destroying window, GetLastError()={}", winapi::GetLastError()));
            }
        }
    }
}

extern "C" fn wnd_proc(hwnd: winapi::HWND, message: winapi::UINT, w_param: winapi::WPARAM, l_param: winapi::LPARAM) -> winapi::LRESULT {
    unsafe {
        match message {
            winapi::WM_CREATE => {
                log::debug(&format!("Received WM_CREATE for window with handle {:?}", hwnd));

                let create_struct = &mut *(l_param as *mut winapi::CREATESTRUCT);
                let window = &mut *(create_struct.lpCreateParams as *mut WindowContext);
                let hdc: winapi::HDC = winapi::GetDC(hwnd);

                // Save pointer to the window context, so it can be used in all future events
                winapi::SetWindowLongPtrA(hwnd, winapi::GWLP_USERDATA, window as *mut _ as winapi::LONG_PTR);

                window.hwnd = hwnd;
                window.hdc = hdc;
                window.initialized = true;
            }
            winapi::WM_SIZE => {
                let window_ptr = winapi::GetWindowLongPtrA(hwnd, winapi::GWLP_USERDATA);
                let window = &mut *(window_ptr as *mut WindowContext);

                window.wnd_proc_events.push_front(WndProcEvent { message, l_param });
            }
            winapi::WM_CLOSE => {
                log::debug(&format!("Received WM_CLOSE for window with handle {:?}", hwnd));

                if winapi::DestroyWindow(hwnd) == 0 {
                    panic!("{}", winapi::GetLastError());
                }

                return 0;
            }
            winapi::WM_DESTROY => {
                log::debug(&format!("Received WM_DESTROY for window with handle {:?}", hwnd));

                let window_ptr = winapi::GetWindowLongPtrA(hwnd, winapi::GWLP_USERDATA);
                let window = &mut *(window_ptr as *mut WindowContext);

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
