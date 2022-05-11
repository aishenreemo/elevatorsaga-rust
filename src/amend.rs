use crate::config::Config;
use crate::game::Game;
use crate::Command;
use crate::Error;

use sdl2::render::WindowCanvas;

pub fn update(
    game: &mut Game,
    canvas: &WindowCanvas,
    _cfg: &Config,
    commands: &[Command],
) -> Result<(), Error> {
    game.window_size = canvas.output_size()?;

    for command in commands.iter() {
        match command {
            Command::Quit => std::process::exit(0),
        }
    }

    Ok(())
}
