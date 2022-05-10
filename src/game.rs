use std::time::SystemTime;

pub struct Game {
    pub timestamp: SystemTime,
}

pub fn init_game() -> Game {
    Game {
        timestamp: SystemTime::now(),
    }
}
