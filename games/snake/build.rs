fn main() {
    #[cfg(windows)]
    #[cfg(not(debug_assertions))]
    println!("cargo:rustc-link-arg=/EXPORT:NvOptimusEnablement");

    #[cfg(windows)]
    #[cfg(not(debug_assertions))]
    println!("cargo:rustc-link-arg=/EXPORT:AmdPowerXpressRequestHighPerformance");

    #[cfg(windows)]
    link_windows_libs();

    #[cfg(windows)]
    set_windows_icon();
}

#[cfg(windows)]
fn link_windows_libs() {
    println!("cargo:rustc-link-lib=static=gdi32");
    println!("cargo:rustc-link-lib=static=opengl32");
    println!("cargo:rustc-link-search=static=./lemao-openal/libs");
    println!("cargo:rustc-link-lib=static=openal32");
    println!("cargo:rustc-link-lib=static=user32")
}

#[cfg(windows)]
fn set_windows_icon() {
    std::process::Command::new("llvm-rc").arg("./resources.rc").spawn().unwrap().wait().unwrap();
    println!("cargo:rustc-link-arg=./games/snake/resources.res");
}
