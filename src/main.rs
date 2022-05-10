mod config;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use std::time::Duration;
use std::time::SystemTime;

pub type Error = Box<dyn ::std::error::Error>;

fn main() -> Result<(), Error> {
    let cfg = config::init_cfg();
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("rust-sdl2 demo", cfg.window_size.0, cfg.window_size.1)
        .position_centered()
        .build()?;

    let mut canvas = window.into_canvas().build()?;
    let _time_since_init = SystemTime::now();

    canvas.set_draw_color(cfg.palette.black);
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    let palette_arr = cfg.palette.as_array();
    let mut i = 0;
    let time_per_change = 1;

    let mut last_change = SystemTime::now();
    'running: loop {
        let now = SystemTime::now();

        if now.duration_since(last_change)?.as_secs() > time_per_change {
            i = (i + 1) % palette_arr.len();
            last_change = SystemTime::now();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _ => {}
            }
        }

        canvas.set_draw_color(palette_arr[i]);
        canvas.clear();
        canvas.present();

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
