use crate::fonts::FontData;
use crate::math::Dimen;
use std::collections::VecDeque;
use sdl2::pixels::Color;

#[derive(new)]
pub struct Window {
    title: String,
    dimen: Dimen,
    background_color: Color,
}

impl Window {
    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn dimen(&self) -> Dimen {
        self.dimen
    }

    pub fn background_color(&self) -> Color {
        self.background_color
    }
}

pub trait App {
    fn window(&self) -> Window;

    fn on_start(&self) {}

    fn font_bus(&self) -> &VecDeque<FontData>;

    fn font_bus_mut(&mut self) -> &mut VecDeque<FontData>;
}
