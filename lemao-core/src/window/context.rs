use super::input::InputEvent;
use crate::renderer::context::RendererContext;
use crate::renderer::textures::storage::TextureStorage;
use crate::utils::log;
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

    initialized: bool,
    wnd_proc_events: VecDeque<WndProcEvent>,
}

pub struct WndProcEvent {
    message: winapi::UINT,
    l_param: winapi::LPARAM,
}

impl WindowContext {
    pub fn new(title: &str, width: i32, height: i32) -> Result<Box<Self>, String> {
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

            let mut context = Box::new(Self { hwnd: ptr::null_mut(), hdc: ptr::null_mut(), fake: false, initialized: false, wnd_proc_events: VecDeque::new() });
            let title_cstr = CString::new(title).unwrap();

            log::debug(&format!("Initializing a new window instance {:?}", title_cstr));

            let hwnd = winapi::CreateWindowExA(
                0,
                wnd_class.lpszClassName,
                title_cstr.as_ptr(),
                winapi::WS_OVERLAPPEDWINDOW | winapi::WS_VISIBLE,
                0,
                0,
                width,
                height,
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

            let mut context = Box::new(Self { hwnd: ptr::null_mut(), hdc: ptr::null_mut(), fake: true, initialized: false, wnd_proc_events: VecDeque::new() });
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
                    winapi::WM_QUIT => return Some(InputEvent::WindowClosed),
                    _ => {}
                }
            }

            if let Some(event) = self.wnd_proc_events.pop_back() {
                match event.message {
                    winapi::WM_SIZE => {
                        let width = (event.l_param & 0xffff) as u32;
                        let height = (event.l_param >> 16) as u32;

                        return Some(InputEvent::WindowSizeChanged(width, height));
                    }
                    _ => return None,
                }
            }

            None
        }
    }

    pub fn create_renderer(&self, textures: Arc<Mutex<TextureStorage>>) -> Result<RendererContext, String> {
        let mut renderer = RendererContext::new(self.hdc, textures)?;
        renderer.init();
        renderer.init_storages();
        renderer.init_default_shader();
        renderer.set_default_shader();

        Ok(renderer)
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

                window.wnd_proc_events.push_back(WndProcEvent { message, l_param });
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
