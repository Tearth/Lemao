use std::time::SystemTime;

pub struct GameState {
    pub tick_length: u32,
    pub time_of_last_tick: SystemTime,
    pub lifetime: u32,
    pub food_last_refresh_time: SystemTime,
    pub snake_killed: bool,
    pub snake_killed_time: SystemTime,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            tick_length: 0,
            time_of_last_tick: SystemTime::now(),
            lifetime: 0,
            food_last_refresh_time: SystemTime::now(),
            snake_killed: false,
            snake_killed_time: SystemTime::now(),
        }
    }
}
