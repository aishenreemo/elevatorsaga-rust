mod amend;
mod display;
mod listener;

pub mod config;
pub mod game;

use std::thread::sleep;
use std::time::Duration;

pub type Error = Box<dyn ::std::error::Error>;

pub enum Command {
    Quit,
}

fn main() -> Result<(), Error> {
    let ttf_context = sdl2::ttf::init()?;
    let cfg = config::init_cfg(&ttf_context)?;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo", cfg.window_size.0, cfg.window_size.1)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    let mut game = game::init_game(&cfg);

    display::render(&mut canvas, &game, &cfg)?;

    let mut event_pump = sdl_context.event_pump()?;

    loop {
        let mut commands = vec![];
        for event in event_pump.poll_iter() {
            listener::handle_event(&mut commands, event)?;
        }

        amend::update(&mut game, &canvas, &cfg, &commands)?;
        display::render(&mut canvas, &game, &cfg)?;

        sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
