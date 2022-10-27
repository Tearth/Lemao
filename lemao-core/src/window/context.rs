use lemao_opengl::context::OpenGLContext;
use lemao_winapi::bindings::winapi;
use std::ffi::CString;
use std::mem;
use std::pin::Pin;
use std::ptr;

use super::input::InputEvent;

pub struct WindowContext {
    hwnd: winapi::HWND,
    opengl: Option<OpenGLContext>,

    initialized: bool,
    closed: bool,
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
            let mut context = Box::pin(Self { hwnd: ptr::null_mut(), opengl: None, initialized: false, closed: false });

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
            let mut msg: winapi::MSG = mem::zeroed();
            if winapi::PeekMessageA(&mut msg, ptr::null_mut(), 0, 0, winapi::PM_REMOVE) > 0 {
                match msg.message {
                    winapi::WM_QUIT => Some(InputEvent::WindowClosed),
                    winapi::WM_KEYDOWN => {
                        winapi::TranslateMessage(&msg);
                        winapi::DispatchMessageA(&msg);
                        Some(msg.into())
                    }
                    winapi::WM_CHAR => Some(msg.into()),
                    _ => None,
                }
            } else {
                None
            }
        }
    }

    pub fn is_running(&self) -> bool {
        !self.closed
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
                let context = &mut *(create_struct.lpCreateParams as *mut WindowContext);

                let pixel_format_descriptor = winapi::PIXELFORMATDESCRIPTOR {
                    nSize: mem::size_of::<winapi::PIXELFORMATDESCRIPTOR>() as u16,
                    nVersion: 1,
                    dwFlags: winapi::PFD_DRAW_TO_WINDOW | winapi::PFD_SUPPORT_OPENGL | winapi::PFD_DOUBLEBUFFER,
                    iPixelType: winapi::PFD_TYPE_RGBA as u8,
                    cColorBits: 32,
                    cRedBits: 0,
                    cRedShift: 0,
                    cGreenBits: 0,
                    cGreenShift: 0,
                    cBlueBits: 0,
                    cBlueShift: 0,
                    cAlphaBits: 0,
                    cAlphaShift: 0,
                    cAccumBits: 0,
                    cAccumRedBits: 0,
                    cAccumGreenBits: 0,
                    cAccumBlueBits: 0,
                    cAccumAlphaBits: 0,
                    cDepthBits: 24,
                    cStencilBits: 8,
                    cAuxBuffers: 0,
                    iLayerType: winapi::PFD_MAIN_PLANE as u8,
                    bReserved: 0,
                    dwLayerMask: 0,
                    dwVisibleMask: 0,
                    dwDamageMask: 0,
                };

                let window_device_context: winapi::HDC = winapi::GetDC(hwnd);
                let window_pixel_format = winapi::ChoosePixelFormat(window_device_context, &pixel_format_descriptor);
                if winapi::SetPixelFormat(window_device_context, window_pixel_format, &pixel_format_descriptor) == 0 {
                    panic!("{}", winapi::GetLastError());
                }

                let opengl_context: winapi::HGLRC = winapi::wglCreateContext(window_device_context);
                if winapi::wglMakeCurrent(window_device_context, opengl_context) == 0 {
                    panic!("{}", winapi::GetLastError());
                }

                // Save pointer to the window context, so it can be used in all future events
                winapi::SetWindowLongPtrA(hwnd, winapi::GWLP_USERDATA, context as *mut _ as winapi::LONG_PTR);

                context.hwnd = hwnd;
                context.opengl = Some(Default::default());
                context.initialized = true;
            }
            winapi::WM_CLOSE => {
                if winapi::DestroyWindow(hwnd) == 0 {
                    panic!("{}", winapi::GetLastError());
                }

                return 0;
            }
            winapi::WM_DESTROY => {
                // let window_context_ptr = winapi::GetWindowLongPtrA(hwnd, winapi::GWLP_USERDATA);
                // let window_context = &mut *(window_context_ptr as *mut Pin<Box<WindowContext>>);
                winapi::PostQuitMessage(0);
                return 0;
            }
            _ => {}
        }

        winapi::DefWindowProcA(hwnd, message, w_param, l_param)
    }
}
