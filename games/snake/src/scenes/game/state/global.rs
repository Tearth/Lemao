use lemao_core::lemao_math::vec2::Vec2;

pub struct GlobalAppData {
    pub board_width: u8,
    pub board_height: u8,
    pub cell_size: Vec2,
    pub food_refresh_interval: u32,
    pub food_refresh_amount: u8,

    pub initial_tick_length: u32,
    pub initial_lifetime: u32,
}

impl Default for GlobalAppData {
    fn default() -> Self {
        Self {
            board_width: 40,
            board_height: 20,
            cell_size: Vec2::new(24.0, 24.0),
            food_refresh_interval: 20000,
            food_refresh_amount: 30,
            initial_tick_length: 500,
            initial_lifetime: 3,
        }
    }
}
