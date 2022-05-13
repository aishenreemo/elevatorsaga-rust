use crate::Command;
use crate::Error;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn on_keydown(commands: &mut Vec<Command>, keycode: Option<Keycode>) -> Result<(), Error> {
    match keycode {
        Some(Keycode::Escape) => commands.push(Command::Quit),
        Some(Keycode::Up) => commands.push(Command::ElevatorUp),
        Some(Keycode::Down) => commands.push(Command::ElevatorDown),
        _ => (),
    }

    Ok(())
}

pub fn handle_event(commands: &mut Vec<Command>, event: Event) -> Result<(), Error> {
    println!("{event:?}");
    match event {
        Event::Quit { .. } => commands.push(Command::Quit),
        Event::KeyDown { keycode, .. } => on_keydown(commands, keycode)?,
        _ => (),
    }

    Ok(())
}
