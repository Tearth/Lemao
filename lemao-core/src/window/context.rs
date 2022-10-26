use lemao_winapi::bindings::winapi;
use std::ffi::CString;

pub struct WindowContext {
    pub hwnd: winapi::HWND,
}

impl WindowContext {
    pub fn new(title: &str, width: i32, height: i32) -> Self {
        unsafe {
            let class_cstr = CString::new("LemaoWindow").unwrap();
            let module_handle = winapi::GetModuleHandleA(std::ptr::null_mut());

            let wnd_class = winapi::WNDCLASS {
                lpfnWndProc: wnd_proc,
                hInstance: module_handle,
                hbrBackground: winapi::COLOR_BACKGROUND as winapi::HBRUSH,
                lpszClassName: class_cstr.as_ptr(),
                style: winapi::CS_OWNDC,
                cbClsExtra: 0,
                cbWndExtra: 0,
                hIcon: std::ptr::null_mut(),
                hCursor: winapi::LoadCursorA(std::ptr::null_mut(), 32512 as *const i8),
                lpszMenuName: std::ptr::null_mut(),
            };

            if winapi::RegisterClassA(&wnd_class) == 0 {
                panic!("{}", winapi::GetLastError());
            }

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
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                module_handle,
                std::ptr::null_mut(),
            );

            if hwnd.is_null() {
                panic!("{}", winapi::GetLastError());
            }

            Self { hwnd }
        }
    }

    pub fn is_running(&self) -> bool {
        unsafe {
            let mut msg: winapi::MSG = std::mem::zeroed();
            if winapi::PeekMessageA(&mut msg, self.hwnd, 0, 0, winapi::PM_REMOVE) > 0 {
                if msg.message == winapi::WM_QUIT {
                    return false;
                } else {
                    winapi::TranslateMessage(&msg);
                    winapi::DispatchMessageA(&msg);
                }
            }

            true
        }
    }
}

unsafe extern "C" fn wnd_proc(hwnd: winapi::HWND, message: winapi::UINT, w_param: winapi::WPARAM, l_param: winapi::LPARAM) -> winapi::LRESULT {
    match message {
        winapi::WM_CREATE => {
            let pixel_format_descriptor = winapi::PIXELFORMATDESCRIPTOR {
                nSize: std::mem::size_of::<winapi::PIXELFORMATDESCRIPTOR>() as u16,
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
            winapi::SetPixelFormat(window_device_context, window_pixel_format, &pixel_format_descriptor);

            let opengl_context: winapi::HGLRC = winapi::wglCreateContext(window_device_context);
            winapi::wglMakeCurrent(window_device_context, opengl_context);
        }
        winapi::WM_DESTROY => {
            std::process::exit(0);
        }
        _ => {}
    }

    winapi::DefWindowProcA(hwnd, message, w_param, l_param)
}
