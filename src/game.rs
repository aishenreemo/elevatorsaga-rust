use std::time::SystemTime;

pub struct Game {
    pub last_color_update: SystemTime,
    pub timestamp: SystemTime,
    pub color_index: usize,
}

pub fn init_game() -> Game {
    Game {
        last_color_update: SystemTime::now(),
        timestamp: SystemTime::now(),
        color_index: 0,
    }
}
