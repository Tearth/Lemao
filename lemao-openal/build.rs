use std::path::Path;

fn main() {
    let openal_binging_path = "./src/bindings/openal.rs";
    if !Path::new(openal_binging_path).exists() {
        lemao_bindgen::Builder::default()
            .header("./src/headers/al.h")
            .header("./src/headers/alc.h")
            .clang_args(&["-I./src/headers/"])
            .parse_callbacks(Box::new(lemao_bindgen::CargoCallbacks))
            .layout_tests(false)
            .generate()
            .unwrap()
            .write_to_file(openal_binging_path)
            .unwrap();
    }

    #[cfg(unix)]
    println!("cargo:rustc-link-lib=dylib=openal");
}
