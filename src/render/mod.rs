pub mod board;
pub mod colors;

use crate::math::{Dimen, Point};
use sdl2::render::WindowCanvas;

pub trait Render {
    fn render(&self, canvas: &mut WindowCanvas, pos: &Point) -> Result<(), String>;
    fn dimen(&self) -> Dimen;
}
