use lemao_core::window::context::WindowContext;

fn main() {
    let window = WindowContext::new("Test", 800, 600);
    while window.is_running() {}
}
