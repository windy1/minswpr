#[macro_use]
mod macros;

pub mod board;
pub mod control;

use crate::math::{Dimen, Point};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub trait Render {
    fn render(&mut self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String>;

    fn dimen(&self) -> Dimen;

    fn margins(&self) -> Margins {
        Default::default()
    }
}

#[derive(new)]
pub struct RenderRect {
    dimen: Dimen,
    color: Color,
    #[new(default)]
    margins: Margins,
}

impl RenderRect {
    pub fn with_margins(dimen: Dimen, color: Color, margins: Margins) -> Self {
        Self {
            dimen,
            color,
            margins,
        }
    }
}

impl Render for RenderRect {
    fn render(&mut self, canvas: &mut WindowCanvas, pos: Point) -> Result<(), String> {
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

    fn margins(&self) -> Margins {
        self.margins
    }
}

#[derive(new, Default, Clone, Copy, Debug)]
pub struct Margins {
    #[new(default)]
    pub top: u32,
    #[new(default)]
    pub right: u32,
    #[new(default)]
    pub bottom: u32,
    #[new(default)]
    pub left: u32,
}

impl Margins {
    pub fn top(&mut self, top: u32) -> &mut Self {
        self.top = top;
        self
    }

    pub fn right(&mut self, right: u32) -> &mut Self {
        self.right = right;
        self
    }

    pub fn bottom(&mut self, bottom: u32) -> &mut Self {
        self.bottom = bottom;
        self
    }

    pub fn left(&mut self, left: u32) -> &mut Self {
        self.left = left;
        self
    }
}
