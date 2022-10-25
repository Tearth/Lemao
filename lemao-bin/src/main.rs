use lemao_winapi::bindings::winapi;
use std::ffi::CString;

fn main() {
    let lp_text = CString::new("Hello, world 123!").unwrap();
    let lp_caption = CString::new("MessageBox Example").unwrap();

    unsafe {
        winapi::MessageBoxA(
            std::ptr::null_mut(),
            lp_text.as_ptr(),
            lp_caption.as_ptr(),
            winapi::MB_OK | winapi::MB_ICONINFORMATION,
        );
    }

    println!("Hello, world!");
}
