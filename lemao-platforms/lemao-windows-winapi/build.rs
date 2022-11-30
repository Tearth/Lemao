use std::path::Path;

fn main() {
    let winapi_binging_path = "./src/bindings/winapi.rs";
    if !Path::new(winapi_binging_path).exists() {
        #[cfg(windows)]
        lemao_bindgen::Builder::default()
            .header("C:/Program Files (x86)/Windows Kits/10/Include/10.0.19041.0/um/Windows.h")
            .clang_args(&["-DWIN32_LEAN_AND_MEAN"])
            .parse_callbacks(Box::new(lemao_bindgen::CargoCallbacks))
            .layout_tests(false)
            .blocklist_type("_IMAGE_TLS_DIRECTORY64")
            .blocklist_type("IMAGE_TLS_DIRECTORY64")
            .blocklist_type("PIMAGE_TLS_DIRECTORY64")
            .blocklist_type("IMAGE_TLS_DIRECTORY")
            .blocklist_type("PIMAGE_TLS_DIRECTORY")
            .generate()
            .unwrap()
            .write_to_file(winapi_binging_path)
            .unwrap();
    }
}
