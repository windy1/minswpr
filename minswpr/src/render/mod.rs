#[macro_use]
mod macros;

pub mod board;
pub mod control;

use crate::math::{Dimen, Point};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub trait Render {
    fn render(&self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String>;
    fn dimen(&self) -> Dimen;
}

pub trait RenderMut {
    fn render(&mut self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String>;
    fn dimen(&self) -> Dimen;
}

#[derive(new)]
pub struct RenderRect {
    dimen: Dimen,
    color: Color,
}

impl Render for RenderRect {
    fn render(&self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
        canvas.set_draw_color(self.color);
        canvas.fill_rect(Rect::new(
            pos.x,
            pos.y,
            self.dimen.width(),
            self.dimen.height(),
        ))
    }

    fn dimen(&self) -> Dimen {
        self.dimen
    }
}
