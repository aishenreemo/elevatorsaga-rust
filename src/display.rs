use crate::config::Config;
use crate::game::Game;
use crate::Error;

use sdl2::render::WindowCanvas;

pub fn render(canvas: &mut WindowCanvas, game: &Game, cfg: &Config) -> Result<(), Error> {
    canvas.set_draw_color(cfg.palette.as_array()[game.color_index]);
    canvas.clear();
    canvas.present();

    Ok(())
}
