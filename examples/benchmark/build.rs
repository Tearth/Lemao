fn main() {
    println!("cargo:rustc-link-lib=static=user32");
    println!("cargo:rustc-link-lib=static=gdi32");
    println!("cargo:rustc-link-lib=static=opengl32");
}
