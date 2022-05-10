use crate::config::Config;
use crate::game::Game;
use crate::Command;
use crate::Error;

use std::time::SystemTime;

pub fn update(game: &mut Game, cfg: &Config, commands: &[Command]) -> Result<(), Error> {
    let now = SystemTime::now();
    let duration_since = now.duration_since(game.last_color_update)?;

    if duration_since.as_millis() > cfg.time_per_color_change as u128 {
        game.color_index = (game.color_index + 1) % 8;
        game.last_color_update = now;
    }

    for command in commands.iter() {
        match command {
            Command::Quit => std::process::exit(0),
        }
    }

    Ok(())
}
