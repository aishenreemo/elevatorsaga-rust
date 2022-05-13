use crate::config::Config;
use crate::game::Game;
use crate::Command;
use crate::Error;

use sdl2::render::WindowCanvas;

fn elevator_up(game: &mut Game) {
    if game.elevators[0].position == game.floors_length - 1 {
        return;
    }

    game.elevators[0].position += 1;
}

fn elevator_down(game: &mut Game) {
    if game.elevators[0].position == 0 {
        return;
    }

    game.elevators[0].position -= 1;
}

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
            Command::ElevatorUp => elevator_up(game),
            Command::ElevatorDown => elevator_down(game),
        }
    }

    Ok(())
}
