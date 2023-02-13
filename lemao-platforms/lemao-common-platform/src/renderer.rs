pub trait RendererPlatformSpecific {
    fn set_swap_interval(&self, interval: u32);
    fn close(&self);
}
