fn main() {
    println!("cargo:rustc-link-lib=static=user32");
    println!("cargo:rustc-link-lib=static=gdi32");
    println!("cargo:rustc-link-lib=static=opengl32");
    println!("cargo:rustc-link-search=static=./lemao-openal/libs");
    println!("cargo:rustc-link-lib=static=openal32");
}
