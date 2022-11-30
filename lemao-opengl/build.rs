use std::path::Path;

fn main() {
    build_core_binding();

    #[cfg(windows)]
    build_windows_binding();

    #[cfg(unix)]
    build_linux_binding();
}

fn build_core_binding() {
    let opengl_binging_path = "./src/bindings/opengl.rs";
    if !Path::new(opengl_binging_path).exists() {
        lemao_bindgen::Builder::default()
            .header("./src/headers/glcorearb.h")
            .clang_args(&["-I./src/headers/"])
            .parse_callbacks(Box::new(lemao_bindgen::CargoCallbacks))
            .layout_tests(false)
            .allowlist_file("./src/headers/glcorearb.h")
            .allowlist_file("./src/headers/khrplatform.h")
            .generate()
            .unwrap()
            .write_to_file(opengl_binging_path)
            .unwrap();
    }
}

#[cfg(windows)]
fn build_windows_binding() {
    let wgl_binging_path = "./src/bindings/wgl.rs";
    if !Path::new(wgl_binging_path).exists() {
        lemao_bindgen::Builder::default()
            .header("./src/headers/glcorearb.h")
            .header("./src/headers/wgl.h")
            .clang_args(&["-I./src/headers/"])
            .parse_callbacks(Box::new(lemao_bindgen::CargoCallbacks))
            .layout_tests(false)
            .allowlist_file("./src/headers/wgl.h")
            .generate()
            .unwrap()
            .write_to_file(wgl_binging_path)
            .unwrap();
    }

    let winapi_binging_path = "./src/bindings/winapi.rs";
    if !Path::new(winapi_binging_path).exists() {
        lemao_bindgen::Builder::default()
            .header("C:/Program Files (x86)/Windows Kits/10/Include/10.0.19041.0/um/Windows.h")
            .clang_args(&["-DWIN32_LEAN_AND_MEAN"])
            .parse_callbacks(Box::new(lemao_bindgen::CargoCallbacks))
            .layout_tests(false)
            .allowlist_function("LoadLibraryA")
            .allowlist_function("GetProcAddress")
            .allowlist_function("wglGetProcAddress")
            .generate()
            .unwrap()
            .write_to_file(winapi_binging_path)
            .unwrap();
    }
}

#[cfg(unix)]
fn build_linux_binding() {
    let glx_binging_path = "./src/bindings/glx.rs";
    if !Path::new(glx_binging_path).exists() {
        lemao_bindgen::Builder::default()
            .header("/usr/include/GL/glx.h")
            .parse_callbacks(Box::new(lemao_bindgen::CargoCallbacks))
            .layout_tests(false)
            .allowlist_file("/usr/include/GL/glx.h")
            .generate()
            .unwrap()
            .write_to_file(glx_binging_path)
            .unwrap();
    }
}
