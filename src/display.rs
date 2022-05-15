use crate::config::Config;
use crate::game::Game;
use crate::Error;

use sdl2::render::WindowCanvas;

fn render_outline(canvas: &mut WindowCanvas, game: &Game, cfg: &Config) -> Result<(), Error> {
    canvas.set_draw_color(cfg.palette.white);
    canvas.draw_line(
        game.layout.top_area.bottom_left(),
        game.layout.top_area.bottom_right(),
    )?;

    Ok(())
}

fn render_top_area(canvas: &mut WindowCanvas, game: &Game, cfg: &Config) -> Result<(), Error> {
    canvas.set_draw_color(cfg.palette.green);
    canvas.draw_rect(game.layout.main_area)?;

    canvas.set_draw_color(cfg.palette.green);
    canvas.fill_rect(game.layout.log_area)?;

    render_floors(canvas, game, cfg)?;
    render_elevators(canvas, game, cfg)?;

    Ok(())
}

fn render_elevators(canvas: &mut WindowCanvas, game: &Game, cfg: &Config) -> Result<(), Error> {
    canvas.set_draw_color(cfg.palette.blue);
    for elevator in game.elevators.iter() {
        canvas.fill_rect(elevator.layout.rect)?;
    }
    Ok(())
}

fn render_floors(canvas: &mut WindowCanvas, game: &Game, cfg: &Config) -> Result<(), Error> {
    let texture_creator = canvas.texture_creator();

    canvas.set_draw_color(cfg.palette.green);
    for floor in game.floors.iter() {
        let texture = texture_creator.create_texture_from_surface(&floor.layout.label)?;

        canvas.copy(&texture, None, floor.layout.label_rect)?;
        canvas.draw_line(floor.layout.rect.top_left(), floor.layout.rect.top_right())?;
    }

    Ok(())
}

pub fn render(canvas: &mut WindowCanvas, game: &Game, cfg: &Config) -> Result<(), Error> {
    canvas.set_draw_color(cfg.palette.black);
    canvas.clear();

    render_outline(canvas, game, cfg)?;
    render_top_area(canvas, game, cfg)?;

    canvas.present();

    Ok(())
}
