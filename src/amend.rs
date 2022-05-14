use crate::config::Config;
use crate::game::ElevatorMoveState;
use crate::game::Game;
use crate::Command;
use crate::Error;

use sdl2::render::WindowCanvas;

fn elevator_up(game: &mut Game) {
    let elevator = &mut game.elevators[0];
    let pos = elevator.destination.unwrap_or(elevator.position);

    if pos + 1 == game.floors_length {
        return;
    }

    elevator.move_state = ElevatorMoveState::Up;
    elevator.destination = Some(pos + 1);
}

fn elevator_down(game: &mut Game) {
    let elevator = &mut game.elevators[0];
    let pos = elevator.destination.unwrap_or(elevator.position);
    if pos == 0 {
        return;
    }

    elevator.move_state = ElevatorMoveState::Down;
    elevator.destination = Some(pos - 1);
}

pub fn update(
    game: &mut Game,
    canvas: &WindowCanvas,
    _cfg: &Config,
    commands: &[Command],
) -> Result<(), Error> {
    game.window_size = canvas.output_size()?;

    for elevator in game.elevators.iter_mut() {
        if elevator.move_state != ElevatorMoveState::Idle {
            let pos = elevator.destination.unwrap_or(elevator.position);
            let diff = (pos as i32 - elevator.position as i32).abs() as f32;

            if elevator.vertical_offset >= 1.0 {
                elevator.move_state = ElevatorMoveState::Idle;
                elevator.position = elevator.destination.take().unwrap();
                elevator.vertical_offset = 0.0;
                continue;
            }

            elevator.vertical_offset += 0.05 / diff;
        }
    }

    for command in commands.iter() {
        match command {
            Command::Quit => std::process::exit(0),
            Command::ElevatorUp => elevator_up(game),
            Command::ElevatorDown => elevator_down(game),
        }
    }

    Ok(())
}
