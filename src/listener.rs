use crate::Command;
use crate::Error;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

fn on_keydown(commands: &mut Vec<Command>, keycode: Option<Keycode>) -> Result<(), Error> {
    // match keycode {
    //    Some(Keycode::Escape) => commands.push(Command::Quit),
    //    _ => (),
    // }

    if let Some(Keycode::Escape) = keycode {
        commands.push(Command::Quit);
    }

    Ok(())
}

pub fn handle_event(commands: &mut Vec<Command>, event: Event) -> Result<(), Error> {
    match event {
        Event::Quit { .. } => commands.push(Command::Quit),
        Event::KeyDown { keycode, .. } => on_keydown(commands, keycode)?,
        _ => (),
    }

    Ok(())
}
