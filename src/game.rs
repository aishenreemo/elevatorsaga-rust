use crate::config::Config;

use std::time::SystemTime;

pub struct Game {
    pub timestamp: SystemTime,
    pub window_size: (u32, u32),
    pub floors_length: usize,
    pub elevators: Vec<Elevator>,
}

#[derive(Copy, Clone)]
pub struct Elevator {
    pub position: usize,
    pub capacity: usize,
}

impl Default for Elevator {
    fn default() -> Self {
        Self {
            position: 0,
            capacity: 4,
        }
    }
}

pub fn init_game(cfg: &Config) -> Game {
    Game {
        timestamp: SystemTime::now(),
        window_size: cfg.window_size,
        floors_length: cfg.floors_length,
        elevators: vec![Elevator::default(); cfg.elevators_length],
    }
}
