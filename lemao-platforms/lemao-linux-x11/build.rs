use std::path::Path;

fn main() {
    let x11_binging_path = "./src/bindings/x11.rs";
    if !Path::new(x11_binging_path).exists() {
        #[cfg(unix)]
        lemao_bindgen::Builder::default()
            .header("/usr/include/X11/Xlib.h")
            .parse_callbacks(Box::new(lemao_bindgen::CargoCallbacks))
            .layout_tests(false)
            .generate()
            .unwrap()
            .write_to_file(x11_binging_path)
            .unwrap();
    }
}
