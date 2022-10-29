use super::input::InputEvent;
use crate::renderer::context::RendererContext;
use lemao_winapi::bindings::winapi;
use std::ffi::CString;
use std::mem;
use std::pin::Pin;
use std::ptr;

pub struct WindowContext {
    hwnd: winapi::HWND,
    hdc: winapi::HDC,
    renderer: Option<RendererContext>,

    initialized: bool,
}

impl WindowContext {
    pub fn new(title: &str, width: i32, height: i32) -> Pin<Box<Self>> {
        unsafe {
            let class_cstr = CString::new("LemaoWindow").unwrap();
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
                hCursor: winapi::LoadCursorA(ptr::null_mut(), 32512 as *const i8),
                lpszMenuName: ptr::null_mut(),
            };

            if winapi::RegisterClassA(&wnd_class) == 0 {
                panic!("{}", winapi::GetLastError());
            }

            // We box and pin window context, so the pointer will be always valid
            let mut context = Box::pin(Self { hwnd: ptr::null_mut(), hdc: ptr::null_mut(), renderer: None, initialized: false });

            let title_cstr = CString::new(title).unwrap();
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
                &mut *context as *mut _ as winapi::LPVOID,
            );

            if hwnd.is_null() {
                panic!("{}", winapi::GetLastError());
            }

            // Wait for WM_CREATE, where the context is initialized
            while !context.initialized {}

            context
        }
    }

    pub fn poll_event(&self) -> Option<InputEvent> {
        unsafe {
            let mut message: winapi::MSG = mem::zeroed();
            if winapi::PeekMessageA(&mut message, ptr::null_mut(), 0, 0, winapi::PM_REMOVE) > 0 {
                winapi::TranslateMessage(&message);
                winapi::DispatchMessageA(&message);

                match message.message {
                    winapi::WM_KEYDOWN => Some(message.into()),
                    winapi::WM_KEYUP => Some(message.into()),
                    winapi::WM_CHAR => Some(message.into()),
                    winapi::WM_QUIT => Some(InputEvent::WindowClosed),
                    _ => None,
                }
            } else {
                None
            }
        }
    }

    pub fn get_renderer(&self) -> &RendererContext {
        self.renderer.as_ref().unwrap()
    }

    pub fn swap_buffers(&self) {
        unsafe {
            if !self.hdc.is_null() && winapi::SwapBuffers(self.hdc) == 0 {
                panic!("{}", winapi::GetLastError());
            }
        }
    }

    pub fn close(&self) {
        unsafe {
            if winapi::DestroyWindow(self.hwnd) == 0 {
                panic!("{}", winapi::GetLastError());
            }
        }
    }
}

extern "C" fn wnd_proc(hwnd: winapi::HWND, message: winapi::UINT, w_param: winapi::WPARAM, l_param: winapi::LPARAM) -> winapi::LRESULT {
    unsafe {
        match message {
            winapi::WM_CREATE => {
                let create_struct = &mut *(l_param as *mut winapi::CREATESTRUCT);
                let window = &mut *(create_struct.lpCreateParams as *mut WindowContext);
                let hdc: winapi::HDC = winapi::GetDC(hwnd);

                // Save pointer to the window context, so it can be used in all future events
                winapi::SetWindowLongPtrA(hwnd, winapi::GWLP_USERDATA, window as *mut _ as winapi::LONG_PTR);

                window.hwnd = hwnd;
                window.hdc = hdc;
                window.renderer = Some(RendererContext::new(hdc));
                window.initialized = true;
            }
            winapi::WM_SIZE => {
                let window_ptr = winapi::GetWindowLongPtrA(hwnd, winapi::GWLP_USERDATA);
                let window = &mut *(window_ptr as *mut WindowContext);
                let renderer = window.get_renderer();

                let width = (l_param & 0xffff) as i32;
                let height = (l_param >> 16) as i32;

                renderer.set_viewport(width, height);
            }
            winapi::WM_CLOSE => {
                if winapi::DestroyWindow(hwnd) == 0 {
                    panic!("{}", winapi::GetLastError());
                }

                return 0;
            }
            winapi::WM_DESTROY => {
                let window_ptr = winapi::GetWindowLongPtrA(hwnd, winapi::GWLP_USERDATA);
                let window = &mut *(window_ptr as *mut WindowContext);
                let renderer = window.get_renderer();

                renderer.release();
                window.hwnd = ptr::null_mut();
                window.hdc = ptr::null_mut();

                winapi::PostQuitMessage(0);
                return 0;
            }
            _ => {}
        }

        winapi::DefWindowProcA(hwnd, message, w_param, l_param)
    }
}
