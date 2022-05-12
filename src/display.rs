use crate::config::Config;
use crate::game::Game;
use crate::Error;

use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

fn render_outline(canvas: &mut WindowCanvas, game: &Game, cfg: &Config) -> Result<(), Error> {
    let ws = game.window_size;
    let ws = (ws.0 as i32, ws.1 as i32);

    canvas.set_draw_color(cfg.palette.white);
    canvas.draw_line((0, ws.1 / 2), (ws.0, ws.1 / 2))?;
    canvas.draw_line((ws.0 / 2, ws.1 / 2), (ws.0 / 2, ws.0))?;

    Ok(())
}

fn render_top_area(canvas: &mut WindowCanvas, game: &Game, cfg: &Config) -> Result<(), Error> {
    let ws = game.window_size;
    let ws = (ws.0 as i32, ws.1 as i32);

    let main_rect = Rect::from_center(
        (ws.0 / 2, ws.1 / 4),
        (ws.0 as f32 * 0.9) as u32,
        ((ws.1 as f32 / 2.0) * 0.8) as u32,
    );

    canvas.set_draw_color(cfg.palette.green);
    canvas.draw_rect(main_rect)?;

    let bisector_x_offset = (main_rect.width() as f32 * 0.75) as i32;

    canvas.draw_line(
        (main_rect.x() + bisector_x_offset, main_rect.y()),
        (
            main_rect.x() + bisector_x_offset,
            main_rect.y() + main_rect.height() as i32,
        ),
    )?;

    render_floors(canvas, game, cfg, main_rect)?;
    render_elevators(canvas, game, cfg, main_rect)?;

    Ok(())
}

fn render_elevators(
    _canvas: &mut WindowCanvas,
    _game: &Game,
    _cfg: &Config,
    _mr: Rect,
) -> Result<(), Error> {
    Ok(())
}

fn render_floors(
    canvas: &mut WindowCanvas,
    game: &Game,
    cfg: &Config,
    mr: Rect,
) -> Result<(), Error> {
    let texture_creator = canvas.texture_creator();
    let height = mr.height() as f32 / game.floors_length as f32;
    let len = game.floors_length;

    for i in 0..len {
        let x_offset = (mr.width() as f32 * 0.75) as i32;
        let y_offset = (len - i) as i32 * height as i32;
        let floor_label = cfg
            .fonts
            .mangonel
            .render(&i.to_string())
            .blended(cfg.palette.orange)?;

        let text_rect = Rect::new(
            mr.x() + (mr.width() as f32 * 0.01) as i32,
            (mr.y() + y_offset) - (height * 0.9) as i32,
            (mr.width() as f32 * 0.04) as u32,
            height as u32,
        );

        let texture = texture_creator.create_texture_from_surface(&floor_label)?;
        canvas.copy(&texture, None, text_rect)?;

        if i == 0 {
            continue;
        }

        canvas.draw_line(
            (mr.x(), mr.y() + y_offset),
            (mr.x() + x_offset, mr.y() + y_offset),
        )?;
    }

    Ok(())
}

pub fn render(canvas: &mut WindowCanvas, game: &Game, cfg: &Config) -> Result<(), Error> {
    // clear background
    canvas.set_draw_color(cfg.palette.black);
    canvas.clear();

    render_outline(canvas, game, cfg)?;
    render_top_area(canvas, game, cfg)?;

    canvas.present();

    Ok(())
}
