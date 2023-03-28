use std::time::SystemTime;

#[derive(Debug)]
pub struct GameState {
    pub score: u32,
    pub best_score: u32,
    pub tick_length: u32,
    pub time_of_last_tick: SystemTime,
    pub lifetime: u32,
    pub food_last_refresh_time: SystemTime,
    pub snake_killed: bool,
    pub snake_killed_time: SystemTime,
    pub game_start_time: SystemTime,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            score: 0,
            best_score: 0,
            tick_length: 0,
            time_of_last_tick: SystemTime::now(),
            lifetime: 0,
            food_last_refresh_time: SystemTime::now(),
            snake_killed: false,
            snake_killed_time: SystemTime::now(),
            game_start_time: SystemTime::now(),
        }
    }
}
