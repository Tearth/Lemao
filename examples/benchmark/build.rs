fn main() {
    println!("cargo:rustc-link-arg=/EXPORT:NvOptimusEnablement");
    println!("cargo:rustc-link-arg=/EXPORT:AmdPowerXpressRequestHighPerformance");

    #[cfg(windows)]
    link_windows_libs();
}

#[cfg(windows)]
fn link_windows_libs() {
    println!("cargo:rustc-link-lib=static=gdi32");
    println!("cargo:rustc-link-lib=static=opengl32");
    println!("cargo:rustc-link-search=static=./lemao-openal/libs");
    println!("cargo:rustc-link-lib=static=openal32");
    println!("cargo:rustc-link-lib=static=user32")
}
