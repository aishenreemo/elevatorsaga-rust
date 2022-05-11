use crate::config::Config;

use std::time::SystemTime;

pub struct Game {
    pub timestamp: SystemTime,
    pub window_size: (u32, u32),
    pub floors_length: usize,
}

pub fn init_game(cfg: &Config) -> Game {
    Game {
        timestamp: SystemTime::now(),
        window_size: cfg.window_size,
        floors_length: cfg.floors_length,
    }
}
