use crate::render::colors;
use sdl2::pixels::Color;

pub struct Config<'a> {
    pub title: &'a str,
    pub width: u32,
    pub height: u32,
    pub bg_color: Color,
}

impl<'a> Config<'a> {
    const DEF_TITLE: &'static str = "minswpr";
    const DEF_WIDTH: u32 = 800;
    const DEF_HEIGHT: u32 = 600;
    const DEF_BG_COLOR: Color = colors::BLACK;

    pub fn new() -> Config<'a> {
        Self {
            title: "",
            width: 0,
            height: 0,
            bg_color: colors::BLACK,
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = title;
        self
    }

    pub fn width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn bg_color(mut self, bg_color: Color) -> Self {
        self.bg_color = bg_color;
        self
    }
}

impl<'a> Default for Config<'a> {
    fn default() -> Self {
        Config::new()
            .title(Self::DEF_TITLE)
            .width(Self::DEF_WIDTH)
            .height(Self::DEF_HEIGHT)
            .bg_color(Self::DEF_BG_COLOR)
    }
}
