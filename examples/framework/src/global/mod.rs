pub struct GlobalAppData {
    pub test: u32,
}

impl Default for GlobalAppData {
    fn default() -> Self {
        Self { test: 100 }
    }
}
