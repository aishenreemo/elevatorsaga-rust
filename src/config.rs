use crate::Error;

use sdl2::pixels::Color;
use sdl2::ttf::Font;
use sdl2::ttf::Sdl2TtfContext;

pub struct Config<'ttf> {
    pub window_size: (u32, u32),
    pub palette: Palette,
    pub floors_length: usize,
    pub elevators_length: usize,
    pub fonts: Fonts<'ttf>,
}

pub struct Palette {
    pub black: Color,
    pub red: Color,
    pub green: Color,
    pub orange: Color,
    pub blue: Color,
    pub violet: Color,
    pub cyan: Color,
    pub white: Color,
}

pub struct Fonts<'ttf> {
    pub mangonel: Font<'ttf, 'static>,
}

pub fn init_cfg(ttf_context: &'_ Sdl2TtfContext) -> Result<Config<'_>, Error> {
    Ok(Config {
        window_size: (800, 600),
        palette: Palette {
            black: Color::RGB(19, 23, 24),
            red: Color::RGB(223, 91, 97),
            green: Color::RGB(135, 199, 161),
            orange: Color::RGB(222, 143, 120),
            blue: Color::RGB(103, 145, 201),
            violet: Color::RGB(188, 131, 227),
            cyan: Color::RGB(112, 185, 202),
            white: Color::RGB(196, 196, 196),
        },
        floors_length: 6,
        elevators_length: 2,
        fonts: Fonts {
            mangonel: ttf_context.load_font("assets/fonts/Mangonel.ttf", 128)?,
        },
    })
}
