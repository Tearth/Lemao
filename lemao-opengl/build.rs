use std::path::Path;

fn main() {
    let winapi_binging_path = "./src/bindings/opengl.rs";
    if !Path::new(winapi_binging_path).exists() {
        lemao_bindgen::Builder::default()
            .header("./src/headers/glcorearb.h")
            .clang_args(&["-I./src/headers/"])
            .parse_callbacks(Box::new(lemao_bindgen::CargoCallbacks))
            .layout_tests(false)
            .allowlist_file("./src/headers/glcorearb.h")
            .allowlist_file("./src/headers/khrplatform.h")
            .generate()
            .unwrap()
            .write_to_file(winapi_binging_path)
            .unwrap();
    }
}
