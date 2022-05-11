use crate::config::Config;
use crate::game::Game;
use crate::Error;

use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

use std::cmp;

pub fn render(canvas: &mut WindowCanvas, game: &Game, cfg: &Config) -> Result<(), Error> {
    // clear background
    canvas.set_draw_color(cfg.palette.black);
    canvas.clear();

    // draw line horizontally in center then a line vertically in the bottom center
    let ws = game.window_size;
    let ws = (ws.0 as i32, ws.1 as i32);
    let half_vertical_size = ws.1 / 2;
    let half_horizontal_size = ws.0 / 2;

    canvas.set_draw_color(cfg.palette.white);
    canvas.draw_line((0, half_vertical_size), (ws.0, half_vertical_size))?;
    canvas.draw_line(
        (half_horizontal_size, half_vertical_size),
        (half_horizontal_size, ws.0),
    )?;

    // top area
    let mr_center_point = (half_horizontal_size, half_vertical_size / 2);
    let mr_width = ws.0 as f32 * 0.9;
    let mr_height = half_vertical_size as f32 * 0.8;
    let main_rect = Rect::from_center(mr_center_point, mr_width as u32, mr_height as u32);

    canvas.set_draw_color(cfg.palette.green);
    canvas.draw_rect(main_rect)?;

    let top_left = main_rect.top_left();
    let bottom_left = main_rect.bottom_left();
    let x_offset = (mr_width * 0.75) as i32;

    canvas.draw_line(
        (top_left.x() + x_offset, top_left.y()),
        (bottom_left.x() + x_offset, bottom_left.y()),
    )?;

    let floor_height = mr_height / game.floors_length as f32;
    let index_text_width = mr_width * 0.04;
    let text_offset = (
        top_left.x() + (mr_width * 0.01) as i32,
        (floor_height * 0.9) as i32,
    );
    let max_text_height = mr_height * 0.2;
    let text_height = cmp::min(max_text_height as u32, floor_height as u32);
    let texture_creator = canvas.texture_creator();

    for i in 0..game.floors_length {
        let y_offset = (game.floors_length - i) as i32 * floor_height as i32;
        let text = format!("{}", i);
        let surface = cfg
            .fonts
            .mangonel
            .render(&text)
            .blended(cfg.palette.orange)?;
        let text_rect = Rect::new(
            text_offset.0,
            (top_left.y() + y_offset) - text_offset.1,
            index_text_width as u32,
            text_height,
        );

        let texture = texture_creator.create_texture_from_surface(&surface)?;
        canvas.copy(&texture, None, text_rect)?;

        if i == 0 {
            continue;
        }

        canvas.draw_line(
            (top_left.x(), top_left.y() + y_offset),
            (top_left.x() + x_offset, top_left.y() + y_offset),
        )?;
    }

    canvas.present();

    Ok(())
}
