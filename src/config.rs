use sdl2::pixels::Color;

pub struct Config {
    pub window_size: (u32, u32),
    pub palette: Palette,
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

impl Palette {
    pub fn as_array(&self) -> [Color; 8] {
        [
            self.black,
            self.red,
            self.green,
            self.orange,
            self.blue,
            self.violet,
            self.cyan,
            self.white,
        ]
    }
}

pub fn init_cfg() -> Config {
    Config {
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
    }
}
